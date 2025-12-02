//! Check command - Verify protocol integrity

use anyhow::Result;
use console::style;
use serde::Serialize;
use std::path::Path;

use crate::config::{self, Config, NEW_AI_DIR, OLD_AI_DIR, VERSION_FILE};
use crate::utils::{print_header, print_info, print_success, print_warning};

#[derive(Debug, Serialize)]
struct CheckResult {
    pub passed: bool,
    pub issues: Vec<Issue>,
    pub version: String,
    pub ai_directory: String,
}

#[derive(Debug, Serialize, Clone)]
struct Issue {
    pub severity: String,
    pub message: String,
    pub fix: Option<String>,
}

/// Run the check command
pub async fn run(fix: bool, format: &str) -> Result<()> {
    print_header("🔍 Checking Git-Core Protocol integrity");

    let mut issues: Vec<Issue> = Vec::new();
    let config = Config::load()?;

    // Check 1: Protocol installed
    if !config.is_installed() {
        issues.push(Issue {
            severity: "error".to_string(),
            message: "Git-Core Protocol is not installed".to_string(),
            fix: Some("Run: git-core install".to_string()),
        });
    } else {
        print_success(&format!("Protocol installed: v{}", config.version));
    }

    // Check 2: Directory structure
    let new_dir = Path::new(NEW_AI_DIR);
    let old_dir = Path::new(OLD_AI_DIR);

    let ai_directory = if new_dir.exists() {
        print_success("Using modern .✨/ directory");
        NEW_AI_DIR.to_string()
    } else if old_dir.exists() {
        issues.push(Issue {
            severity: "warning".to_string(),
            message: "Using legacy .ai/ directory".to_string(),
            fix: Some("Run: git-core migrate".to_string()),
        });
        OLD_AI_DIR.to_string()
    } else {
        issues.push(Issue {
            severity: "error".to_string(),
            message: "No AI directory found (.✨/ or .ai/)".to_string(),
            fix: Some("Run: git-core install".to_string()),
        });
        "none".to_string()
    };

    // Check 3: ARCHITECTURE.md exists
    let arch_path = if new_dir.exists() {
        new_dir.join("ARCHITECTURE.md")
    } else if old_dir.exists() {
        old_dir.join("ARCHITECTURE.md")
    } else {
        Path::new("ARCHITECTURE.md").to_path_buf()
    };

    if arch_path.exists() {
        print_success("ARCHITECTURE.md exists");
    } else {
        issues.push(Issue {
            severity: "warning".to_string(),
            message: "ARCHITECTURE.md not found".to_string(),
            fix: Some("Create .✨/ARCHITECTURE.md with your project architecture".to_string()),
        });
    }

    // Check 4: AGENTS.md exists
    if Path::new("AGENTS.md").exists() {
        print_success("AGENTS.md exists");
    } else {
        issues.push(Issue {
            severity: "warning".to_string(),
            message: "AGENTS.md not found in project root".to_string(),
            fix: Some("Run: git-core install --force".to_string()),
        });
    }

    // Check 5: GitHub workflows
    let workflows_dir = Path::new(".github/workflows");
    if workflows_dir.exists() {
        let required_workflows = ["update-protocol.yml"];
        for workflow in required_workflows {
            if workflows_dir.join(workflow).exists() {
                print_success(&format!("Workflow {} exists", workflow));
            } else {
                issues.push(Issue {
                    severity: "info".to_string(),
                    message: format!("Workflow {} not found", workflow),
                    fix: Some("Run: git-core upgrade".to_string()),
                });
            }
        }
    } else {
        issues.push(Issue {
            severity: "warning".to_string(),
            message: ".github/workflows directory not found".to_string(),
            fix: Some("Run: git-core install --force".to_string()),
        });
    }

    // Check 6: Copilot instructions
    if Path::new(".github/copilot-instructions.md").exists() {
        print_success("Copilot instructions configured");
    } else {
        issues.push(Issue {
            severity: "info".to_string(),
            message: "Copilot instructions not found".to_string(),
            fix: Some("Run: git-core upgrade".to_string()),
        });
    }

    // Summary
    println!();
    let result = CheckResult {
        passed: issues.iter().all(|i| i.severity != "error"),
        issues: issues.clone(),
        version: config.version.clone(),
        ai_directory,
    };

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            print_summary(&issues, fix)?;
        }
    }

    // Auto-fix if requested
    if fix && !issues.is_empty() {
        print_header("🔧 Applying fixes");
        apply_fixes(&issues).await?;
    }

    Ok(())
}

fn print_summary(issues: &[Issue], fix_available: bool) -> Result<()> {
    let errors = issues.iter().filter(|i| i.severity == "error").count();
    let warnings = issues.iter().filter(|i| i.severity == "warning").count();
    let infos = issues.iter().filter(|i| i.severity == "info").count();

    if issues.is_empty() {
        println!("{}", style("✅ All checks passed!").green().bold());
    } else {
        println!("{}", style("📊 Summary:").cyan().bold());

        if errors > 0 {
            println!("  {} {} error(s)", style("❌").red(), errors);
        }
        if warnings > 0 {
            println!("  {} {} warning(s)", style("⚠").yellow(), warnings);
        }
        if infos > 0 {
            println!("  {} {} info(s)", style("ℹ").blue(), infos);
        }

        println!();
        println!("{}", style("Issues found:").yellow());

        for issue in issues {
            let icon = match issue.severity.as_str() {
                "error" => style("❌").red(),
                "warning" => style("⚠").yellow(),
                _ => style("ℹ").blue(),
            };

            println!("  {} {}", icon, issue.message);
            if let Some(fix) = &issue.fix {
                println!("    {} {}", style("Fix:").dim(), style(fix).cyan());
            }
        }

        if fix_available && !issues.is_empty() {
            println!();
            print_info("Run with --fix to automatically fix issues");
        }
    }

    Ok(())
}

async fn apply_fixes(issues: &[Issue]) -> Result<()> {
    for issue in issues {
        if issue.severity == "warning" && issue.message.contains("legacy .ai/") {
            print_info("Migrating directory structure...");
            crate::commands::migrate::run_migration(false, true)?;
        }
    }

    print_success("Fixes applied where possible");
    Ok(())
}
