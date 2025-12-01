use anyhow::Result;
use crate::search::SearchResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct Insight {
    pub dependency_name: String,
    pub version: String,
    pub analysis: String,
}

// ============== GEMINI 3 PRO STRUCTS ==============
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContentResponse>,
}

#[derive(Deserialize)]
struct GeminiContentResponse {
    parts: Option<Vec<GeminiPartResponse>>,
}

#[derive(Deserialize)]
struct GeminiPartResponse {
    text: String,
}

// ============== RATE LIMITER CONFIG ==============
// Gemini Free Tier: 15 requests/min, 1500 requests/day
// Strategy:
//   - Batch 10 deps per call (reduces 50 deps → 5 calls)
//   - 5 seconds between requests (12 req/min, safe margin)
//   - Max ~100 calls/workflow = well under 1500/day limit
const RATE_LIMIT_DELAY_MS: u64 = 5000;
const BATCH_SIZE: usize = 10;

pub async fn analyze_findings(results: Vec<SearchResult>) -> Result<Vec<Insight>> {
    let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_default();

    if gemini_key.is_empty() {
        println!("⚠️ GEMINI_API_KEY not found. Skipping intelligence analysis.");
        return Ok(Vec::new());
    }

    let client = Client::new();
    let mut insights = Vec::new();

    // Filter only dependencies with issues (save API calls)
    let relevant: Vec<_> = results.into_iter().filter(|r| !r.issues.is_empty()).collect();
    let total = relevant.len();

    if total == 0 {
        println!("✅ No issues found in dependencies. Skipping analysis.");
        return Ok(Vec::new());
    }

    println!("🧠 Analyzing {} dependencies with issues using Gemini 3 Pro...", total);

    // AGGRESSIVE BATCHING: 10 deps per call to minimize API usage
    let batches: Vec<Vec<&SearchResult>> = relevant.chunks(BATCH_SIZE).map(|c| c.iter().collect()).collect();
    let total_batches = batches.len();

    println!("📊 Strategy: {} batches of up to {} deps each", total_batches, BATCH_SIZE);
    println!("📊 Estimated API calls: {} (Free tier: 1500/day)", total_batches);

    for (batch_idx, batch) in batches.iter().enumerate() {
        println!("\n📦 Batch {}/{} ({} deps)...", batch_idx + 1, total_batches, batch.len());

        // Build combined prompt for the batch
        let batch_prompt = build_batch_prompt(&batch);

        // Call Gemini 3 Pro
        println!("  🔷 Calling Gemini 3 Pro...");
        let result = call_gemini(&client, &gemini_key, &batch_prompt).await;

        match &result {
            Ok(_) => println!("  ✅ Success!"),
            Err(e) => println!("  ⚠️ Error: {}", e),
        }

        // Store results for each dep in batch
        let analysis_text = result.unwrap_or_else(|e| format!("Analysis failed: {}", e));

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
        "You are a Senior Software Engineer analyzing GitHub issues for multiple libraries.\n\
        For EACH library below, provide:\n\
        1. **Known Anomalies**: Bugs or quirks in THIS SPECIFIC version\n\
        2. **Anti-patterns to Avoid**: Common mistakes found in issues\n\
        3. **Intelligent Pattern**: The recommended way to use this version safely\n\n\
        Be concise but specific. Focus on actionable insights.\n\n"
    );

    for (i, res) in batch.iter().enumerate() {
        prompt.push_str(&format!(
            "---\n## Library {}: {} (version {})\n### Issues Found:\n",
            i + 1, res.dependency.name, res.dependency.version
        ));
        for issue in &res.issues {
            prompt.push_str(&format!("- [{}] {}\n", issue.state, issue.title));
        }
        prompt.push('\n');
    }

    prompt
}

async fn call_gemini(client: &Client, api_key: &str, prompt: &str) -> Result<String> {
    let request_body = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![GeminiPart { text: prompt.to_string() }],
        }],
    };

    // Use Gemini 3 Pro Preview (latest and most intelligent)
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-pro-preview:generateContent?key={}",
        api_key
    );

    let resp = client.post(&url)
        .json(&request_body)
        .timeout(Duration::from_secs(120)) // Longer timeout for complex analysis
        .send()
        .await?;

    if resp.status().is_success() {
        let gemini_resp: GeminiResponse = resp.json().await?;
        if let Some(candidates) = gemini_resp.candidates {
            if let Some(first) = candidates.first() {
                if let Some(content) = &first.content {
                    if let Some(parts) = &content.parts {
                        if let Some(text_part) = parts.first() {
                            return Ok(text_part.text.clone());
                        }
                    }
                }
            }
        }
    } else {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        println!("  ⚠️ Gemini error {}: {}", status, body);
    }
    Err(anyhow::anyhow!("Gemini API error"))
}
