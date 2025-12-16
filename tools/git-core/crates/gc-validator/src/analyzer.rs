//! Workflow analyzer with parallel execution

use crate::github::{GitHubClient, WorkflowAnalysis, WorkflowRun};
use anyhow::Result;
use futures::future::join_all;
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub total_runs: usize,
    pub successful: usize,
    pub failed: usize,
    pub cancelled: usize,
    pub errors: Vec<ErrorReport>,
    pub performance: PerformanceReport,
    pub security: SecurityReport,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ErrorReport {
    pub workflow_name: String,
    pub run_id: u64,
    pub job_name: String,
    pub step_name: Option<String>,
    pub error_message: String,
    pub frequency: u32,
}

#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub avg_duration_seconds: f64,
    pub max_duration_seconds: i64,
    pub min_duration_seconds: i64,
    pub slowest_workflows: Vec<(String, i64)>,
    pub parallel_efficiency: f64,
}

#[derive(Debug, Clone)]
pub struct SecurityReport {
    pub hardcoded_secrets: Vec<String>,
    pub outdated_actions: Vec<String>,
    pub unsafe_permissions: Vec<String>,
    pub supply_chain_risks: Vec<String>,
}

/// Run parallel analysis on all workflows
pub async fn run_analysis(
    client: &GitHubClient,
    analysis_types: &[String],
    include_success: bool,
    output_format: &str,
) -> Result<AnalysisResult> {
    info!("🔍 Starting parallel workflow analysis...");
    info!("📋 Analysis types: {:?}", analysis_types);

    // Fetch all runs in parallel batches
    let runs = client.get_workflow_runs(50).await?;

    // Filter runs based on criteria
    let runs_to_analyze: Vec<WorkflowRun> = if include_success {
        runs
    } else {
        runs.into_iter()
            .filter(|r| r.conclusion.as_deref() != Some("success"))
            .collect()
    };

    info!("📊 Analyzing {} workflow runs...", runs_to_analyze.len());

    // Parallel analysis
    let analyses = client.analyze_runs_parallel(runs_to_analyze).await?;

    // Build result
    let result = build_analysis_result(&analyses, analysis_types).await;

    // Output based on format
    match output_format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "total_runs": result.total_runs,
                "successful": result.successful,
                "failed": result.failed,
                "errors": result.errors.len(),
                "recommendations": result.recommendations,
            }))?);
        }
        "markdown" => {
            print_markdown_report(&result);
        }
        _ => {
            print_terminal_report(&result);
        }
    }

    Ok(result)
}

/// Health check for all workflows
pub async fn health_check(client: &GitHubClient, quick: bool) -> Result<()> {
    info!("🏥 Running workflow health check...");

    let workflows = client.get_workflows().await?;
    info!("Found {} workflows", workflows.len());

    if quick {
        // Quick check - just status
        for wf in &workflows {
            let status_icon = match wf.state.as_str() {
                "active" => "✅",
                "disabled" => "⏸️",
                _ => "❓",
            };
            println!("{} {} ({})", status_icon, wf.name, wf.path);
        }
    } else {
        // Deep check - analyze recent runs for each workflow
        let runs = client.get_workflow_runs(100).await?;

        // Group runs by workflow
        let mut workflow_runs: HashMap<u64, Vec<&WorkflowRun>> = HashMap::new();
        for run in &runs {
            workflow_runs.entry(run.workflow_id).or_default().push(run);
        }

        println!("\n📊 Workflow Health Report\n");
        println!("{:<40} {:>8} {:>8} {:>8} {:>10}", "Workflow", "Success", "Failed", "Total", "Health");
        println!("{}", "-".repeat(80));

        for wf in &workflows {
            if let Some(wf_runs) = workflow_runs.get(&wf.id) {
                let total = wf_runs.len();
                let success = wf_runs.iter()
                    .filter(|r| r.conclusion.as_deref() == Some("success"))
                    .count();
                let failed = wf_runs.iter()
                    .filter(|r| r.conclusion.as_deref() == Some("failure"))
                    .count();

                let health = if total > 0 {
                    (success as f64 / total as f64) * 100.0
                } else {
                    100.0
                };

                let health_icon = if health >= 90.0 { "🟢" }
                    else if health >= 70.0 { "🟡" }
                    else { "🔴" };

                println!("{:<40} {:>8} {:>8} {:>8} {:>6.1}% {}",
                    &wf.name[..wf.name.len().min(40)],
                    success, failed, total, health, health_icon);
            } else {
                println!("{:<40} {:>8} {:>8} {:>8} {:>10}",
                    &wf.name[..wf.name.len().min(40)],
                    "-", "-", "0", "N/A");
            }
        }
    }

    Ok(())
}

async fn build_analysis_result(
    analyses: &[WorkflowAnalysis],
    analysis_types: &[String],
) -> AnalysisResult {
    let total_runs = analyses.len();
    let successful = analyses.iter()
        .filter(|a| a.run.conclusion.as_deref() == Some("success"))
        .count();
    let failed = analyses.iter()
        .filter(|a| a.run.conclusion.as_deref() == Some("failure"))
        .count();
    let cancelled = analyses.iter()
        .filter(|a| a.run.conclusion.as_deref() == Some("cancelled"))
        .count();

    // Collect errors
    let mut errors = Vec::new();
    let mut error_freq: HashMap<String, u32> = HashMap::new();

    for analysis in analyses {
        for error in &analysis.errors {
            *error_freq.entry(error.clone()).or_insert(0) += 1;

            for job in &analysis.jobs {
                if job.conclusion.as_deref() == Some("failure") {
                    errors.push(ErrorReport {
                        workflow_name: analysis.run.name.clone(),
                        run_id: analysis.run.id,
                        job_name: job.name.clone(),
                        step_name: job.steps.as_ref().and_then(|steps| {
                            steps.iter()
                                .find(|s| s.conclusion.as_deref() == Some("failure"))
                                .map(|s| s.name.clone())
                        }),
                        error_message: error.clone(),
                        frequency: *error_freq.get(error).unwrap_or(&1),
                    });
                }
            }
        }
    }

    // Performance analysis
    let durations: Vec<i64> = analyses.iter()
        .filter_map(|a| a.duration_seconds)
        .collect();

    let performance = PerformanceReport {
        avg_duration_seconds: if !durations.is_empty() {
            durations.iter().sum::<i64>() as f64 / durations.len() as f64
        } else {
            0.0
        },
        max_duration_seconds: durations.iter().max().copied().unwrap_or(0),
        min_duration_seconds: durations.iter().min().copied().unwrap_or(0),
        slowest_workflows: analyses.iter()
            .filter_map(|a| a.duration_seconds.map(|d| (a.run.name.clone(), d)))
            .collect::<Vec<_>>()
            .into_iter()
            .take(5)
            .collect(),
        parallel_efficiency: calculate_parallel_efficiency(analyses),
    };

    // Security analysis (basic)
    let security = SecurityReport {
        hardcoded_secrets: Vec::new(), // Would need file access
        outdated_actions: Vec::new(),  // Would need to check action versions
        unsafe_permissions: Vec::new(),
        supply_chain_risks: Vec::new(),
    };

    // Generate recommendations
    let mut recommendations = Vec::new();

    if failed > 0 {
        recommendations.push(format!(
            "🔴 {} workflow runs failed. Review error logs for root cause.",
            failed
        ));
    }

    if performance.avg_duration_seconds > 300.0 {
        recommendations.push(
            "⏱️ Average workflow duration > 5 minutes. Consider parallelizing jobs.".to_string()
        );
    }

    if performance.parallel_efficiency < 0.7 {
        recommendations.push(
            "📊 Low parallel efficiency detected. Jobs may be waiting unnecessarily.".to_string()
        );
    }

    if cancelled > 0 {
        recommendations.push(format!(
            "⚠️ {} runs were cancelled. Check for timeout issues or manual cancellations.",
            cancelled
        ));
    }

    AnalysisResult {
        total_runs,
        successful,
        failed,
        cancelled,
        errors,
        performance,
        security,
        recommendations,
    }
}

fn calculate_parallel_efficiency(analyses: &[WorkflowAnalysis]) -> f64 {
    // Calculate how well jobs are parallelized
    // Efficiency = (sum of job durations) / (total workflow duration * num_jobs)

    let mut total_efficiency = 0.0;
    let mut count = 0;

    for analysis in analyses {
        if let Some(workflow_duration) = analysis.duration_seconds {
            let job_count = analysis.jobs.len();
            if job_count > 1 && workflow_duration > 0 {
                // Estimate: if all jobs run in parallel, efficiency = 1.0
                // if all sequential, efficiency = 1/num_jobs
                let avg_job_time = workflow_duration as f64 / job_count as f64;
                let efficiency = avg_job_time / workflow_duration as f64 * job_count as f64;
                total_efficiency += efficiency.min(1.0);
                count += 1;
            }
        }
    }

    if count > 0 {
        total_efficiency / count as f64
    } else {
        1.0
    }
}

fn print_terminal_report(result: &AnalysisResult) {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║              📊 WORKFLOW ANALYSIS REPORT                        ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ Total Runs: {:>6}  │  ✅ Success: {:>4}  │  ❌ Failed: {:>4}  ║",
        result.total_runs, result.successful, result.failed);
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║ 📈 Performance                                                  ║");
    println!("║   Avg Duration: {:>6.1}s  │  Max: {:>6}s  │  Min: {:>6}s      ║",
        result.performance.avg_duration_seconds,
        result.performance.max_duration_seconds,
        result.performance.min_duration_seconds);
    println!("║   Parallel Efficiency: {:>5.1}%                                  ║",
        result.performance.parallel_efficiency * 100.0);
    println!("╠════════════════════════════════════════════════════════════════╣");

    if !result.recommendations.is_empty() {
        println!("║ 💡 Recommendations                                              ║");
        for rec in &result.recommendations {
            println!("║   • {}  ", &rec[..rec.len().min(55)]);
        }
    }

    println!("╚════════════════════════════════════════════════════════════════╝\n");
}

fn print_markdown_report(result: &AnalysisResult) {
    println!("# 📊 Workflow Analysis Report\n");
    println!("## Summary\n");
    println!("| Metric | Value |");
    println!("|--------|-------|");
    println!("| Total Runs | {} |", result.total_runs);
    println!("| Successful | {} |", result.successful);
    println!("| Failed | {} |", result.failed);
    println!("| Cancelled | {} |", result.cancelled);
    println!();
    println!("## Performance\n");
    println!("| Metric | Value |");
    println!("|--------|-------|");
    println!("| Average Duration | {:.1}s |", result.performance.avg_duration_seconds);
    println!("| Max Duration | {}s |", result.performance.max_duration_seconds);
    println!("| Parallel Efficiency | {:.1}% |", result.performance.parallel_efficiency * 100.0);
    println!();

    if !result.recommendations.is_empty() {
        println!("## Recommendations\n");
        for rec in &result.recommendations {
            println!("- {}", rec);
        }
    }
}
