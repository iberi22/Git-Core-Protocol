use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use std::fs;
use std::path::PathBuf;

use crate::syncer::IssueSyncer;

#[derive(Args)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: IssueCommand,
}

#[derive(Subcommand)]
pub enum IssueCommand {
    /// Create a new issue
    Create(CreateArgs),
    /// Sync issues with GitHub
    Sync,
    /// List issues
    List(ListArgs),
    /// Clean up closed issues
    Clean(CleanArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    /// Issue title
    #[arg(short, long)]
    pub title: String,
    /// Issue labels
    #[arg(short, long, value_delimiter = ',')]
    pub labels: Vec<String>,
    /// Assign to Jules
    #[arg(long)]
    pub jules: bool,
    /// Create local-only issue
    #[arg(long)]
    pub local_only: bool,
}

#[derive(Args)]
pub struct ListArgs {
    /// List open issues
    #[arg(long)]
    pub open: bool,
    /// List closed issues
    #[arg(long)]
    pub closed: bool,
}

#[derive(Args)]
pub struct CleanArgs {
    /// Dry run (see what would be deleted)
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn handle_issue_command(
    args: IssueArgs,
    mut syncer: IssueSyncer,
    issues_dir: PathBuf,
) -> Result<()> {
    match args.command {
        IssueCommand::Create(create_args) => {
            create_issue(create_args, &mut syncer, issues_dir).await?;
        }
        IssueCommand::Sync => {
            println!("Syncing all issues...");
            let report = syncer.sync_all().await?;
            crate::print_report(&report);
        }
        IssueCommand::List(list_args) => {
            list_issues(list_args, &syncer).await?;
        }
        IssueCommand::Clean(clean_args) => {
            println!("Cleaning up closed issues...");
            let mut syncer = syncer.with_dry_run(clean_args.dry_run);
            let report = syncer.pull().await?;
            crate::print_report(&report);
        }
    }
    Ok(())
}

async fn create_issue(
    args: CreateArgs,
    syncer: &mut IssueSyncer,
    issues_dir: PathBuf,
) -> Result<()> {
    let title_slug = args
        .title
        .to_lowercase()
        .replace([' ', '\t'], "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
    let filename = format!("FEAT_{}.md", title_slug);
    let filepath = issues_dir.join(&filename);

    let mut assignees = Vec::new();
    if args.jules {
        assignees.push("jules".to_string());
    }

    let mut content = format!("---\ntitle: \"{}\"\n", args.title);

    if !args.labels.is_empty() {
        content.push_str("labels:\n");
        for label in &args.labels {
            content.push_str(&format!("  - {}\n", label));
        }
    }

    if !assignees.is_empty() {
        content.push_str("assignees:\n");
        for assignee in &assignees {
            content.push_str(&format!("  - {}\n", assignee));
        }
    }

    content.push_str("---\n\n");
    content.push_str(&format!("# {}\n\n", args.title));
    content.push_str("## Description\n\n");

    fs::write(&filepath, content).context("Failed to write issue file")?;

    println!("Created local issue file: {}", filepath.display());

    if !args.local_only {
        println!("Syncing new issue to GitHub...");
        let report = syncer.push().await?;
        crate::print_report(&report);
    }

    Ok(())
}

async fn list_issues(args: ListArgs, syncer: &IssueSyncer) -> Result<()> {
    let state = if args.closed { "closed" } else { "open" };
    println!("Fetching {} issues...", state);

    let issues = if args.closed {
        syncer.github().fetch_closed_issues().await?
    } else {
        syncer.github().fetch_open_issues().await?
    };

    if issues.is_empty() {
        println!("No {} issues found.", state);
        return Ok(());
    }

    for issue in issues {
        println!(
            "#{} - {} [{}]",
            issue.number,
            issue.title,
            format!("{:?}", issue.state).to_lowercase()
        );
    }

    Ok(())
}
