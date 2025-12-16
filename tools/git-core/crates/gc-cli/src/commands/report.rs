use gc_core::ports::{GitHubPort, SystemPort};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ReportCmd {
    /// Generate a full report (Gemini + Copilot)
    Full {
        /// Pull Request Number
        #[arg(long)]
        pr: Option<u64>,
    },
    /// Generate only Gemini report
    Gemini {
        /// Pull Request Number
        #[arg(long)]
        pr: Option<u64>,
    },
    /// Generate only Copilot report
    Copilot {
        /// Pull Request Number
        #[arg(long)]
        pr: Option<u64>,
        /// Model to use
        #[arg(long, default_value = "claude-sonnet-4.5")]
        model: String,
    },
}
use console::style;

pub async fn execute(
    cmd: ReportCmd,
    system: &impl SystemPort,
    github: &impl GitHubPort,
) -> color_eyre::Result<()> {
    // 1. Resolve PR Number
    // Logic: If provided, use it. If not, try to get from `gh pr view`.
    // NOTE: This assumes `gh` is installed for context resolution if arg not provided.
    // Ideally we'd use GitPort to find branch and query GH API, but this is faster for migration.

    let (pr_number, report_type, model) = match cmd {
        ReportCmd::Full { pr } => (pr, "full".to_string(), "claude-sonnet-4.5".to_string()),
        ReportCmd::Gemini { pr } => (pr, "gemini".to_string(), "".to_string()),
        ReportCmd::Copilot { pr, model } => (pr, "copilot".to_string(), model),
    };

    let pr_number = if let Some(n) = pr_number {
        n
    } else {
        // Try to resolve current PR
        let output = system.run_command_output("gh", &vec![String::from("pr"), String::from("view"), String::from("--json"), String::from("number")]).await;
        match output {
            Ok(json) => {
                // simple parse: {"number": 123}
                // avoid full serde here for just one int if possible, or use serde_json
                // But we have serde dep in core, let's just do simple regex or string find
                // "number":123
                // This is brittle. Better use serde if available or robust regex.
                // Assuming json format: { ... "number":123 ... }
                // Let's rely on serde_json::from_str
                let v: serde_json::Value = serde_json::from_str(&json)?;
                v["number"].as_u64().ok_or_else(|| color_eyre::eyre::eyre!("Could not parse PR number from gh output"))?
            }
            Err(_) => {
                color_eyre::eyre::bail!("Could not resolve PR number. Please provide --pr <NUMBER>");
            }
        }
    };

    println!("{}", style(format!("🤖 Analyzing PR #{}...", pr_number)).cyan());

    // 2. Fetch PR Data (Title, Body, Diff)
    // We hardcode owner/repo for now or need to detect it.
    // `gh` knows the context. Octocrab needs it explicit.
    // Detection via `gh repo view --json owner,name`?
    // Let's do that for robustness.
    // Let's do that for robustness.
    let repo_json = system.run_command_output("gh", &vec![String::from("repo"), String::from("view"), String::from("--json"), String::from("owner,name")]).await?;
    let repo_val: serde_json::Value = serde_json::from_str(&repo_json)?;
    let owner = repo_val["owner"]["login"].as_str().unwrap_or("iberi22");
    let repo = repo_val["name"].as_str().unwrap_or("agents-flows-recipes"); // fallback unsafe

    let diff = github.get_pr_diff(owner, repo, pr_number).await?;

    // We assume we can fetch issue details to get title/body using octocrab issues or pulls
    // GitHubPort doesn't have get_pr_details yet.
    // Wait, I should have added `get_pr_details` to port.
    // Doing it raw here or just passing empty context?
    // The prompts need Title + Body.
    // MVP shortcut: Just analyze the Diff if Title/Body fetching is complex without update.
    // BUT quality depends on context.
    // Let's just use "PR Analysis" generic title if we don't update port now.
    // OR: use `gh pr view --json title,body` since we rely on `gh` anyway for context.

    let pr_json = system.run_command_output("gh", &vec![String::from("pr"), String::from("view"), pr_number.to_string(), String::from("--json"), String::from("title,body")]).await?;
    let pr_val: serde_json::Value = serde_json::from_str(&pr_json)?;
    let title = pr_val["title"].as_str().unwrap_or("Unknown Title");
    let body = pr_val["body"].as_str().unwrap_or("");

    // 3. Generate Reports
    let mut final_report = String::new();
    final_report.push_str(&format!("## 🤖 AI Analysis Report (PR #{})\n\n", pr_number));
    final_report.push_str("> Generado por `gc report`\n\n");

    if report_type == "full" || report_type == "gemini" {
        println!("{}", style("🔮 Generating Gemini Analysis...").magenta());
        let prompt = format!(
            "Analiza este PR:\n\nTitulo: {}\nDesc:\n{}\n\nDiff:\n{}\n\nGenera reporte tecnico en Español: Resumen, Impacto, Riesgos.",
            title, body, diff
        );
        match system.run_command_output("gemini", &vec![String::from("-p"), prompt, String::from("-o"), String::from("text")]).await {
            Ok(out) => {
                final_report.push_str("### 🔮 Gemini Analysis\n\n");
                final_report.push_str(&out);
                final_report.push_str("\n\n");
            },
            Err(e) => eprintln!("Gemini failed: {}", e),
        }
    }

    if report_type == "full" || report_type == "copilot" {
        println!("{}", style(format!("🤖 Generating Copilot Analysis ({})", model)).blue());
         let prompt = format!(
            "Analiza este PR:\n\nTitulo: {}\nDesc:\n{}\n\nDiff:\n{}\n\nGenera reporte tecnico en Español.",
            title, body, diff
        );
        // copilot -p <prompt> --model <model> -s --allow-all-tools
         match system.run_command_output("copilot", &vec![String::from("-p"), prompt, String::from("--model"), model.clone(), String::from("-s"), String::from("--allow-all-tools")]).await {
            Ok(out) => {
                final_report.push_str(&format!("### 🤖 Copilot Analysis ({})\n\n", model));
                final_report.push_str(&out);
                final_report.push_str("\n\n");
            },
            Err(e) => eprintln!("Copilot failed: {}", e),
        }
    }

    final_report.push_str("---\n*Generated via Git-Core Protocol*");

    // 4. Post Comment
    println!("{}", style("posting comment...").yellow());
    // github.post_comment(owner, repo, pr_number, &final_report).await?; // This works if GitHubPort works.
    // Or stick to `gh pr comment` for now as MVP since we used `gh` for context anyway.
    // But let's try the native port!

    // We encounter a catch-22: `post_comment` needs `owner` and `repo`.
    // We fetched owner/repo via `gh repo view`.
    // So we can use the Port!

    github.post_comment(owner, repo, pr_number, &final_report).await?;

    println!("{}", style("✅ Report posted successfully!").green());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::mocks::{MockSystemPort, MockGitHubPort};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_report_success() {
        let cmd = ReportCmd::Full { pr: Some(123) };
        let mut mock_system = MockSystemPort::new();
        let mut mock_github = MockGitHubPort::new();

        // 1. Repo Context Mock
        mock_system.expect_run_command_output()
            .with(eq("gh"), eq(vec![String::from("repo"), String::from("view"), String::from("--json"), String::from("owner,name")]))
            .returning(|_, _| Ok(r#"{"owner":{"login":"iberi22"},"name":"agents-flows-recipes"}"#.to_string()));

        // 2. PR Diff Mock
        mock_github.expect_get_pr_diff()
            .with(eq("iberi22"), eq("agents-flows-recipes"), eq(123))
            .returning(|_, _, _| Ok("diff content...".to_string()));

        // 3. PR Title/Body Mock
        mock_system.expect_run_command_output()
            .with(eq("gh"), eq(vec![String::from("pr"), String::from("view"), String::from("123"), String::from("--json"), String::from("title,body")]))
            .returning(|_, _| Ok(r#"{"title":"Fix Bug","body":"Fixed it"}"#.to_string()));

        // 4. Gemini Report Mock
        mock_system.expect_run_command_output()
             .with(eq("gemini"), always()) // Match any prompt args
             .returning(|_, _| Ok("Gemini Analysis Result".to_string()));

        // 5. Copilot Report Mock
        mock_system.expect_run_command_output()
             .with(eq("copilot"), always())
             .returning(|_, _| Ok("Copilot Analysis Result".to_string()));

        // 6. Post Comment Mock
        mock_github.expect_post_comment()
             .with(eq("iberi22"), eq("agents-flows-recipes"), eq(123), always()) // Match any body
             .returning(|_, _, _, _| Ok(()));

        let res = execute(cmd, &mock_system, &mock_github).await;
        assert!(res.is_ok());
    }
}
