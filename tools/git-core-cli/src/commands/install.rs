//! Install command - Download and install Git-Core Protocol

use anyhow::Result;
use console::style;
use std::path::Path;

use crate::config::Config;
use crate::installer;
use crate::utils::{self, confirm, print_header, print_info, print_success, print_warning};

/// Run the install command
pub async fn run(
    path: Option<String>,
    force: bool,
    organize: bool,
    version: Option<String>,
    auto_yes: bool,
) -> Result<()> {
    print_header("📥 Installing Git-Core Protocol");

    // Resolve target directory
    let target_path = utils::resolve_target_path(path, auto_yes)?;
    print_info(&format!("Target: {}", style(target_path.display()).cyan()));

    // Change to target directory
    utils::change_to_target_dir(&target_path)?;

    // Check if already installed
    let config = Config::load()?;
    if config.is_installed() && !force {
        print_warning(&format!(
            "Protocol v{} is already installed. Use --force to reinstall or 'upgrade' command.",
            config.version
        ));

        if !auto_yes && !confirm("Continue anyway?", false)? {
            return Ok(());
        }
    }

    // Check if directory has files
    if !utils::is_dir_empty()? && !auto_yes {
        print_warning("Current directory is not empty.");

        let options = &[
            "Continue and merge files",
            "Organize existing files first",
            "Cancel",
        ];

        match utils::select("What would you like to do?", options)? {
            0 => {} // Continue
            1 => organize_files()?,
            2 => {
                print_info("Installation cancelled.");
                return Ok(());
            }
            _ => unreachable!(),
        }
    } else if organize {
        organize_files()?;
    }

    // Download protocol
    let spinner = utils::create_spinner("Downloading Git-Core Protocol...");
    let temp_dir = installer::download::fetch_protocol(version.as_deref()).await?;
    spinner.finish_with_message("Downloaded!");

    // Install files
    print_header("📦 Installing protocol files");
    installer::install::install_files(&temp_dir, force).await?;

    // Run migration if needed
    if Config::needs_migration() {
        print_header("🔄 Migrating directory structure");
        crate::commands::migrate::run_migration(false, false)?;
    }

    // Show completion
    let new_config = Config::load()?;
    print_header("✅ Installation Complete");

    println!();
    println!("  {} Protocol v{} installed successfully!",
        style("🧠").cyan(),
        style(&new_config.version).green()
    );
    println!();
    println!("  {} Files installed:", style("📋").cyan());
    println!("     .✨/ARCHITECTURE.md  - Document your architecture here");
    println!("     .github/             - Copilot rules + workflows");
    println!("     scripts/             - Init and update scripts");
    println!("     AGENTS.md            - Rules for all AI agents");
    println!();
    println!("  {} Next step:", style("🚀").yellow());
    println!("     Run: {} or {}",
        style("./scripts/init_project.sh").cyan(),
        style(".\\scripts\\init_project.ps1").cyan()
    );

    Ok(())
}

/// Organize existing files into appropriate directories
fn organize_files() -> Result<()> {
    print_info("Organizing existing files...");

    // Create directories
    for dir in &["docs/archive", "scripts", "tests", "src"] {
        utils::ensure_dir(Path::new(dir))?;
    }

    // Files to keep in root
    let keep_in_root = ["README.md", "AGENTS.md", "CHANGELOG.md", "CONTRIBUTING.md", "LICENSE.md", "LICENSE"];

    // Move markdown files to docs/archive
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |e| e == "md") {
            let filename = path.file_name().unwrap().to_string_lossy();

            if !keep_in_root.contains(&filename.as_ref()) {
                let dest = Path::new("docs/archive").join(&*filename);
                std::fs::rename(&path, &dest)?;
                print_info(&format!("Moved {} → docs/archive/", filename));
            }
        }
    }

    print_success("Files organized!");
    Ok(())
}
