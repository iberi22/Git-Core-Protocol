use clap::Args;
use gc_core::ports::{SystemPort, Result, CoreError};
use serde::{Serialize, Deserialize};

use chrono::Datelike;
use sha2::{Sha256, Digest};

#[derive(Args, Debug)]
pub struct TelemetryArgs {
    #[arg(long)]
    pub dry_run: bool,

    #[arg(long)]
    pub internal: bool,

    #[arg(long)]
    pub anonymous: Option<bool>,

    #[arg(long)]
    pub include_patterns: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metrics {
    schema_version: String,
    submission_method: String,
    project_id: String,
    anonymous: bool,
    timestamp: String,
    week: i32,
    year: i32,
    protocol_version: String,
    order1: Order1Metrics,
    order2: Order2Metrics,
    order3: Order3Metrics,
    #[serde(skip_serializing_if = "Option::is_none")]
    patterns: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Order1Metrics {
    issues_open: usize,
    issues_closed_total: usize,
    prs_open: usize,
    prs_merged_total: usize,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Order2Metrics {
    agent_state_usage_pct: f64,
    atomic_commit_ratio: f64,
    sample_size: usize,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Order3Metrics {
    friction_reports: usize,
    evolution_proposals: usize,
}

const OFFICIAL_REPO_OWNER: &str = "iberi22";
const OFFICIAL_REPO_NAME: &str = "Git-Core-Protocol";
const INTERNAL_LABEL: &str = "telemetry-internal";

pub async fn execute(args: TelemetryArgs, system: &impl SystemPort) -> Result<()> {
    let mode = if args.internal { "Internal (Issues)" } else { "Public (Discussions)" };
    println!("📡 Git-Core Protocol - Federated Telemetry System v2.1 (Rust)");
    println!("   Mode: {}", mode);
    println!("   Destination: github.com/{}/{}", OFFICIAL_REPO_OWNER, OFFICIAL_REPO_NAME);

    let anonymous = args.anonymous.unwrap_or(!args.internal);

    // 1. Collect Metrics
    println!("\n📊 Collecting local metrics...");

    let now = chrono::Utc::now();
    let timestamp = now.to_rfc3339();
    let iso_week = now.iso_week();
    let week = iso_week.week() as i32;
    let year = iso_week.year();

    // Project ID
    let repo_url_out = system.run_command_output("git", &["config".to_string(), "--get".to_string(), "remote.origin.url".to_string()]).await.unwrap_or_default();
    let repo_name_raw = repo_url_out.trim();
    let repo_name = if repo_name_raw.is_empty() {
        "unknown".to_string()
    } else {
        let parts: Vec<&str> = repo_name_raw.split(&['/', ':'][..]).collect();
        let name = parts.last().unwrap_or(&"unknown").trim_end_matches(".git");
        if parts.len() >= 2 {
             let owner = parts[parts.len()-2];
             format!("{}/{}", owner, name)
        } else {
            name.to_string()
        }
    };

    let project_id = if anonymous {
        let mut hasher = Sha256::new();
        hasher.update(repo_name.as_bytes());
        let result = hasher.finalize();
        let hash_str = hex::encode(result);
        format!("anon-{}", &hash_str[0..8])
    } else {
        repo_name.to_string()
    };

    println!("   Project ID: {}", project_id);

    let mut metrics = Metrics {
        schema_version: "2.1".to_string(),
        submission_method: if args.internal { "issue".to_string() } else { "discussion".to_string() },
        project_id: project_id.clone(),
        anonymous,
        timestamp,
        week,
        year,
        protocol_version: "2.1".to_string(),
        order1: Order1Metrics::default(),
        order2: Order2Metrics::default(),
        order3: Order3Metrics::default(),
        patterns: None,
    };

    match collect_order1(system).await {
        Ok(m) => {
            metrics.order1 = m;
            println!("   ✓ Order 1 metrics collected");
        },
        Err(e) => eprintln!("   Could not collect Order 1 metrics: {}", e),
    }

    match collect_order2(system).await {
        Ok(m) => {
            metrics.order2 = m;
            println!("   ✓ Order 2 metrics collected");
        },
        Err(e) => eprintln!("   Could not collect Order 2 metrics: {}", e),
    }

    match collect_order3(system).await {
        Ok(m) => {
            metrics.order3 = m;
            println!("   ✓ Order 3 metrics collected");
        },
        Err(e) => eprintln!("   Could not collect Order 3 metrics: {}", e),
    }

    if args.include_patterns {
        let mut patterns = Vec::new();
        if metrics.order2.agent_state_usage_pct < 50.0 {
            patterns.push("low_agent_state_adoption".to_string());
        }
        if metrics.order2.atomic_commit_ratio < 70.0 {
            patterns.push("low_atomic_commit_ratio".to_string());
        }
        if metrics.order3.friction_reports > 5 {
            patterns.push("high_friction".to_string());
        }
        metrics.patterns = Some(patterns);
    }

    // 2. Generate Payload
    let telemetry_json = serde_json::to_string_pretty(&metrics).unwrap();
    println!("\n📄 Generated telemetry:");
    println!("{}", telemetry_json);

    let submission_title = if args.internal {
        format!("[Telemetry-Internal] {} - Week {} ({})", project_id, week, year)
    } else {
        format!("📊 {} - Week {} ({})", project_id, week, year)
    };

    if args.dry_run {
        let target_type = if args.internal { "Issue" } else { "Discussion" };
        println!("\n🔍 DRY RUN - No {} will be created", target_type);
        println!("   Would create {}: '{}'", target_type, submission_title);
        if args.internal {
            println!("   Label: {}", INTERNAL_LABEL);
        }
        return Ok(());
    }

    // 3. Submit
    if args.internal {
        submit_internal(&submission_title, &metrics, system).await?;
    } else {
         submit_public(&submission_title, &metrics, system).await?;
    }

    Ok(())
}

async fn get_gh_count(system: &impl SystemPort, args: &[&str]) -> Result<usize> {
    let args_vec = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let output = system.run_command_output("gh", &args_vec).await?;
    let json: serde_json::Value = serde_json::from_str(&output)
        .map_err(|e| CoreError::System(format!("JSON Parse Error: {}", e)))?;
    if let Some(arr) = json.as_array() {
         Ok(arr.len())
    } else {
         Ok(0)
    }
}

async fn collect_order1(system: &impl SystemPort) -> Result<Order1Metrics> {
    let issues_open = get_gh_count(system, &["issue", "list", "--state", "open", "--json", "number"]).await?;
    let issues_closed = get_gh_count(system, &["issue", "list", "--state", "closed", "--limit", "100", "--json", "number"]).await?;
    let prs_open = get_gh_count(system, &["pr", "list", "--state", "open", "--json", "number"]).await?;
    let prs_merged = get_gh_count(system, &["pr", "list", "--state", "merged", "--limit", "100", "--json", "number"]).await?;

    Ok(Order1Metrics {
        issues_open,
        issues_closed_total: issues_closed,
        prs_open,
        prs_merged_total: prs_merged,
    })
}

async fn collect_order2(system: &impl SystemPort) -> Result<Order2Metrics> {
    // 1. Agent State Usage
    let args_vec = ["issue", "list", "--limit", "10", "--json", "number"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let output = system.run_command_output("gh", &args_vec).await?;
    let issues: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap_or_default();

    let mut agent_state_count = 0;
    for issue in &issues {
        if let Some(num) = issue["number"].as_u64() {
             let args_view = ["issue".to_string(), "view".to_string(), num.to_string(), "--json".to_string(), "body".to_string()];
             let args_view_vec = args_view.iter().map(|s| s.to_string()).collect::<Vec<_>>();
             let body_json = system.run_command_output("gh", &args_view_vec).await?;
             let body_obj: serde_json::Value = serde_json::from_str(&body_json).unwrap_or_default();
             if let Some(body) = body_obj["body"].as_str() {
                 if body.contains("<agent-state>") {
                     agent_state_count += 1;
                 }
             }
        }
    }

    let usage_pct = if !issues.is_empty() {
        (agent_state_count as f64 / issues.len() as f64) * 100.0
    } else {
        0.0
    };

    // 2. Atomic Commits
    let log_args = ["log", "--oneline", "-50"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let log_out = system.run_command_output("git", &log_args).await?;
    let lines: Vec<&str> = log_out.lines().collect();
    let total_commits = lines.len();

    let atomic_regex = regex::Regex::new(r"^[a-f0-9]+ (feat|fix|docs|style|refactor|test|chore)\(").unwrap();
    let atomic_commits = lines.iter().filter(|l| atomic_regex.is_match(l)).count();

    let atomic_ratio = if total_commits > 0 {
         (atomic_commits as f64 / total_commits as f64) * 100.0
    } else {
        0.0
    };

    Ok(Order2Metrics {
        agent_state_usage_pct: (usage_pct * 10.0).round() / 10.0,
        atomic_commit_ratio: (atomic_ratio * 10.0).round() / 10.0,
        sample_size: issues.len(),
    })
}

async fn collect_order3(system: &impl SystemPort) -> Result<Order3Metrics> {
    // Friction
    let args_friction = ["issue".to_string(), "list".to_string(), "--label".to_string(), "friction".to_string(), "--state".to_string(), "all".to_string(), "--json".to_string(), "number".to_string()];
    let args_vec_f = args_friction.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let output_f = system.run_command_output("gh", &args_vec_f).await?;
    let json_f: serde_json::Value = serde_json::from_str(&output_f).unwrap_or(serde_json::Value::Array(vec![]));
    let friction = json_f.as_array().map(|a| a.len()).unwrap_or(0);

    // Evolution
    let args_evolution = ["issue".to_string(), "list".to_string(), "--label".to_string(), "evolution".to_string(), "--state".to_string(), "all".to_string(), "--json".to_string(), "number".to_string()];
    let args_vec_e = args_evolution.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let output_e = system.run_command_output("gh", &args_vec_e).await?;
    let json_e: serde_json::Value = serde_json::from_str(&output_e).unwrap_or(serde_json::Value::Array(vec![]));
    let evolution = json_e.as_array().map(|a| a.len()).unwrap_or(0);

    Ok(Order3Metrics {
        friction_reports: friction,
        evolution_proposals: evolution,
    })
}

async fn submit_internal(title: &str, metrics: &Metrics, system: &impl SystemPort) -> Result<()> {
    println!("\n🔧 Creating Issue (Internal Mode)...");
    let metrics_json = serde_json::to_string_pretty(metrics).unwrap();
    let body = format!(r#"## 📡 Internal Telemetry Submission

**Project:** `{}`
**Week:** {} ({})
**Protocol Version:** {}
**Mode:** Internal (dogfooding)

### Metrics

```json
{}
```

---
*Auto-generated by Git-Core Protocol Telemetry System v2.1 (Rust)*"#,
        metrics.project_id, metrics.week, metrics.year, metrics.protocol_version, metrics_json);

    let args = ["issue".to_string(), "create".to_string(),
        "--repo".to_string(), format!("{}/{}", OFFICIAL_REPO_OWNER, OFFICIAL_REPO_NAME),
        "--title".to_string(), title.to_string(),
        "--body".to_string(), body.to_string(),
        "--label".to_string(), INTERNAL_LABEL.to_string()
    ];
    let args_vec = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    system.run_command("gh", &args_vec).await?;

    println!("\n✅ Internal telemetry submitted!");
    Ok(())
}

async fn submit_public(title: &str, metrics: &Metrics, system: &impl SystemPort) -> Result<()> {
    println!("\n🔍 Getting repository info (Public Mode)...");

    let query = format!(r#"query {{
  repository(owner: "{}", name: "{}") {{
    id
    discussionCategories(first: 20) {{
      nodes {{
        id
        name
        slug
      }}
    }}
  }}
}}"#, OFFICIAL_REPO_OWNER, OFFICIAL_REPO_NAME);

    let query_arg = format!("query={}", query);
    let args = ["api".to_string(), "graphql".to_string(), "-f".to_string(), query_arg];
    let args_vec = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let repo_json = system.run_command_output("gh", &args_vec).await?;

    let repo_data: serde_json::Value = serde_json::from_str(&repo_json).map_err(|e| CoreError::System(format!("GraphQL Parse Error: {}", e)))?;

    let repo_id = repo_data["data"]["repository"]["id"].as_str().ok_or_else(|| CoreError::System("Repo ID not found".into()))?;

    let categories = repo_data["data"]["repository"]["discussionCategories"]["nodes"].as_array().ok_or_else(|| CoreError::System("Categories not found".into()))?;

    let mut category_id = None;
    let mut category_name = "Unknown";

    for cat in categories {
        let name = cat["name"].as_str().unwrap_or("");
        let slug = cat["slug"].as_str().unwrap_or("");
        if name.contains("Telemetry") || slug.contains("telemetry") {
            category_id = cat["id"].as_str();
            category_name = name;
            break;
        }
    }

    if category_id.is_none() {
        for cat in categories {
             if cat["slug"].as_str().unwrap_or("") == "general" {
                 category_id = cat["id"].as_str();
                 category_name = cat["name"].as_str().unwrap_or("General");
                 break;
             }
        }
    }

    let category_id = category_id.ok_or_else(|| CoreError::System("No suitable discussion category found".into()))?;

    println!("   Repository ID: {}", repo_id);
    println!("   Category: {} ({})", category_name, category_id);

    println!("\n🚀 Creating Discussion (Public Mode)...");

    let metrics_json = serde_json::to_string_pretty(metrics).unwrap();
    let body = format!(r#"## 📡 Telemetry Submission

**Project ID:** `{}`
**Week:** {} ({})
**Protocol Version:** {}

### Metrics

```json
{}
```

---
*Auto-generated by Git-Core Protocol Telemetry System v2.1 (Rust)*"#,
        metrics.project_id, metrics.week, metrics.year, metrics.protocol_version, metrics_json);

    let escaped_body = body.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");

    let mutation = format!(r#"mutation {{
  createDiscussion(input: {{
    repositoryId: "{}"
    categoryId: "{}"
    title: "{}"
    body: "{}"
  }}) {{
    discussion {{
      url
    }}
  }}
}}"#, repo_id, category_id, title, escaped_body);

    let mut_arg = format!("query={}", mutation);
    let mut_args = ["api".to_string(), "graphql".to_string(), "-f".to_string(), mut_arg];
    let mut_args_vec = mut_args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let result_json = system.run_command_output("gh", &mut_args_vec).await?;

    let result: serde_json::Value = serde_json::from_str(&result_json).map_err(|_e| CoreError::System("Failed to parse mutation response".into()))?;

    if let Some(url) = result["data"]["createDiscussion"]["discussion"]["url"].as_str() {
        println!("\n✅ Telemetry submitted successfully!");
        println!("   Discussion: {}", url);
    } else {
         return Err(CoreError::System(format!("Failed to create discussion: {}", result_json)));
    }

    Ok(())
}
