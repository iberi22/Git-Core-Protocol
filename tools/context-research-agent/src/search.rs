use anyhow::Result;
use crate::context::Dependency;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use futures::future::join_all;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub dependency: Dependency,
    pub issues: Vec<GitHubIssue>,
    pub release_notes: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubIssue {
    pub title: String,
    pub html_url: String,
    pub body: Option<String>,
    pub state: String,
}

#[derive(Deserialize)]
struct GitHubSearchResponse {
    items: Vec<GitHubIssue>,
}

pub async fn gather_context(deps: &[Dependency]) -> Result<Vec<SearchResult>> {
    let client = Client::new();
    let token = env::var("GITHUB_TOKEN").ok();

    let mut tasks = Vec::new();

    for dep in deps {
        let dep = dep.clone();
        let client = client.clone();
        let token = token.clone();

        tasks.push(tokio::spawn(async move {
            fetch_dependency_context(client, token, dep).await
        }));
    }

    let results = join_all(tasks).await;

    let mut search_results = Vec::new();
    for res in results {
        if let Ok(Ok(data)) = res {
            search_results.push(data);
        }
    }

    Ok(search_results)
}

async fn fetch_dependency_context(client: Client, token: Option<String>, dep: Dependency) -> Result<SearchResult> {
    // 1. Find Repository (Simple search or assumption)
    // For MVP, we search issues globally with the package name and version
    // Query: "repo:owner/name version" or just "package name version bug"

    let query = format!("{} {} is:issue is:closed label:bug", dep.name, dep.version);
    let url = format!("https://api.github.com/search/issues?q={}&sort=updated&order=desc&per_page=5", query);

    let mut req = client.get(&url).header("User-Agent", "Context-Research-Agent");
    if let Some(t) = &token {
        req = req.header("Authorization", format!("Bearer {}", t));
    }

    let resp = req.send().await?;
    let issues = if resp.status().is_success() {
        let search_resp: GitHubSearchResponse = resp.json().await?;
        search_resp.items
    } else {
        Vec::new()
    };

    Ok(SearchResult {
        dependency: dep,
        issues,
        release_notes: None, // TODO: Implement release notes fetching
    })
}
