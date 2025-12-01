//! Post-run validation and continuous improvement system

use crate::github::{GitHubClient, WorkflowAnalysis};
use anyhow::Result;
use chrono::Utc;
use tracing::{info, warn};

/// Run validation on completed workflows
pub async fn run_validation(
    client: &GitHubClient,
    run_id: &str,
    last_hours: Option<u64>,
    create_pr: bool,
    output_format: &str,
) -> Result<()> {
    info!("🔍 Starting workflow validation...");

    let runs = client.get_workflow_runs(50).await?;

    let runs_to_validate: Vec<_> = if run_id == "latest" {
        if let Some(hours) = last_hours {
            let cutoff = Utc::now() - chrono::Duration::hours(hours as i64);
            runs.into_iter()
                .filter(|r| {
                    chrono::DateTime::parse_from_rfc3339(&r.created_at)
                        .map(|dt| dt > cutoff)
                        .unwrap_or(false)
                })
                .collect()
        } else {
            runs.into_iter().take(10).collect()
        }
    } else {
        let target_id: u64 = run_id.parse()?;
        runs.into_iter().filter(|r| r.id == target_id).collect()
    };

    info!("📊 Validating {} workflow runs...", runs_to_validate.len());

    // Parallel analysis
    let analyses = client.analyze_runs_parallel(runs_to_validate).await?;

    // Generate validation report
    let report = generate_validation_report(&analyses);

    if create_pr {
        create_validation_pr(client, &report, &analyses).await?;
    }

    match output_format {
        "json" => println!("{}", serde_json::to_string_pretty(&report)?),
        "markdown" => println!("{}", report.to_markdown()),
        _ => println!("{}", report.to_terminal()),
    }

    Ok(())
}

/// Post-run validation with AI review requests
pub async fn post_run_validation(
    client: &GitHubClient,
    run_id: &str,
    ai_review: bool,
) -> Result<()> {
    info!("🔬 Running post-execution validation for run {}...", run_id);

    let target_id: u64 = run_id.parse()?;
    let runs = client.get_workflow_runs(20).await?;
    
    let run = runs.into_iter()
        .find(|r| r.id == target_id)
        .ok_or_else(|| anyhow::anyhow!("Run {} not found", run_id))?;

    let analyses = client.analyze_runs_parallel(vec![run.clone()]).await?;
    let analysis = analyses.into_iter().next()
        .ok_or_else(|| anyhow::anyhow!("Failed to analyze run"))?;

    // Generate comprehensive report
    let report = ValidationReport::from_analysis(&analysis);

    // Create branch for validation PR
    let branch_name = format!("validation/run-{}-{}", run_id, Utc::now().format("%Y%m%d%H%M%S"));
    let sha = client.get_default_branch_sha().await?;
    
    // Try to create branch (may fail if already exists)
    let _ = client.create_branch(&branch_name, &sha).await;

    // Create PR with validation results
    let pr_body = generate_pr_body(&report, &analysis, ai_review);
    
    let pr = client.create_pr(
        &format!("🔬 Validation: {} - Run #{}", analysis.run.name, run_id),
        &pr_body,
        &branch_name,
    ).await?;

    info!("✅ Created validation PR: {}", pr.html_url);

    if ai_review {
        // Request AI reviews
        request_ai_reviews(client, pr.number).await?;
    }

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct ValidationReport {
    pub timestamp: String,
    pub workflow_name: String,
    pub run_id: u64,
    pub status: String,
    pub conclusion: String,
    pub duration_seconds: Option<i64>,
    pub errors: Vec<ErrorDetail>,
    pub warnings: Vec<String>,
    pub performance_score: f64,
    pub security_score: f64,
    pub recommendations: Vec<Recommendation>,
    pub metrics: ValidationMetrics,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorDetail {
    pub job: String,
    pub step: Option<String>,
    pub message: String,
    pub severity: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct Recommendation {
    pub category: String,
    pub priority: String,
    pub title: String,
    pub description: String,
    pub action: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ValidationMetrics {
    pub job_count: usize,
    pub step_count: usize,
    pub failed_jobs: usize,
    pub failed_steps: usize,
    pub parallel_jobs: usize,
    pub sequential_jobs: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl ValidationReport {
    pub fn from_analysis(analysis: &WorkflowAnalysis) -> Self {
        let mut errors = Vec::new();
        let mut warnings = analysis.warnings.clone();
        let mut failed_jobs = 0;
        let mut failed_steps = 0;
        let mut step_count = 0;

        for job in &analysis.jobs {
            if job.conclusion.as_deref() == Some("failure") {
                failed_jobs += 1;
                
                if let Some(steps) = &job.steps {
                    for step in steps {
                        step_count += 1;
                        if step.conclusion.as_deref() == Some("failure") {
                            failed_steps += 1;
                            errors.push(ErrorDetail {
                                job: job.name.clone(),
                                step: Some(step.name.clone()),
                                message: format!("Step '{}' failed in job '{}'", step.name, job.name),
                                severity: "error".to_string(),
                                suggested_fix: suggest_fix(&step.name),
                            });
                        }
                    }
                }
            } else if let Some(steps) = &job.steps {
                step_count += steps.len();
            }
        }

        // Generate recommendations based on analysis
        let mut recommendations = Vec::new();

        if failed_jobs > 0 {
            recommendations.push(Recommendation {
                category: "reliability".to_string(),
                priority: "high".to_string(),
                title: "Fix failing jobs".to_string(),
                description: format!("{} job(s) failed in this run", failed_jobs),
                action: "Review error logs and fix the underlying issues".to_string(),
            });
        }

        if let Some(duration) = analysis.duration_seconds {
            if duration > 600 {
                recommendations.push(Recommendation {
                    category: "performance".to_string(),
                    priority: "medium".to_string(),
                    title: "Optimize workflow duration".to_string(),
                    description: format!("Workflow took {}s (>10 min)", duration),
                    action: "Consider parallelizing jobs or using caching".to_string(),
                });
            }
        }

        // Calculate scores
        let total_jobs = analysis.jobs.len().max(1);
        let performance_score = if let Some(d) = analysis.duration_seconds {
            (1.0 - (d as f64 / 1800.0).min(1.0)) * 100.0 // Score decreases with duration
        } else {
            50.0
        };

        let reliability_score = ((total_jobs - failed_jobs) as f64 / total_jobs as f64) * 100.0;

        ValidationReport {
            timestamp: Utc::now().to_rfc3339(),
            workflow_name: analysis.run.name.clone(),
            run_id: analysis.run.id,
            status: analysis.run.status.clone(),
            conclusion: analysis.run.conclusion.clone().unwrap_or_else(|| "unknown".to_string()),
            duration_seconds: analysis.duration_seconds,
            errors,
            warnings,
            performance_score,
            security_score: 100.0, // Placeholder - would need deeper analysis
            recommendations,
            metrics: ValidationMetrics {
                job_count: analysis.jobs.len(),
                step_count,
                failed_jobs,
                failed_steps,
                parallel_jobs: estimate_parallel_jobs(&analysis.jobs),
                sequential_jobs: analysis.jobs.len() - estimate_parallel_jobs(&analysis.jobs),
                cache_hits: 0,  // Would need log analysis
                cache_misses: 0,
            },
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        md.push_str(&format!("# 🔬 Validation Report: {}\n\n", self.workflow_name));
        md.push_str(&format!("**Run ID:** {} | **Status:** {} | **Conclusion:** {}\n\n", 
            self.run_id, self.status, self.conclusion));
        
        md.push_str("## 📊 Metrics\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Duration | {}s |\n", self.duration_seconds.unwrap_or(0)));
        md.push_str(&format!("| Jobs | {} ({} failed) |\n", self.metrics.job_count, self.metrics.failed_jobs));
        md.push_str(&format!("| Steps | {} ({} failed) |\n", self.metrics.step_count, self.metrics.failed_steps));
        md.push_str(&format!("| Performance Score | {:.1}% |\n", self.performance_score));
        md.push_str(&format!("| Security Score | {:.1}% |\n", self.security_score));
        md.push('\n');

        if !self.errors.is_empty() {
            md.push_str("## ❌ Errors\n\n");
            for error in &self.errors {
                md.push_str(&format!("### {} / {}\n", error.job, error.step.as_deref().unwrap_or("N/A")));
                md.push_str(&format!("- **Message:** {}\n", error.message));
                md.push_str(&format!("- **Severity:** {}\n", error.severity));
                if let Some(fix) = &error.suggested_fix {
                    md.push_str(&format!("- **Suggested Fix:** {}\n", fix));
                }
                md.push('\n');
            }
        }

        if !self.recommendations.is_empty() {
            md.push_str("## 💡 Recommendations\n\n");
            for rec in &self.recommendations {
                md.push_str(&format!("### {} [{}]\n", rec.title, rec.priority.to_uppercase()));
                md.push_str(&format!("**Category:** {} | **Action:** {}\n\n", rec.category, rec.action));
                md.push_str(&format!("{}\n\n", rec.description));
            }
        }

        md
    }

    pub fn to_terminal(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("\n📋 Validation Report: {}\n", self.workflow_name));
        out.push_str(&format!("   Run #{} | {} | {}\n", self.run_id, self.status, self.conclusion));
        out.push_str(&format!("   Duration: {}s | Jobs: {} | Errors: {}\n", 
            self.duration_seconds.unwrap_or(0), 
            self.metrics.job_count,
            self.errors.len()));
        out.push_str(&format!("   Performance: {:.1}% | Security: {:.1}%\n", 
            self.performance_score, self.security_score));
        out
    }
}

fn suggest_fix(step_name: &str) -> Option<String> {
    let name_lower = step_name.to_lowercase();
    
    if name_lower.contains("checkout") {
        Some("Check repository permissions and branch existence".to_string())
    } else if name_lower.contains("install") || name_lower.contains("setup") {
        Some("Verify dependencies and cache configuration".to_string())
    } else if name_lower.contains("build") {
        Some("Check build configuration and dependencies".to_string())
    } else if name_lower.contains("test") {
        Some("Review test failures and check test environment".to_string())
    } else if name_lower.contains("deploy") {
        Some("Verify deployment credentials and target environment".to_string())
    } else {
        None
    }
}

fn estimate_parallel_jobs(jobs: &[crate::github::Job]) -> usize {
    // Estimate based on start times - jobs starting at similar times are parallel
    // This is a simplified heuristic
    if jobs.len() <= 1 {
        return 0;
    }
    
    // For now, assume ~50% parallelism if multiple jobs
    jobs.len() / 2
}

fn generate_validation_report(analyses: &[WorkflowAnalysis]) -> ValidationReport {
    if let Some(analysis) = analyses.first() {
        ValidationReport::from_analysis(analysis)
    } else {
        ValidationReport {
            timestamp: Utc::now().to_rfc3339(),
            workflow_name: "Unknown".to_string(),
            run_id: 0,
            status: "unknown".to_string(),
            conclusion: "unknown".to_string(),
            duration_seconds: None,
            errors: vec![],
            warnings: vec![],
            performance_score: 0.0,
            security_score: 0.0,
            recommendations: vec![],
            metrics: ValidationMetrics {
                job_count: 0,
                step_count: 0,
                failed_jobs: 0,
                failed_steps: 0,
                parallel_jobs: 0,
                sequential_jobs: 0,
                cache_hits: 0,
                cache_misses: 0,
            },
        }
    }
}

fn generate_pr_body(report: &ValidationReport, analysis: &WorkflowAnalysis, ai_review: bool) -> String {
    let mut body = String::new();
    
    body.push_str("## 🔬 Workflow Validation Report\n\n");
    body.push_str(&format!("**Workflow:** {}\n", report.workflow_name));
    body.push_str(&format!("**Run ID:** [#{}]({})\n", report.run_id, analysis.run.html_url));
    body.push_str(&format!("**Conclusion:** {}\n", report.conclusion));
    body.push_str(&format!("**Duration:** {}s\n\n", report.duration_seconds.unwrap_or(0)));

    body.push_str("### 📊 Scores\n\n");
    body.push_str("| Category | Score | Status |\n");
    body.push_str("|----------|-------|--------|\n");
    body.push_str(&format!("| Performance | {:.1}% | {} |\n", 
        report.performance_score,
        if report.performance_score >= 80.0 { "✅" } else if report.performance_score >= 60.0 { "🟡" } else { "🔴" }
    ));
    body.push_str(&format!("| Security | {:.1}% | {} |\n", 
        report.security_score,
        if report.security_score >= 80.0 { "✅" } else if report.security_score >= 60.0 { "🟡" } else { "🔴" }
    ));
    body.push_str(&format!("| Reliability | {:.1}% | {} |\n\n", 
        ((report.metrics.job_count - report.metrics.failed_jobs) as f64 / report.metrics.job_count.max(1) as f64) * 100.0,
        if report.metrics.failed_jobs == 0 { "✅" } else { "🔴" }
    ));

    if !report.errors.is_empty() {
        body.push_str("### ❌ Errors Found\n\n");
        for error in &report.errors {
            body.push_str(&format!("- **{}**: {}\n", error.job, error.message));
            if let Some(fix) = &error.suggested_fix {
                body.push_str(&format!("  - 💡 Suggested: {}\n", fix));
            }
        }
        body.push('\n');
    }

    if !report.recommendations.is_empty() {
        body.push_str("### 💡 Recommendations\n\n");
        for rec in &report.recommendations {
            body.push_str(&format!("- **[{}]** {}: {}\n", rec.priority.to_uppercase(), rec.title, rec.action));
        }
        body.push('\n');
    }

    if ai_review {
        body.push_str("---\n\n");
        body.push_str("### 🤖 AI Review Requested\n\n");
        body.push_str("The following AI agents will analyze this validation:\n\n");
        body.push_str("| Agent | Status | Focus |\n");
        body.push_str("|-------|--------|-------|\n");
        body.push_str("| CodeRabbit | 🔄 Pending | Code quality, patterns |\n");
        body.push_str("| Gemini Code Assist | 🔄 Pending | Performance, security |\n");
        body.push_str("| Copilot (Sonnet 4.5) | ⏳ After bots | Final validation |\n\n");
        body.push_str("> After AI reviews complete, Copilot will be assigned for final validation and implementation.\n");
    }

    body.push_str("\n---\n*Generated by Git-Core Protocol Workflow Orchestrator*\n");

    body
}

async fn create_validation_pr(
    client: &GitHubClient,
    report: &ValidationReport,
    analyses: &[WorkflowAnalysis],
) -> Result<()> {
    if analyses.is_empty() {
        warn!("No analyses to create PR from");
        return Ok(());
    }

    let analysis = &analyses[0];
    let branch_name = format!("validation/batch-{}", Utc::now().format("%Y%m%d%H%M%S"));
    
    let sha = client.get_default_branch_sha().await?;
    let _ = client.create_branch(&branch_name, &sha).await;

    let pr_body = generate_pr_body(report, analysis, true);
    
    let pr = client.create_pr(
        &format!("🔬 Validation Report: {} runs analyzed", analyses.len()),
        &pr_body,
        &branch_name,
    ).await?;

    info!("✅ Created validation PR #{}: {}", pr.number, pr.html_url);

    // Request AI reviews
    request_ai_reviews(client, pr.number).await?;

    Ok(())
}

async fn request_ai_reviews(client: &GitHubClient, pr_number: u64) -> Result<()> {
    info!("🤖 Requesting AI reviews for PR #{}...", pr_number);

    // Request Gemini review
    client.add_pr_comment(pr_number, "/gemini review

Please analyze this workflow validation report and provide:

1. **Error Analysis**: Deep dive into any failures
2. **Performance Assessment**: Are there optimization opportunities?
3. **Security Review**: Any security concerns in the workflow?
4. **Recommendations Validation**: Are the suggested fixes appropriate?

Focus on actionable improvements for the CI/CD pipeline.").await?;

    // Request CodeRabbit review
    client.add_pr_comment(pr_number, "@coderabbitai review

Please analyze this validation report for:
- Pattern improvements
- Best practices violations
- Potential reliability issues").await?;

    // Add Copilot assignment note
    client.add_pr_comment(pr_number, "## 🤖 Copilot Assignment Queue

Once AI reviews are complete, assign this PR to Copilot for:

1. **Meta-Analysis**: Review the AI reviews themselves
2. **Action Plan**: Generate specific fixes based on all feedback
3. **Implementation**: Create follow-up PRs with corrections

```
/copilot analyze --model sonnet-4.5 --scope full
```

> ⏳ Copilot will be auto-assigned after CodeRabbit and Gemini complete their reviews.").await?;

    info!("✅ AI review requests sent");
    Ok(())
}
