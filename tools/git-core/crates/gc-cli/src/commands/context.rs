use clap::Subcommand;
use gc_core::ports::{FileSystemPort, GitHubPort};
use console::style;

#[derive(Subcommand, Debug)]
pub enum ContextCmd {
    /// Equip a specific agent role
    Equip {
        role: String,
    },
    /// List available agents
    List,
}

pub async fn execute(
    cmd: ContextCmd,
    fs: &impl FileSystemPort,
    github: &impl GitHubPort,
) -> color_eyre::Result<()> {
    match cmd {
        ContextCmd::List => {
            // MVP: Just cat the index file or something simple
            println!("Available roles (check .✨/AGENT_INDEX.md):");
            // ... implementation skipped for MVP smoothness on 'equip' focus
        }
        ContextCmd::Equip { role } => {
            println!("{}", style(format!("🔍 Searching for role '{}'...", role)).cyan());

            let index_path = ".✨/AGENT_INDEX.md";
            if !fs.exists(index_path).await? {
                color_eyre::eyre::bail!("Index file not found at {}", index_path);
            }

            let content = fs.read_file(index_path).await?;

            // Basic line matching logic (Functional style ideally, but imperative is pragmatic here)
            let mut recipe_path = None;

            for line in content.lines() {
                // Check if line contains role (case insensitive)
                if line.to_lowercase().contains(&role.to_lowercase()) {
                    // Extract path between backticks
                    if let Some(start) = line.find('`') {
                        if let Some(end) = line[start+1..].find('`') {
                             recipe_path = Some(line[start+1..start+1+end].to_string());
                             break;
                        }
                    }
                }
            }

            let recipe_path = match recipe_path {
                Some(p) => p,
                None => color_eyre::eyre::bail!("Role '{}' not found in index.", role),
            };

            println!("{}", style(format!("✅ Found Recipe Path: {}", recipe_path)).green());

            println!("{}", style(format!("⬇️ Downloading from iberi22/agents-flows-recipes...")).cyan());

            let recipe_content = github.get_file_content(
                "iberi22",
                "agents-flows-recipes",
                "main",
                &recipe_path
            ).await?;

            let context_path = ".✨/CURRENT_CONTEXT.md";
            let header = format!(r#"# 🎭 ACTIVE AGENT PERSONA: {}
> GENERATED CONTEXT - DO NOT EDIT MANUALLY
> Loaded via Git-Core CLI

---
"#, role);

            let protocol_skills = r#"
---
## 🛡️ MANDATORY PROTOCOL SKILLS
1. **Token Economy:** Use GitHub Issues for state. No TODO.md.
2. **Architecture First:** Verify against .✨/ARCHITECTURE.md.
3. **Atomic Commits:** One logical change per commit.
"#;

            let final_context = format!("{}{}{}", header, recipe_content, protocol_skills);

            fs.write_file(context_path, &final_context).await?;

            println!("{}", style(format!("✨ Agent Equipped! Context written to {}", context_path)).yellow());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::mocks::{MockFileSystemPort, MockGitHubPort};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_context_equip_success() {
        let cmd = ContextCmd::Equip { role: "Architect".to_string() };
        let mut mock_fs = MockFileSystemPort::new();
        let mut mock_github = MockGitHubPort::new();

        // 1. Check Index Exists
        mock_fs.expect_exists()
            .with(eq(".✨/AGENT_INDEX.md"))
            .returning(|_| Ok(true));

        // 2. Read Index
        let index_content = r#"
# Agent Index
- **Architect**: `roles/architect.md`
"#;
        mock_fs.expect_read_file()
            .with(eq(".✨/AGENT_INDEX.md"))
            .returning(move |_| Ok(index_content.to_string()));

        // 3. GitHub Fetch Recipe
        mock_github.expect_get_file_content()
            .with(eq("iberi22"), eq("agents-flows-recipes"), eq("main"), eq("roles/architect.md"))
            .returning(|_, _, _, _| Ok("# Architect Persona\nYou are an architect.".to_string()));

        // 4. Write Context
        mock_fs.expect_write_file()
            .with(eq(".✨/CURRENT_CONTEXT.md"), always()) // Check content if strict
            .returning(|_, _| Ok(()));

        let res = execute(cmd, &mock_fs, &mock_github).await;
        assert!(res.is_ok());
    }
}
