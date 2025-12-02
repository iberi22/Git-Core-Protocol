//! Utility functions for Git-Core CLI

use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;

/// Create a spinner with a message
pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

/// Create a progress bar
pub fn create_progress_bar(len: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    pb.set_message(message.to_string());
    pb
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("  {} {}", style("✓").green().bold(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("  {} {}", style("⚠").yellow().bold(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("  {} {}", style("ℹ").blue().bold(), message);
}

/// Print a section header
pub fn print_header(message: &str) {
    println!();
    println!("{}", style(message).cyan().bold());
}

/// Check if we're in a git repository
pub fn is_git_repo() -> bool {
    Path::new(".git").exists()
}

/// Check if directory is empty (excluding hidden files)
pub fn is_dir_empty() -> Result<bool> {
    let entries: Vec<_> = std::fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.file_name()
                .to_string_lossy()
                .starts_with('.')
        })
        .collect();

    Ok(entries.is_empty())
}

/// Ensure directory exists
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Copy directory recursively
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    ensure_dir(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// Remove directory if it exists
pub fn remove_dir_if_exists(path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// Confirm action with user
pub fn confirm(message: &str, default: bool) -> Result<bool> {
    use dialoguer::Confirm;

    Ok(Confirm::new()
        .with_prompt(message)
        .default(default)
        .interact()?)
}

/// Select from options
pub fn select(message: &str, options: &[&str]) -> Result<usize> {
    use dialoguer::Select;

    Ok(Select::new()
        .with_prompt(message)
        .items(options)
        .default(0)
        .interact()?)
}

/// Prompt for a path with autocomplete
pub fn prompt_path(message: &str, default: &str) -> Result<String> {
    use dialoguer::Input;

    Ok(Input::new()
        .with_prompt(message)
        .default(default.to_string())
        .interact_text()?)
}

/// Resolve and validate target path
/// If path is None, prompts the user interactively (unless auto_yes is true)
pub fn resolve_target_path(path: Option<String>, auto_yes: bool) -> Result<std::path::PathBuf> {
    let target = match path {
        Some(p) => std::path::PathBuf::from(p),
        None => {
            if auto_yes {
                // In auto mode, use current directory
                std::env::current_dir()?
            } else {
                // Interactive mode: ask user
                let current = std::env::current_dir()?;
                let current_str = current.to_string_lossy().to_string();

                let input = prompt_path(
                    "📂 Target directory path",
                    &current_str
                )?;

                std::path::PathBuf::from(input)
            }
        }
    };

    // Expand to absolute path
    let absolute = if target.is_absolute() {
        target
    } else {
        std::env::current_dir()?.join(target)
    };

    Ok(absolute)
}

/// Change to target directory (creating it if needed)
pub fn change_to_target_dir(path: &std::path::Path) -> Result<()> {
    ensure_dir(path)?;
    std::env::set_current_dir(path)?;
    Ok(())
}
