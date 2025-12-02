//! Git-Core Protocol CLI
//!
//! A modern installer and manager for AI-assisted development workflows.

mod commands;
mod config;
mod installer;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(
    name = "git-core",
    about = "🧠 Git-Core Protocol - AI-assisted development workflow",
    version,
    author,
    after_help = "Examples:\n  git-core install        Install protocol in current directory\n  git-core upgrade        Upgrade existing installation\n  git-core migrate        Migrate from .ai/ to .✨/\n  git-core check          Verify protocol integrity"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Run in non-interactive mode
    #[arg(short = 'y', long, global = true)]
    yes: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Git-Core Protocol in a directory
    Install {
        /// Target directory path (default: current directory)
        #[arg(short, long)]
        path: Option<String>,

        /// Force installation, overwriting existing files
        #[arg(short, long)]
        force: bool,

        /// Organize existing files before installing
        #[arg(short, long)]
        organize: bool,

        /// Specify a version to install (default: latest)
        #[arg(long)]
        version: Option<String>,
    },

    /// Upgrade existing Git-Core Protocol installation
    Upgrade {
        /// Target directory path (default: current directory)
        #[arg(short, long)]
        path: Option<String>,

        /// Force upgrade, overwriting ARCHITECTURE.md
        #[arg(short, long)]
        force: bool,

        /// Specify target version (default: latest)
        #[arg(long)]
        version: Option<String>,
    },

    /// Migrate from .ai/ to .✨/ directory
    Migrate {
        /// Target directory path (default: current directory)
        #[arg(short, long)]
        path: Option<String>,

        /// Remove .ai/ after successful migration
        #[arg(long)]
        remove_old: bool,

        /// Dry run - show what would be done
        #[arg(long)]
        dry_run: bool,
    },

    /// Check protocol integrity and configuration
    Check {
        /// Automatically fix issues
        #[arg(long)]
        fix: bool,

        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
    },

    /// Show current protocol status
    Status,

    /// Initialize a new project with Git-Core Protocol
    Init {
        /// Project name
        name: Option<String>,

        /// Target directory path (default: current directory or new folder)
        #[arg(short, long)]
        path: Option<String>,

        /// Project template (minimal, full, custom)
        #[arg(short, long, default_value = "full")]
        template: String,
    },

    /// Update version number and changelog
    Version {
        /// Version bump type (major, minor, patch)
        bump: Option<String>,

        /// Set specific version
        #[arg(long)]
        set: Option<String>,
    },

    /// Self-update git-core CLI
    SelfUpdate {
        /// Force update even if on latest version
        #[arg(short, long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::registry()
        .with(fmt::layer().without_time().with_target(false))
        .with(filter)
        .init();

    // Print header
    println!();
    println!("{}", style("🧠 Git-Core Protocol CLI").cyan().bold());
    println!("{}", style("═".repeat(40)).dim());
    println!();

    // Execute command
    let result = match cli.command {
        Commands::Install { path, force, organize, version } => {
            commands::install::run(path, force, organize, version, cli.yes).await
        }
        Commands::Upgrade { path, force, version } => {
            commands::upgrade::run(path, force, version, cli.yes).await
        }
        Commands::Migrate { path, remove_old, dry_run } => {
            commands::migrate::run(path, remove_old, dry_run, cli.yes).await
        }
        Commands::Check { fix, format } => {
            commands::check::run(fix, &format).await
        }
        Commands::Status => {
            commands::status::run().await
        }
        Commands::Init { name, path, template } => {
            commands::init::run(name, path, &template, cli.yes).await
        }
        Commands::Version { bump, set } => {
            commands::version::run(bump, set).await
        }
        Commands::SelfUpdate { force } => {
            commands::self_update::run(force).await
        }
    };

    // Handle result
    match result {
        Ok(_) => {
            println!();
            println!("{}", style("✅ Done!").green().bold());
            Ok(())
        }
        Err(e) => {
            eprintln!();
            eprintln!("{} {}", style("❌ Error:").red().bold(), e);
            std::process::exit(1);
        }
    }
}
