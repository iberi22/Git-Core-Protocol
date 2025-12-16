use clap::Args;
use gc_core::ports::{SystemPort, Result, CoreError};
use serde::Deserialize;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use console::style;

#[derive(Args, Debug)]
pub struct CiDetectArgs {
    /// Repository to analyze (defaults to GITHUB_REPOSITORY env var)
    #[arg(long, env = "GITHUB_REPOSITORY")]
    pub repository: Option<String>,
}

#[derive(Deserialize, Debug)]
struct RepoInfo {
    #[serde(default, rename = "isPrivate")]
    is_private: bool,
    #[serde(default)]
    visibility: String, // PUBLIC, PRIVATE, INTERNAL
}

pub async fn execute(args: CiDetectArgs, system: &impl SystemPort) -> Result<()> {
    println!("{}", style("🔍 Repository Configuration Detection").cyan());

    let repo_name = args.repository.ok_or_else(|| CoreError::System("Repository not provided and GITHUB_REPOSITORY not set".into()))?;

    // 1. Detect Repository Visibility
    let (is_public, visibility) = match get_repo_visibility(&repo_name, system).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} Error detecting visibility: {}", style("❌").red(), e);
            eprintln!("{} Defaulting to PRIVATE (conservative mode)", style("⚠️").yellow());
            (false, "PRIVATE".to_string())
        }
    };

    println!("📊 Repository: {}", style(&repo_name).cyan());
    println!("🔒 Visibility: {}", style(&visibility).cyan());

    // 2. Detect if Main Repo
    // Matches "Git-Core-Protocol", "git-core", "ai-git-core"
    let is_main_repo = repo_name.contains("Git-Core-Protocol") ||
                       repo_name.contains("git-core") ||
                       repo_name.contains("ai-git-core");

    println!("🏠 Is Main Repo: {}", style(is_main_repo).cyan());

    // 3. Determine Schedule Mode
    let (schedule_mode, enable_schedules) = if is_public {
        println!("{}", style("✅ PUBLIC repo: Aggressive scheduling enabled (unlimited minutes)").green());
        ("aggressive", true)
    } else if is_main_repo {
        println!("{}", style("⚠️  MAIN PRIVATE repo: Moderate scheduling (2,000 min/month limit)").yellow());
        ("moderate", true)
    } else {
        println!("{}", style("🔒 PRIVATE repo: Conservative mode (event-based triggers only)").red());
        ("conservative", false)
    };

    // 4. Output to GITHUB_OUTPUT
    if let Ok(github_output_path) = env::var("GITHUB_OUTPUT") {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(github_output_path)
            .map_err(CoreError::Io)?;

        writeln!(file, "is_public={}", is_public).map_err(CoreError::Io)?;
        writeln!(file, "is_main_repo={}", is_main_repo).map_err(CoreError::Io)?;
        writeln!(file, "enable_schedules={}", enable_schedules).map_err(CoreError::Io)?;
        writeln!(file, "schedule_mode={}", schedule_mode).map_err(CoreError::Io)?;
    }

    // 5. Summary Output
    println!("\n📋 Configuration Summary:");
    println!("   IS_PUBLIC={}", is_public);
    println!("   IS_MAIN_REPO={}", is_main_repo);
    println!("   ENABLE_SCHEDULES={}", enable_schedules);
    println!("   SCHEDULE_MODE={}", schedule_mode);

    println!("\n{} Schedule Mode Details:", style("💡").cyan());
    match schedule_mode {
        "aggressive" => {
            println!("   {} All scheduled workflows enabled", style("•").green());
            println!("   {} High-frequency schedules (every 30 min)", style("•").green());
            println!("   {} Multi-repo monitoring enabled", style("•").green());
        },
        "moderate" => {
            println!("   {} Essential schedules only", style("•").yellow());
            println!("   {} Reduced frequency (every 6 hours)", style("•").yellow());
            println!("   {} Single-repo monitoring", style("•").yellow());
        },
        "conservative" => {
            println!("   {} No scheduled workflows", style("•").red());
            println!("   {} Event-based triggers only", style("•").red());
        },
        _ => {}
    }

    Ok(())
}

async fn get_repo_visibility(repo: &str, system: &impl SystemPort) -> Result<(bool, String)> {
    let args = ["repo", "view", repo, "--json", "isPrivate,visibility"];
    let args_vec = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    // We expect valid JSON output
    let output = system.run_command_output("gh", &args_vec).await?;

    let info: RepoInfo = serde_json::from_str(&output)
        .map_err(|e| CoreError::System(format!("Failed to parse gh output: {}", e)))?;

    // GitHub API 'isPrivate' is reliable. 'visibility' might be PUBLIC, PRIVATE, INTERNAL.
    // Logic from script: if isPublic -> True, else False.
    let is_public = !info.is_private;

    Ok((is_public, info.visibility))
}
