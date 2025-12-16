use clap::Subcommand;
use gc_validator::{github, validator, analyzer};
use color_eyre::Result;

#[derive(Subcommand, Debug)]
pub enum ValidateCmd {
    /// Validate workflow runs
    Run {
        /// Workflow run ID to validate (or "latest")
        #[arg(long, default_value = "latest")]
        run_id: String,

        /// Validate all workflows from last N hours
        #[arg(long)]
        last_hours: Option<u64>,

        /// Create PR with validation results
        #[arg(long, default_value = "true")]
        create_pr: bool,
    },
    /// Analyze repository (errors, performance, security)
    Analyze {
        /// Types of analysis to run
        #[arg(long, value_delimiter = ',', default_value = "errors,performance,security")]
        types: Vec<String>,

        /// Include successful runs
        #[arg(long, default_value = "false")]
        include_success: bool,
    },
}

pub async fn execute(cmd: ValidateCmd) -> Result<()> {
    // Determine token and repo
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN required");
    let repo = std::env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY required"); // Or pass as global arg?
    // For local dev, maybe fallback to git detection?
    // But gc-validator uses GITHUB_REPOSITORY env var logic usually.
    // Let's rely on env vars for now as per original code.

    // Max parallel hardcoded or from config? Default 10.
    let client = github::GitHubClient::new(&token, &repo, 10);

    match cmd {
        ValidateCmd::Run { run_id, last_hours, create_pr } => {
            validator::run_validation(&client, &run_id, last_hours, create_pr, "terminal").await
                .map_err(|e| color_eyre::eyre::eyre!(e))?;
        }
        ValidateCmd::Analyze { types, include_success } => {
            analyzer::run_analysis(&client, &types, include_success, "terminal").await
                .map_err(|e| color_eyre::eyre::eyre!(e))?;
        }
    }

    Ok(())
}
