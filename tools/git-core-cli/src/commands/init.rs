//! Init command - Initialize a new project

use anyhow::Result;
use console::style;
use std::path::Path;

use crate::utils::{self, confirm, print_header, print_info, print_success};

/// Run the init command
pub async fn run(name: Option<String>, path: Option<String>, template: &str, auto_yes: bool) -> Result<()> {
    print_header("🚀 Initializing new Git-Core project");

    // Resolve target directory
    let target_path = utils::resolve_target_path(path, auto_yes)?;
    print_info(&format!("Target: {}", style(target_path.display()).cyan()));

    // Change to target directory
    utils::change_to_target_dir(&target_path)?;

    let project_name = match name {
        Some(n) => n,
        None => {
            // Use target directory name
            target_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "my-project".to_string())
        }
    };

    print_info(&format!("Project: {}", style(&project_name).cyan()));
    print_info(&format!("Template: {}", style(template).cyan()));

    // Check if already initialized
    if Path::new(".git").exists() && !auto_yes {
        if !confirm("Git repository already exists. Continue?", true)? {
            return Ok(());
        }
    }

    // Initialize git if needed
    if !Path::new(".git").exists() {
        print_info("Initializing git repository...");
        std::process::Command::new("git")
            .args(["init"])
            .output()?;
        print_success("Git repository initialized");
    }

    // Install protocol (None for path since we're already in target dir, true for auto_yes)
    print_info("Installing Git-Core Protocol...");
    crate::commands::install::run(None, false, false, None, true).await?;

    // Create initial ARCHITECTURE.md
    create_architecture_file(&project_name, template)?;

    // Create initial issue
    print_header("📋 Next Steps");
    println!();
    println!("  1. Edit {} to document your architecture",
        style(".✨/ARCHITECTURE.md").cyan());
    println!("  2. Run {} to set up labels and initial issues",
        style("./scripts/init_project.sh").cyan());
    println!("  3. Start developing with AI assistance!");
    println!();
    println!("  {} Your state is GitHub Issues, not files!",
        style("Remember:").yellow().bold());

    Ok(())
}

fn create_architecture_file(project_name: &str, template: &str) -> Result<()> {
    let ai_dir = Path::new(".✨");
    utils::ensure_dir(ai_dir)?;

    let arch_path = ai_dir.join("ARCHITECTURE.md");

    // Don't overwrite existing
    if arch_path.exists() {
        return Ok(());
    }

    let content = match template {
        "minimal" => format!(r#"# {} Architecture

## Overview
<!-- Describe your project -->

## Stack
<!-- List your technologies -->

## Structure
<!-- Describe your file structure -->
"#, project_name),

        "full" | _ => format!(r#"# {} Architecture

## Overview
<!-- Brief description of your project -->

## Tech Stack
| Category | Technology | Why |
|----------|------------|-----|
| Language | | |
| Framework | | |
| Database | | |
| Hosting | | |

## CRITICAL DECISIONS
| Decision | Choice | Rationale | Date |
|----------|--------|-----------|------|
| | | | |

## Directory Structure
```
/
├── src/           # Source code
├── tests/         # Test files
├── docs/          # Documentation
├── scripts/       # Utility scripts
└── .✨/           # AI context
```

## Key Components
<!-- Describe main modules/components -->

## Data Flow
<!-- How data moves through your system -->

## Deployment
<!-- How the project is deployed -->

## Development Guidelines
- Follow Conventional Commits
- Reference GitHub Issues in commits
- Keep PRs focused and atomic
"#, project_name),
    };

    std::fs::write(&arch_path, content)?;
    print_success("Created .✨/ARCHITECTURE.md");

    Ok(())
}
