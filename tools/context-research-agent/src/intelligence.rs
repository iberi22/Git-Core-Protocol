use anyhow::Result;
use crate::search::SearchResult;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Insight {
    pub dependency_name: String,
    pub version: String,
    pub analysis: String,
}

// ============== CONFIGURATION ==============
// Priority: Gemini CLI (local OAuth) > GitHub Models (gh CLI) > No analysis
//
// Gemini CLI: Uses local OAuth2 credentials (no API key needed)
//   - Install: npm install -g @google/gemini-cli
//   - Login: gemini login
//   - Models: gemini-2.5-flash (default), gemini-2.5-pro, gemini-3-pro-preview
//
// GitHub Models: Uses gh CLI with Copilot subscription
//   - Install: gh extension install github/gh-models
//   - Models: meta/llama-3.3-70b-instruct (free tier)

const GEMINI_MODEL: &str = "gemini-2.5-flash"; // Fast, reliable, free tier friendly
const GH_MODEL: &str = "meta/llama-3.3-70b-instruct"; // Fallback model
const RATE_LIMIT_DELAY_MS: u64 = 3000; // 3 seconds between calls
const BATCH_SIZE: usize = 5; // Dependencies per batch

// Store the detected gemini command for reuse
static GEMINI_COMMAND: OnceLock<String> = OnceLock::new();

#[derive(Debug, Clone, PartialEq)]
enum AIProvider {
    GeminiCli,
    GitHubModels,
    None,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    response: String,
}

fn detect_available_provider() -> AIProvider {
    // Check Gemini CLI first (preferred - uses local OAuth)
    // Try multiple ways to find gemini (PATH might vary on Windows/Linux/Mac)
    let gemini_commands = ["gemini", "gemini.cmd", "gemini.exe", "gemini.bat"];
    
    for cmd in gemini_commands {
        let gemini_check = Command::new(cmd)
            .args(["--version"])
            .output();
        
        match gemini_check {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("✅ Gemini CLI v{} detected (cmd: {}) - using local OAuth credentials", version.trim(), cmd);
                // Store the working command for later use
                let _ = GEMINI_COMMAND.set(cmd.to_string());
                return AIProvider::GeminiCli;
            }
            _ => continue,
        }
    }

    // Fallback to GitHub Models
    let gh_check = Command::new("gh")
        .args(["models", "list"])
        .output();
    
    if gh_check.map(|o| o.status.success()).unwrap_or(false) {
        println!("✅ GitHub Models detected - using gh CLI");
        return AIProvider::GitHubModels;
    }

    AIProvider::None
}

pub async fn analyze_findings(results: Vec<SearchResult>) -> Result<Vec<Insight>> {
    let provider = detect_available_provider();

    if provider == AIProvider::None {
        println!("⚠️ No AI provider available. Generating report without analysis.");
        println!("   To enable AI analysis, install ONE of:");
        println!("   1. Gemini CLI: npm install -g @google/gemini-cli && gemini login");
        println!("   2. GitHub Models: gh extension install github/gh-models");
        return Ok(Vec::new());
    }

    let mut insights = Vec::new();

    // Filter only dependencies with issues (save API calls)
    let relevant: Vec<_> = results.into_iter().filter(|r| !r.issues.is_empty()).collect();
    let total = relevant.len();

    if total == 0 {
        println!("✅ No issues found in dependencies. Skipping analysis.");
        return Ok(Vec::new());
    }

    let model_name = match provider {
        AIProvider::GeminiCli => GEMINI_MODEL,
        AIProvider::GitHubModels => GH_MODEL,
        AIProvider::None => unreachable!(),
    };
    println!("🧠 Analyzing {} dependencies using {:?} ({})...", total, provider, model_name);

    // Batch dependencies for analysis
    let batches: Vec<Vec<&SearchResult>> = relevant.chunks(BATCH_SIZE).map(|c| c.iter().collect()).collect();
    let total_batches = batches.len();

    println!("📊 Strategy: {} batches of up to {} deps each", total_batches, BATCH_SIZE);

    for (batch_idx, batch) in batches.iter().enumerate() {
        println!("\n📦 Batch {}/{} ({} deps)...", batch_idx + 1, total_batches, batch.len());

        let batch_prompt = build_batch_prompt(&batch);

        let result = match provider {
            AIProvider::GeminiCli => call_gemini_cli(&batch_prompt).await,
            AIProvider::GitHubModels => call_gh_models(&batch_prompt).await,
            AIProvider::None => unreachable!(),
        };

        match &result {
            Ok(text) => println!("  ✅ Success! ({} chars)", text.len()),
            Err(e) => {
                println!("  ⚠️ Error: {}", e);
                println!("  ℹ️ Continuing without AI analysis for this batch...");
            }
        }

        let analysis_text = result.unwrap_or_else(|_| {
            "AI analysis unavailable for this batch.".to_string()
        });

        for dep in batch {
            insights.push(Insight {
                dependency_name: dep.dependency.name.clone(),
                version: dep.dependency.version.clone(),
                analysis: analysis_text.clone(),
            });
        }

        // Rate limit pause before next batch (skip on last)
        if batch_idx < total_batches - 1 {
            println!("  ⏳ Rate limit pause ({}ms)...", RATE_LIMIT_DELAY_MS);
            sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
        }
    }

    println!("\n✅ Analysis complete! {} insights generated.", insights.len());
    Ok(insights)
}

fn build_batch_prompt(batch: &[&SearchResult]) -> String {
    let mut prompt = String::from(
        "You are a Senior Software Engineer analyzing GitHub issues for multiple libraries. \
        For EACH library below, provide: \
        1. Known Anomalies: Bugs or quirks in THIS SPECIFIC version. \
        2. Anti-patterns to Avoid: Common mistakes found in issues. \
        3. Intelligent Pattern: The recommended way to use this version safely. \
        Be concise but specific. Focus on actionable insights. "
    );

    for (i, res) in batch.iter().enumerate() {
        prompt.push_str(&format!(
            "--- Library {}: {} (version {}) Issues Found: ",
            i + 1, res.dependency.name, res.dependency.version
        ));
        for issue in &res.issues {
            prompt.push_str(&format!("[{}] {}. ", issue.state, issue.title));
        }
    }

    prompt
}

/// Call Gemini CLI (local OAuth - preferred method)
async fn call_gemini_cli(prompt: &str) -> Result<String> {
    // Get the command that was detected during provider detection
    let gemini_cmd = GEMINI_COMMAND.get()
        .map(|s| s.as_str())
        .unwrap_or("gemini");
    
    println!("  🔷 Calling Gemini CLI ({}) via '{}'...", GEMINI_MODEL, gemini_cmd);
    
    // Gemini CLI syntax: gemini -m model -o json "prompt"
    let output = Command::new(gemini_cmd)
        .args([
            "-m", GEMINI_MODEL,
            "-o", "json",
            "--sandbox=false",  // Disable sandbox for non-interactive
            prompt,
        ])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Parse JSON response - response is in "response" field
        let response: GeminiResponse = serde_json::from_str(&stdout)
            .map_err(|e| anyhow::anyhow!("Failed to parse Gemini JSON: {}", e))?;
        
        // Clean up markdown code blocks if present
        let cleaned = response.response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();
        
        if cleaned.is_empty() {
            return Err(anyhow::anyhow!("Empty response from Gemini"));
        }
        
        Ok(cleaned)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("429") || stderr.contains("rate") {
            return Err(anyhow::anyhow!("Rate limit hit. Try again later."));
        }
        if stderr.contains("auth") || stderr.contains("login") {
            return Err(anyhow::anyhow!("Not authenticated. Run: gemini login"));
        }
        Err(anyhow::anyhow!("Gemini CLI error: {}", stderr))
    }
}

/// Call GitHub Models via gh CLI (fallback)
async fn call_gh_models(prompt: &str) -> Result<String> {
    println!("  🔷 Calling GitHub Models ({})...", GH_MODEL);
    
    let output = Command::new("gh")
        .args([
            "models",
            "run",
            GH_MODEL,
            prompt,
            "--max-tokens", "2048",
        ])
        .output()?;

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout).to_string();
        if response.trim().is_empty() {
            return Err(anyhow::anyhow!("Empty response from GitHub Models"));
        }
        Ok(response)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("403") || stderr.contains("no_access") {
            return Err(anyhow::anyhow!("No access to model. Ensure you have Copilot subscription."));
        }
        Err(anyhow::anyhow!("GitHub Models error: {}", stderr))
    }
}
