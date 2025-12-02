//! Status command - Show current protocol status

use anyhow::Result;
use console::style;
use std::path::Path;

use crate::config::{Config, NEW_AI_DIR, OLD_AI_DIR};
use crate::utils::print_header;

/// Run the status command
pub async fn run() -> Result<()> {
    print_header("📊 Git-Core Protocol Status");

    let config = Config::load()?;

    // Version info
    println!();
    if config.is_installed() {
        println!("  {} {}",
            style("Version:").dim(),
            style(&config.version).green().bold()
        );
    } else {
        println!("  {} {}",
            style("Version:").dim(),
            style("Not installed").red().bold()
        );
    }

    // Directory info
    let ai_dir = if Path::new(NEW_AI_DIR).exists() {
        format!("{} (modern)", NEW_AI_DIR)
    } else if Path::new(OLD_AI_DIR).exists() {
        format!("{} (legacy - run 'git-core migrate')", OLD_AI_DIR)
    } else {
        "None".to_string()
    };

    println!("  {} {}", style("AI Directory:").dim(), ai_dir);

    // Files check
    println!();
    println!("  {}", style("Files:").cyan().bold());

    let files_to_check = [
        ("AGENTS.md", "Agent configuration"),
        (".github/copilot-instructions.md", "Copilot instructions"),
        (".cursorrules", "Cursor rules"),
        (".windsurfrules", "Windsurf rules"),
    ];

    for (file, desc) in files_to_check {
        let status = if Path::new(file).exists() {
            style("✓").green()
        } else {
            style("✗").red()
        };
        println!("    {} {} - {}", status, file, style(desc).dim());
    }

    // AI directory files
    let ai_path = Config::get_ai_dir();
    if let Some(ai_path) = ai_path {
        println!();
        println!("  {}", style(format!("{}/", ai_path.display())).cyan().bold());

        let ai_files = ["ARCHITECTURE.md", "CONTEXT_LOG.md", "AGENT_INDEX.md"];
        for file in ai_files {
            let file_path = ai_path.join(file);
            let status = if file_path.exists() {
                style("✓").green()
            } else {
                style("✗").red()
            };
            println!("    {} {}", status, file);
        }
    }

    // Git info
    println!();
    println!("  {}", style("Git:").cyan().bold());

    if Path::new(".git").exists() {
        println!("    {} Repository initialized", style("✓").green());
    } else {
        println!("    {} Not a git repository", style("✗").yellow());
    }

    // Suggestions
    if !config.is_installed() {
        println!();
        println!("  {} Run {} to install the protocol",
            style("💡").yellow(),
            style("git-core install").cyan()
        );
    } else if Path::new(OLD_AI_DIR).exists() && !Path::new(NEW_AI_DIR).exists() {
        println!();
        println!("  {} Run {} to migrate to the new directory structure",
            style("💡").yellow(),
            style("git-core migrate").cyan()
        );
    }

    Ok(())
}
