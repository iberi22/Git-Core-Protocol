//! Migrate command - Migrate from .ai/ to .✨/ directory

use anyhow::Result;
use console::style;
use std::path::Path;

use crate::config::{NEW_AI_DIR, OLD_AI_DIR};
use crate::utils::{self, confirm, copy_dir_recursive, print_header, print_info, print_success, print_warning};

/// Run the migrate command
pub async fn run(path: Option<String>, remove_old: bool, dry_run: bool, auto_yes: bool) -> Result<()> {
    print_header("🔄 Migrating directory structure");

    // Resolve target directory
    let target_path = utils::resolve_target_path(path, auto_yes)?;
    print_info(&format!("Target: {}", style(target_path.display()).cyan()));

    // Change to target directory
    utils::change_to_target_dir(&target_path)?;

    let old_dir = Path::new(OLD_AI_DIR);
    let new_dir = Path::new(NEW_AI_DIR);

    // Check if migration is needed
    if !old_dir.exists() {
        if new_dir.exists() {
            print_success("Already using .✨/ directory. No migration needed!");
        } else {
            print_warning("No .ai/ or .✨/ directory found. Run 'install' first.");
        }
        return Ok(());
    }

    if new_dir.exists() {
        print_warning(".✨/ already exists. Migration may overwrite files.");
        if !auto_yes && !confirm("Continue?", false)? {
            return Ok(());
        }
    }

    // Run migration
    if dry_run {
        print_info("DRY RUN - No changes will be made");
        println!();
        println!("  Would migrate:");

        for entry in std::fs::read_dir(old_dir)? {
            let entry = entry?;
            let name = entry.file_name();
            println!("    {} → .✨/{}",
                style(format!(".ai/{}", name.to_string_lossy())).yellow(),
                style(name.to_string_lossy()).green()
            );
        }

        if remove_old {
            println!();
            println!("  {} Would remove .ai/ directory", style("⚠").yellow());
        }
    } else {
        run_migration(remove_old, auto_yes)?;
    }

    Ok(())
}

/// Perform the actual migration
pub fn run_migration(remove_old: bool, auto_yes: bool) -> Result<()> {
    let old_dir = Path::new(OLD_AI_DIR);
    let new_dir = Path::new(NEW_AI_DIR);

    if !old_dir.exists() {
        return Ok(());
    }

    // Create new directory and copy files
    utils::ensure_dir(new_dir)?;

    let spinner = utils::create_spinner("Migrating files...");

    for entry in std::fs::read_dir(old_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = new_dir.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    spinner.finish_with_message("Migration complete!");

    print_success(&format!("Migrated {} → {}", OLD_AI_DIR, NEW_AI_DIR));

    // Optionally remove old directory
    if remove_old {
        std::fs::remove_dir_all(old_dir)?;
        print_success("Removed .ai/ directory");
    } else {
        print_info("You can safely remove .ai/ after verifying the migration");
    }

    Ok(())
}
