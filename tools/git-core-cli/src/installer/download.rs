//! Download functionality - Fetch protocol from GitHub

use anyhow::Result;
use std::path::PathBuf;
use tempfile::TempDir;

use crate::config::{GITHUB_OWNER, GITHUB_REPO, RAW_URL};
use crate::utils::create_spinner;

/// Fetch the protocol from GitHub
pub async fn fetch_protocol(version: Option<&str>) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;

    // Clone repository
    let url = format!("https://github.com/{}/{}", GITHUB_OWNER, GITHUB_REPO);
    let branch = version.unwrap_or("main");

    let output = std::process::Command::new("git")
        .args([
            "clone",
            "--depth", "1",
            "--branch", branch,
            &url,
            temp_dir.path().to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to clone repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Remove .git directory
    let git_dir = temp_dir.path().join(".git");
    if git_dir.exists() {
        std::fs::remove_dir_all(&git_dir)?;
    }

    Ok(temp_dir)
}

/// Get the latest version from GitHub
pub async fn get_latest_version() -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", RAW_URL, ".git-core-protocol-version");

    let response = client
        .get(&url)
        .header("User-Agent", "git-core-cli")
        .send()
        .await?;

    if response.status().is_success() {
        let version = response.text().await?.trim().to_string();
        Ok(version)
    } else {
        // Try to get from GitHub releases
        get_latest_release_version().await
    }
}

/// Get version from GitHub releases API
async fn get_latest_release_version() -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        GITHUB_OWNER, GITHUB_REPO
    );

    let response = client
        .get(&url)
        .header("User-Agent", "git-core-cli")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    if response.status().is_success() {
        let release: serde_json::Value = response.json().await?;
        if let Some(tag) = release["tag_name"].as_str() {
            return Ok(tag.trim_start_matches('v').to_string());
        }
    }

    Ok("unknown".to_string())
}

/// Download a specific file from the repository
pub async fn download_file(path: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", RAW_URL, path);

    let response = client
        .get(&url)
        .header("User-Agent", "git-core-cli")
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        anyhow::bail!("Failed to download {}: {}", path, response.status());
    }
}
