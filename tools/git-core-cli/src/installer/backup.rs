//! Backup and restore functionality

use anyhow::Result;
use std::path::Path;

use crate::config::{self, BACKUP_DIR, NEW_AI_DIR, OLD_AI_DIR, PROTOCOL_WORKFLOWS};
use crate::utils::{self, print_success};

/// Backup user files before upgrade
pub async fn backup_user_files() -> Result<()> {
    let backup_dir = Path::new(BACKUP_DIR);
    utils::ensure_dir(backup_dir)?;

    // Find AI directory
    let ai_dir = if Path::new(NEW_AI_DIR).exists() {
        Path::new(NEW_AI_DIR)
    } else if Path::new(OLD_AI_DIR).exists() {
        Path::new(OLD_AI_DIR)
    } else {
        return Ok(());
    };

    // Backup ARCHITECTURE.md
    let arch_path = ai_dir.join("ARCHITECTURE.md");
    if arch_path.exists() {
        std::fs::copy(&arch_path, backup_dir.join("ARCHITECTURE.md"))?;
        print_success(&format!("{}/ARCHITECTURE.md backed up", ai_dir.display()));
    }

    // Backup CONTEXT_LOG.md
    let context_path = ai_dir.join("CONTEXT_LOG.md");
    if context_path.exists() {
        std::fs::copy(&context_path, backup_dir.join("CONTEXT_LOG.md"))?;
        print_success(&format!("{}/CONTEXT_LOG.md backed up", ai_dir.display()));
    }

    // Backup custom workflows
    let workflows_dir = Path::new(".github/workflows");
    if workflows_dir.exists() {
        let backup_workflows = backup_dir.join("workflows");
        utils::ensure_dir(&backup_workflows)?;

        for entry in std::fs::read_dir(workflows_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "yml" || e == "yaml") {
                let filename = path.file_name().unwrap().to_string_lossy();

                // Only backup non-protocol workflows
                if !PROTOCOL_WORKFLOWS.contains(&filename.as_ref()) {
                    std::fs::copy(&path, backup_workflows.join(&*filename))?;
                    print_success(&format!("Custom workflow: {} backed up", filename));
                }
            }
        }
    }

    Ok(())
}

/// Restore user files after upgrade
pub async fn restore_user_files() -> Result<()> {
    let backup_dir = Path::new(BACKUP_DIR);

    if !backup_dir.exists() {
        return Ok(());
    }

    let ai_dir = Path::new(NEW_AI_DIR);
    utils::ensure_dir(ai_dir)?;

    // Restore ARCHITECTURE.md
    let arch_backup = backup_dir.join("ARCHITECTURE.md");
    if arch_backup.exists() {
        std::fs::copy(&arch_backup, ai_dir.join("ARCHITECTURE.md"))?;
        print_success(".✨/ARCHITECTURE.md restored");
    }

    // Restore CONTEXT_LOG.md
    let context_backup = backup_dir.join("CONTEXT_LOG.md");
    if context_backup.exists() {
        std::fs::copy(&context_backup, ai_dir.join("CONTEXT_LOG.md"))?;
        print_success(".✨/CONTEXT_LOG.md restored");
    }

    // Restore custom workflows
    let backup_workflows = backup_dir.join("workflows");
    if backup_workflows.exists() {
        let workflows_dir = Path::new(".github/workflows");
        utils::ensure_dir(workflows_dir)?;

        for entry in std::fs::read_dir(&backup_workflows)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path.file_name().unwrap();

            std::fs::copy(&path, workflows_dir.join(filename))?;
            print_success(&format!("Custom workflow restored: {}", filename.to_string_lossy()));
        }
    }

    // Cleanup backup directory
    std::fs::remove_dir_all(backup_dir)?;

    Ok(())
}
