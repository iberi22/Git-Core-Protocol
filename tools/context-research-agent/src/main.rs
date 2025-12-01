use clap::Parser;
use anyhow::Result;
use dotenv::dotenv;
use std::path::PathBuf;

mod context;
mod search;
mod intelligence;
mod report;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the output report file
    #[arg(short, long, default_value = "docs/agent-docs/RESEARCH_STACK_CONTEXT.md")]
    output: PathBuf,

    /// Path to the workspace root
    #[arg(short, long, default_value = ".")]
    workspace: PathBuf,

    /// Check quarantine status for dependencies
    #[arg(long, default_value = "true")]
    check_quarantine: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    println!("🔍 Starting Contextual Research Agent...");
    println!("📂 Workspace: {:?}", args.workspace);

    // 1. Analyze Context (Dependencies & Versions)
    println!("📦 Analyzing dependencies...");
    let dependencies = context::analyze_workspace(&args.workspace).await?;
    println!("✅ Found {} dependencies.", dependencies.len());

    // 2. Search GitHub for Issues & Patterns (Parallel)
    println!("🌐 Searching GitHub for context (Issues, Discussions, Releases)...");
    let search_results = search::gather_context(&dependencies).await?;
    println!("✅ Gathered context for {} items.", search_results.len());

    // 3. Analyze with Intelligence (Gemini or GitHub Models)
    println!("🧠 Analyzing anomalies and patterns...");
    let insights = intelligence::analyze_findings(search_results).await?;
    println!("✅ Generated {} insights.", insights.len());

    // 4. Get AI Provider Info (for dynamic report)
    let ai_provider = intelligence::get_provider_info();
    println!("📊 AI Provider: {} ({})", ai_provider.name, ai_provider.model);

    // 5. Check Quarantine Status
    println!("🚧 Checking quarantine status...");
    let quarantine_deps: Vec<report::QuarantineStatus> = dependencies.iter()
        .map(|dep| {
            // For now, we assume all dependencies are stable (release date unknown)
            // In production, this would fetch from crates.io/npm/pypi APIs
            report::check_quarantine_status(&dep.name, &dep.version, None)
        })
        .collect();

    let quarantined_count = quarantine_deps.iter().filter(|q| q.is_quarantined).count();
    println!("✅ Quarantine check complete. {} in quarantine.", quarantined_count);

    // 6. Generate Report
    println!("📝 Generating Living Context Report...");
    report::generate_report(&args.output, &dependencies, &insights, &ai_provider, &quarantine_deps).await?;
    println!("✅ Report saved to {:?}", args.output);

    Ok(())
}
