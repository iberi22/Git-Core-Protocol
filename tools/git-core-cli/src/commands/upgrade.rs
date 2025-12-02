//! Upgrade command - Update existing Git-Core Protocol installation

use anyhow::Result;
use console::style;

use crate::config::{Config, NEW_AI_DIR, PRESERVE_FILES};
use crate::installer;
use crate::utils::{self, confirm, print_header, print_info, print_success, print_warning};

/// Run the upgrade command
pub async fn run(path: Option<String>, force: bool, version: Option<String>, auto_yes: bool) -> Result<()> {
    print_header("🔄 Upgrading Git-Core Protocol");

    // Resolve target directory
    let target_path = utils::resolve_target_path(path, auto_yes)?;
    print_info(&format!("Target: {}", style(target_path.display()).cyan()));

    // Change to target directory
    utils::change_to_target_dir(&target_path)?;

    // Check if protocol is installed
    let config = Config::load()?;
    if !config.is_installed() {
        print_warning("Git-Core Protocol is not installed. Use 'install' command instead.");
        return Ok(());
    }

    let current_version = &config.version;
    print_info(&format!("Current version: {}", style(current_version).yellow()));

    // Get latest version
    let spinner = utils::create_spinner("Checking for updates...");
    let latest_version = installer::download::get_latest_version().await?;
    spinner.finish_and_clear();

    print_info(&format!("Latest version: {}", style(&latest_version).green()));

    // Check if upgrade is needed
    if current_version == &latest_version && !force {
        print_success("You're already on the latest version!");
        return Ok(());
    }

    // Confirm upgrade
    if !force && !auto_yes {
        let msg = format!(
            "Upgrade from v{} to v{}? (Your ARCHITECTURE.md will be preserved)",
            current_version, latest_version
        );
        if !confirm(&msg, true)? {
            print_info("Upgrade cancelled.");
            return Ok(());
        }
    }

    // Warn about force mode
    if force {
        print_warning("FORCE MODE: All files will be overwritten, including ARCHITECTURE.md!");
        if !auto_yes && !confirm("Are you sure?", false)? {
            return Ok(());
        }
    }

    // Backup user files
    print_header("💾 Backing up user files");
    installer::backup::backup_user_files().await?;

    // Download and install
    print_header("📥 Downloading update");
    let spinner = utils::create_spinner("Downloading...");
    let temp_dir = installer::download::fetch_protocol(version.as_deref()).await?;
    spinner.finish_with_message("Downloaded!");

    print_header("📦 Installing update");
    installer::install::install_files(&temp_dir, true).await?;

    // Restore user files (unless force mode)
    if !force {
        print_header("📥 Restoring user files");
        installer::backup::restore_user_files().await?;
    }

    // Show completion
    let new_config = Config::load()?;

    println!();
    println!("{}", style("════════════════════════════════════════").green());
    println!("  {} Upgraded: {} → {}",
        style("✅").green(),
        style(current_version).yellow(),
        style(&new_config.version).green()
    );
    if !force {
        println!("  {} Your ARCHITECTURE.md was preserved", style("✓").green());
    }
    println!("{}", style("════════════════════════════════════════").green());

    Ok(())
}
