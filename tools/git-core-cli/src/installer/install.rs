//! Installation functionality

use anyhow::Result;
use std::path::Path;
use tempfile::TempDir;

use crate::config::{NEW_AI_DIR, OLD_AI_DIR, PRESERVE_FILES};
use crate::utils::{self, copy_dir_recursive, print_success, print_warning};

/// Install protocol files from temp directory
pub async fn install_files(temp_dir: &TempDir, upgrade: bool) -> Result<()> {
    let src = temp_dir.path();

    // Handle AI directory (.✨ or .ai in template)
    let template_ai_dir = if src.join(".✨").exists() {
        src.join(".✨")
    } else if src.join(".ai").exists() {
        src.join(".ai")
    } else {
        return Err(anyhow::anyhow!("No AI directory found in template"));
    };

    let dest_ai_dir = Path::new(NEW_AI_DIR);

    if upgrade {
        // Remove old directories
        utils::remove_dir_if_exists(Path::new(NEW_AI_DIR))?;
        utils::remove_dir_if_exists(Path::new(OLD_AI_DIR))?;
    }

    // Install AI directory
    if !dest_ai_dir.exists() || upgrade {
        utils::ensure_dir(dest_ai_dir)?;
        copy_dir_recursive(&template_ai_dir, dest_ai_dir)?;

        if upgrade {
            print_success(".✨/ (upgraded)");
        } else {
            print_success(".✨/");
        }
    } else {
        // Merge new files only
        print_warning(".✨/ (exists, merging new files)");
        merge_directory(&template_ai_dir, dest_ai_dir)?;
    }

    // Install other directories
    let dirs = [".github", "scripts", "docs"];
    for dir in dirs {
        let src_dir = src.join(dir);
        let dst_dir = Path::new(dir);

        if src_dir.exists() {
            if upgrade {
                utils::remove_dir_if_exists(dst_dir)?;
                copy_dir_recursive(&src_dir, dst_dir)?;
                print_success(&format!("{}/ (upgraded)", dir));
            } else if !dst_dir.exists() {
                copy_dir_recursive(&src_dir, dst_dir)?;
                print_success(&format!("{}/", dir));
            } else {
                merge_directory(&src_dir, dst_dir)?;
                print_success(&format!("{}/ (merged)", dir));
            }
        }
    }

    // Install protocol files
    let protocol_files = [
        ".cursorrules",
        ".windsurfrules",
        "AGENTS.md",
        ".git-core-protocol-version",
    ];

    for file in protocol_files {
        let src_file = src.join(file);
        let dst_file = Path::new(file);

        if src_file.exists() {
            if upgrade {
                std::fs::copy(&src_file, dst_file)?;
                print_success(&format!("{} (upgraded)", file));
            } else if !dst_file.exists() {
                std::fs::copy(&src_file, dst_file)?;
                print_success(file);
            } else {
                print_warning(&format!("{} (exists)", file));
            }
        }
    }

    // Install files that should never be overwritten
    for file in PRESERVE_FILES {
        let src_file = src.join(file);
        let dst_file = Path::new(file);

        if src_file.exists() && !dst_file.exists() {
            std::fs::copy(&src_file, dst_file)?;
            print_success(file);
        } else if dst_file.exists() {
            print_warning(&format!("{} (preserved)", file));
        }
    }

    Ok(())
}

/// Merge new files into existing directory
fn merge_directory(src: &Path, dst: &Path) -> Result<()> {
    utils::ensure_dir(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            merge_directory(&src_path, &dst_path)?;
        } else if !dst_path.exists() {
            std::fs::copy(&src_path, &dst_path)?;
            print_success(&format!("  + {}", entry.file_name().to_string_lossy()));
        }
    }

    Ok(())
}
