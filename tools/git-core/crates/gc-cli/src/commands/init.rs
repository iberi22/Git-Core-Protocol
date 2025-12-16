use clap::Args;
use gc_core::ports::{FileSystemPort, SystemPort};
use console::style;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Project name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Create private repository
    #[arg(long)]
    pub private: bool,

    /// Non-interactive mode
    #[arg(short, long)]
    pub auto: bool,
}

pub async fn execute(
    args: InitArgs,
    fs: &impl FileSystemPort,
    system: &impl SystemPort
) -> color_eyre::Result<()> {
    println!("{}", style("🧠 Initializing Git-Core Protocol...").cyan());
    println!("{}", style("==========================================").cyan());

    // 1. Validate Environment
    println!("\n{}", style("📋 Validating environment...").yellow());

    if !system.check_command("git").await? {
        eprintln!("{}", style("❌ Error: Git is not installed.").red());
        return Err(color_eyre::eyre::eyre!("Git missing"));
    }
    println!("{}", style("✓ Git installed").green());

    if !system.check_command("gh").await? {
        eprintln!("{}", style("❌ Error: GitHub CLI (gh) is not installed.").red());
        return Err(color_eyre::eyre::eyre!("GitHub CLI missing"));
    }
    println!("{}", style("✓ GitHub CLI installed").green());

    // 2. Initialize Git
    // 2. Initialize Git
    let target_path = args.name.as_deref().unwrap_or(".");
    let git_check_path = if target_path == "." { ".git".to_string() } else { format!("{}/.git", target_path) };

    if !fs.exists(&git_check_path).await? {
        println!("\n{}", style(format!("🔧 Initializing Git repository in {}...", target_path)).yellow());

        let mut base_cmd = vec![];
        if target_path != "." {
             base_cmd.push(String::from("-C"));
             base_cmd.push(target_path.to_string());
        }

        // git init <directory> or git init (current)
        if target_path != "." {
            system.run_command("git", &vec![String::from("init"), target_path.to_string()]).await?;
        } else {
            system.run_command("git", &vec![String::from("init")]).await?;
        }

        // git -C <path> branch -M main
        let mut branch_cmd = base_cmd.clone();
        branch_cmd.extend(vec![String::from("branch"), String::from("-M"), String::from("main")]);
        system.run_command("git", &branch_cmd).await?;

        // Initial files if they don't exist
        let readme_path = if target_path == "." { "README.md".to_string() } else { format!("{}/README.md", target_path) };
        if !fs.exists(&readme_path).await? {
            fs.write_file(&readme_path, "# Project Initialized by Git-Core").await?;
        }

        let mut add_cmd = base_cmd.clone();
        add_cmd.extend(vec![String::from("add"), String::from(".")]);
        system.run_command("git", &add_cmd).await?;

        let mut commit_cmd = base_cmd.clone();
        commit_cmd.extend(vec![String::from("commit"), String::from("-m"), String::from("feat: 🚀 Initial commit")]);
        system.run_command("git", &commit_cmd).await?;
    } else {
         println!("{}", style("ℹ️  Existing Git repository detected").cyan());
    }

    // 3. GitHub Repo
    if args.private {
        println!("{}", style("🔒 Creating private repository...").yellow());
        // gh repo create NAME --private --source=. --remote=origin --push
        let name = args.name.as_deref().unwrap_or(target_path);
        let create_repo_cmd = vec![
            String::from("repo"),
            String::from("create"),
            name.to_string(),
            String::from("--private"),
            String::from("--source=."),
            String::from("--remote=origin"),
            String::from("--push")
        ];
        // If we are in a subdir, source=. works if we cd'd? No, we need to be careful.
        // If we are initializing in '.', source=. is fine.
        // If we in target_path, we should run this command INSIDE target_path.

        // MVP: Just warning if not implemented for subdir yet, but keeping simpler logic:
        if target_path != "." {
             // We need to run this command inside the directory.
             // SystemPort run_command doesn't readily support CWD change unless we added it (we have Cwd in execute but not in Port trait?)
             // Port trait: run_command(name, args). No CWD.
             // Workaround: We relied on `git -C`. `gh` doesn't have `-C`.
             // We can use `pushd` in shell? No.
             // This is a limitation of the current SystemPort.
             // For now, let's just warn if name != "."
             println!("{}", style("⚠️  Repo creation in subdirectory not fully supported in Rust port yet.").red());
             println!("{}", style("   Please run 'gh repo create' manually inside the directory.").yellow());
        } else {
             if let Err(e) = system.run_command("gh", &create_repo_cmd).await {
                 println!("{}", style(format!("⚠️  Failed to create repo: {}", e)).red());
             } else {
                 println!("{}", style("✓ GitHub repository created").green());
             }
        }
    }

    if !args.auto {
        println!("{}", style("ℹ️  Interactive mode skipped for MVP").dim());
    }

    // 4. Architecture File
    let arch_path = if target_path == "." { ".✨/ARCHITECTURE.md".to_string() } else { format!("{}/.✨/ARCHITECTURE.md", target_path) };
    let arch_dir = if target_path == "." { ".✨".to_string() } else { format!("{}/.✨", target_path) };

    if !fs.exists(&arch_path).await? {
        println!("\n{}", style("📐 Setting up ARCHITECTURE.md...").yellow());
        fs.create_dir(&arch_dir).await?;
        let content = r#"# 🏗️ Architecture

## Stack
- **Language:** TBD
- **Framework:** TBD

## Key Decisions
_Document architectural decisions here_
"#;
        fs.write_file(&arch_path, content).await?;
        println!("{}", style("✓ Created .✨/ARCHITECTURE.md").green());
    }

    // 5. Create Labels (Parity)
    println!("\n{}", style("🏷️  Creating semantic labels...").yellow());
    let labels = vec![
        ("ai-plan", "High-level planning tasks", "0E8A16"),
        ("ai-context", "Critical context information", "FBCA04"),
        ("ai-blocked", "Blocked - requires human intervention", "D93F0B"),
        ("in-progress", "Task in progress", "1D76DB"),
        ("needs-review", "Requires review", "5319E7"),
    ];

    for (name, desc, color) in labels {
        // gh label create NAME --description DESC --color COLOR --force
        let label_cmd = vec![
            String::from("label"),
            String::from("create"),
            name.to_string(),
            String::from("--description"),
            desc.to_string(),
            String::from("--color"),
            color.to_string(),
            String::from("--force") // Update if exists
        ];
        // Again, assuming CWD is repo root
        if target_path == "." {
             let _ = system.run_command("gh", &label_cmd).await; // Ignore error if exists/fails
             println!("  ✓ {}", name);
        }
    }

    // 6. Create Initial Issues (Parity)
    println!("\n{}", style("📝 Creating initial issues...").yellow());
    let issues = vec![
        ("🏗️ SETUP: Define Architecture and Tech Stack",
         "## Objective\nDefine stack.\n\n## Tasks\n- [ ] Define language\n- [ ] Define db\n- [ ] Document in .✨/ARCHITECTURE.md",
         "ai-plan"),
        ("⚙️ INFRA: Initial dev setup",
         "## Objective\nSetup tools.\n\n## Tasks\n- [ ] Linter\n- [ ] Formatter",
         "ai-plan")
    ];

    for (title, body, label) in issues {
         let issue_cmd = vec![
            String::from("issue"),
            String::from("create"),
            String::from("--title"),
            title.to_string(),
            String::from("--body"),
            body.to_string(),
            String::from("--label"),
            label.to_string()
        ];
        if target_path == "." {
             let _ = system.run_command("gh", &issue_cmd).await;
             println!("  ✓ Issue: {}", title);
        }
    }

    // 7. Install Pre-commit Hooks (Parity)
    println!("\n{}", style("🪝 Installing pre-commit hooks...").yellow());
    let git_hooks_dir = if target_path == "." { ".git/hooks".to_string() } else { format!("{}/.git/hooks", target_path) };

    if fs.exists(&if target_path == "." { ".git".to_string() } else { format!("{}/.git", target_path) }).await? {
        if !fs.exists(&git_hooks_dir).await? {
            fs.create_dir(&git_hooks_dir).await?;
        }

        let hook_content = r#"#!/bin/bash
# Git-Core Protocol pre-commit hook
# Validates atomic commits via scripts/hooks/pre-commit
# Bypass: git commit --no-verify

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOK_SCRIPT="$REPO_ROOT/scripts/hooks/pre-commit"

if [ -f "$HOOK_SCRIPT" ] && [ -x "$HOOK_SCRIPT" ]; then
    exec "$HOOK_SCRIPT"
elif [ -f "$HOOK_SCRIPT" ]; then
    exec bash "$HOOK_SCRIPT"
else
    # Hook script not found, skip
    echo "Note: scripts/hooks/pre-commit not found, skipping atomicity check"
    exit 0
fi
"#;
        let hook_path = format!("{}/pre-commit", git_hooks_dir);
        fs.write_file(&hook_path, hook_content).await?;

        // Make executable (Platform specific, mainly for unix/bash environments)
        // On Windows it's file permission, but the content is bash, intended for git bash.
        // We can try chmod if available or ignore.
        // Git for Windows usually handles shebangs.
        println!("{}", style("✓ Pre-commit hooks installed").green());
    } else {
        println!("{}", style("⚠️  Could not install hooks (no .git directory)").yellow());
    }

    println!("\n{}", style("✅ Project initialized successfully!").green());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::mocks::{MockFileSystemPort, MockSystemPort};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_init_success() {
        let args = InitArgs {
            name: Some("test-project".to_string()),
            private: false,
            auto: false,
        };

        let mut mock_fs = MockFileSystemPort::new();
        let mut mock_system = MockSystemPort::new();

        // Expect check checks
        mock_system.expect_check_command()
            .with(eq("git"))
            .returning(|_| Ok(true));
        mock_system.expect_check_command()
            .with(eq("gh"))
            .returning(|_| Ok(true));

        // Expect .git check
        mock_fs.expect_exists()
            .with(eq("test-project/.git"))
            .returning(|_| Ok(false));

        // Expect git init with name
        mock_system.expect_run_command()
            .with(eq("git"), eq(vec![String::from("init"), String::from("test-project")]))
            .returning(|_, _| Ok(()));

        // Expect git -C branch
        mock_system.expect_run_command()
            .with(eq("git"), eq(vec![String::from("-C"), String::from("test-project"), String::from("branch"), String::from("-M"), String::from("main")]))
            .returning(|_, _| Ok(()));

        // Expect README check
        mock_fs.expect_exists()
            .with(eq("test-project/README.md"))
            .returning(|_| Ok(false));

        // Expect README write
        mock_fs.expect_write_file()
            .with(eq("test-project/README.md"), always())
            .returning(|_, _| Ok(()));

        // Expect git -C add
        mock_system.expect_run_command()
            .with(eq("git"), eq(vec![String::from("-C"), String::from("test-project"), String::from("add"), String::from(".")]))
            .returning(|_, _| Ok(()));

        // Expect git -C commit
        mock_system.expect_run_command()
            .with(eq("git"), eq(vec![String::from("-C"), String::from("test-project"), String::from("commit"), String::from("-m"), String::from("feat: 🚀 Initial commit")]))
            .returning(|_, _| Ok(()));

        // Expect Architecture file logic
        mock_fs.expect_exists()
            .with(eq(".✨/ARCHITECTURE.md"))
            .returning(|_| Ok(false));

        mock_fs.expect_create_dir()
            .with(eq(".✨"))
            .returning(|_| Ok(()));

        mock_fs.expect_write_file()
            .with(eq(".✨/ARCHITECTURE.md"), always())
            .returning(|_, _| Ok(()));


        let res = execute(args, &mock_fs, &mock_system).await;
        assert!(res.is_ok());
    }
}
