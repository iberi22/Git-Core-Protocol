//! Report generation with parallel data gathering

use crate::github::{GitHubClient, WorkflowRun};
use anyhow::Result;
use chrono::{Utc, Duration};
use futures::future::join_all;
use tracing::info;

/// Generate comprehensive report
pub async fn generate_report(
    client: &GitHubClient,
    report_type: &str,
    hours: u64,
    output_format: &str,
) -> Result<()> {
    info!("📝 Generating {} report for last {} hours...", report_type, hours);

    let runs = client.get_workflow_runs(100).await?;
    let cutoff = Utc::now() - Duration::hours(hours as i64);

    let filtered_runs: Vec<_> = runs.into_iter()
        .filter(|r| {
            chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|dt| dt > cutoff)
                .unwrap_or(false)
        })
        .collect();

    info!("📊 Found {} runs in time range", filtered_runs.len());

    // Parallel analysis
    let analyses = client.analyze_runs_parallel(filtered_runs).await?;

    match report_type {
        "summary" => generate_summary_report(&analyses, output_format),
        "detailed" => generate_detailed_report(&analyses, output_format),
        "diff" => generate_diff_report(&analyses, output_format),
        _ => generate_summary_report(&analyses, output_format),
    }

    Ok(())
}

fn generate_summary_report(analyses: &[crate::github::WorkflowAnalysis], format: &str) {
    let total = analyses.len();
    let success = analyses.iter().filter(|a| a.run.conclusion.as_deref() == Some("success")).count();
    let failed = analyses.iter().filter(|a| a.run.conclusion.as_deref() == Some("failure")).count();
    let cancelled = analyses.iter().filter(|a| a.run.conclusion.as_deref() == Some("cancelled")).count();

    let total_duration: i64 = analyses.iter().filter_map(|a| a.duration_seconds).sum();
    let avg_duration = if total > 0 { total_duration / total as i64 } else { 0 };

    match format {
        "json" => {
            println!("{}", serde_json::json!({
                "summary": {
                    "total_runs": total,
                    "successful": success,
                    "failed": failed,
                    "cancelled": cancelled,
                    "success_rate": if total > 0 { success as f64 / total as f64 * 100.0 } else { 0.0 },
                    "avg_duration_seconds": avg_duration
                }
            }));
        }
        "markdown" => {
            println!("# 📊 Workflow Summary Report\n");
            println!("| Metric | Value |");
            println!("|--------|-------|");
            println!("| Total Runs | {} |", total);
            println!("| Successful | {} ({:.1}%) |", success, if total > 0 { success as f64 / total as f64 * 100.0 } else { 0.0 });
            println!("| Failed | {} |", failed);
            println!("| Cancelled | {} |", cancelled);
            println!("| Avg Duration | {}s |", avg_duration);
        }
        _ => {
            println!("\n📊 Workflow Summary");
            println!("   Total: {} | ✅ {} | ❌ {} | ⏹️ {}", total, success, failed, cancelled);
            println!("   Success Rate: {:.1}%", if total > 0 { success as f64 / total as f64 * 100.0 } else { 0.0 });
            println!("   Avg Duration: {}s\n", avg_duration);
        }
    }
}

fn generate_detailed_report(analyses: &[crate::github::WorkflowAnalysis], format: &str) {
    match format {
        "markdown" => {
            println!("# 📋 Detailed Workflow Report\n");
            println!("## Runs\n");
            println!("| Workflow | Run ID | Status | Duration | Errors |");
            println!("|----------|--------|--------|----------|--------|");

            for analysis in analyses {
                println!("| {} | {} | {} | {}s | {} |",
                    analysis.run.name,
                    analysis.run.id,
                    analysis.run.conclusion.as_deref().unwrap_or("pending"),
                    analysis.duration_seconds.unwrap_or(0),
                    analysis.errors.len()
                );
            }

            // Error summary
            let all_errors: Vec<_> = analyses.iter()
                .flat_map(|a| a.errors.iter().map(|e| (a.run.name.clone(), e.clone())))
                .collect();

            if !all_errors.is_empty() {
                println!("\n## Errors\n");
                for (workflow, error) in &all_errors {
                    println!("- **{}**: {}", workflow, error);
                }
            }
        }
        "json" => {
            let report: Vec<_> = analyses.iter().map(|a| {
                serde_json::json!({
                    "workflow": a.run.name,
                    "run_id": a.run.id,
                    "status": a.run.status,
                    "conclusion": a.run.conclusion,
                    "duration_seconds": a.duration_seconds,
                    "job_count": a.jobs.len(),
                    "errors": a.errors,
                    "warnings": a.warnings
                })
            }).collect();

            println!("{}", serde_json::to_string_pretty(&report).unwrap());
        }
        _ => {
            println!("\n📋 Detailed Workflow Report\n");
            for analysis in analyses {
                let status_icon = match analysis.run.conclusion.as_deref() {
                    Some("success") => "✅",
                    Some("failure") => "❌",
                    Some("cancelled") => "⏹️",
                    _ => "🔄",
                };

                println!("{} {} (Run #{}) - {}s",
                    status_icon,
                    analysis.run.name,
                    analysis.run.id,
                    analysis.duration_seconds.unwrap_or(0)
                );

                if !analysis.errors.is_empty() {
                    for error in &analysis.errors {
                        println!("   ⚠️  {}", error);
                    }
                }
            }
        }
    }
}

fn generate_diff_report(analyses: &[crate::github::WorkflowAnalysis], format: &str) {
    // Compare recent runs with older runs to show trends
    let half = analyses.len() / 2;
    let recent = &analyses[..half.min(analyses.len())];
    let older = &analyses[half.min(analyses.len())..];

    let recent_success = recent.iter().filter(|a| a.run.conclusion.as_deref() == Some("success")).count();
    let older_success = older.iter().filter(|a| a.run.conclusion.as_deref() == Some("success")).count();

    let recent_avg_duration: i64 = recent.iter()
        .filter_map(|a| a.duration_seconds)
        .sum::<i64>() / recent.len().max(1) as i64;

    let older_avg_duration: i64 = older.iter()
        .filter_map(|a| a.duration_seconds)
        .sum::<i64>() / older.len().max(1) as i64;

    let success_trend = if recent.len() > 0 && older.len() > 0 {
        (recent_success as f64 / recent.len() as f64) - (older_success as f64 / older.len() as f64)
    } else {
        0.0
    };

    let duration_trend = recent_avg_duration - older_avg_duration;

    match format {
        "markdown" => {
            println!("# 📈 Workflow Trend Report\n");
            println!("| Metric | Recent | Previous | Trend |");
            println!("|--------|--------|----------|-------|");
            println!("| Success Rate | {:.1}% | {:.1}% | {} |",
                if recent.len() > 0 { recent_success as f64 / recent.len() as f64 * 100.0 } else { 0.0 },
                if older.len() > 0 { older_success as f64 / older.len() as f64 * 100.0 } else { 0.0 },
                if success_trend > 0.0 { "📈" } else if success_trend < 0.0 { "📉" } else { "➡️" }
            );
            println!("| Avg Duration | {}s | {}s | {} |",
                recent_avg_duration,
                older_avg_duration,
                if duration_trend < 0 { "📈 Faster" } else if duration_trend > 0 { "📉 Slower" } else { "➡️ Same" }
            );
        }
        _ => {
            println!("\n📈 Workflow Trends\n");
            println!("Success Rate: {:.1}% → {:.1}% {}",
                if older.len() > 0 { older_success as f64 / older.len() as f64 * 100.0 } else { 0.0 },
                if recent.len() > 0 { recent_success as f64 / recent.len() as f64 * 100.0 } else { 0.0 },
                if success_trend > 0.0 { "📈" } else if success_trend < 0.0 { "📉" } else { "➡️" }
            );
            println!("Avg Duration: {}s → {}s {}",
                older_avg_duration,
                recent_avg_duration,
                if duration_trend < 0 { "📈" } else if duration_trend > 0 { "📉" } else { "➡️" }
            );
        }
    }
}
