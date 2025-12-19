//! Issue Syncer - Bidirectional sync between .md files and GitHub Issues
//!
//! High-performance Rust tool replacing sync-issues.ps1

mod commands;
mod github;
mod mapping;
mod parser;
mod syncer;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::commands::issue::IssueArgs;
use crate::github::GitHubClient;
use crate::syncer::IssueSyncer;

#[derive(Parser)]
#[command(name = "issue-syncer")]
#[command(about = "Bidirectional sync between .md files and GitHub Issues", long_about = None)]
#[command(version)]
struct Cli {
    /// GitHub repository (format: owner/repo)
    #[arg(short, long)]
    repo: Option<String>,

    /// GitHub token
    #[arg(short, long)]
    token: Option<String>,

    /// Issues directory path
    #[arg(long, default_value = ".github/issues")]
    issues_dir: PathBuf,

    /// Enable dry-run mode (no actual changes)
    #[arg(long, default_value = "false")]
    dry_run: bool,

    /// Enable verbose logging
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bidirectional sync: push local files + pull closed issues
    Sync,

    /// Push local .md files to GitHub Issues
    Push,

    /// Pull and delete files for closed issues
    Pull,

    /// Show current mapping statistics
    Status,

    /// Manage issues
    Issue(IssueArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    // Parse repo
    let repo = cli.repo
        .or_else(|| std::env::var("GITHUB_REPOSITORY").ok())
        .context("Repository not specified (use --repo or GITHUB_REPOSITORY)")?;

    let (owner, repo_name) = repo
        .split_once('/')
        .context("Invalid repo format (expected owner/repo)")?;

    // Get GitHub token
    let token = cli.token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .context("GitHub token not provided (use --token or GITHUB_TOKEN)")?;

    // Create GitHub client
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .context("Failed to create GitHub client")?;

    let github = GitHubClient::new(
        octocrab,
        owner.to_string(),
        repo_name.to_string(),
    );

    // Setup paths
    let issues_dir = if cli.issues_dir.is_absolute() {
        cli.issues_dir
    } else {
        std::env::current_dir()?.join(&cli.issues_dir)
    };

    let mapping_file = issues_dir.join(".issue-mapping.json");

    // Create syncer
    let mut syncer = IssueSyncer::new(github, issues_dir.clone(), mapping_file)?
        .with_dry_run(cli.dry_run);

    // Execute command
    match cli.command {
        Commands::Sync => {
            info!("Starting bidirectional sync");
            let report = syncer.sync_all().await?;
            print_report(&report);
        }
        Commands::Push => {
            info!("Pushing local files to GitHub");
            let report = syncer.push().await?;
            print_report(&report);
        }
        Commands::Pull => {
            info!("Pulling closed issues from GitHub");
            let report = syncer.pull().await?;
            print_report(&report);
        }
        Commands::Status => {
            let mapping = syncer.mapping();
            println!("📊 Mapping Statistics:");
            println!("  Total mappings: {}", mapping.len());
            println!("  Files: {:?}", mapping.files());
            println!("  Issues: {:?}", mapping.issues());
        }
        Commands::Issue(args) => {
            commands::issue::handle_issue_command(args, syncer, issues_dir).await?;
        }
    }

    Ok(())
}

pub fn print_report(report: &syncer::SyncReport) {
    println!("\n✅ Sync Complete");
    println!("  Created:  {}", report.created);
    println!("  Updated:  {}", report.updated);
    println!("  Deleted:  {}", report.deleted);
    println!("  Skipped:  {}", report.skipped);
    println!("  Errors:   {}", report.errors);
    println!("  Total:    {}", report.total_operations());
}
