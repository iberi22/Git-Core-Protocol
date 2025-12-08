//! Integration tests for Issue Syncer
//!
//! Tests the complete sync workflow with realistic scenarios.

use issue_syncer::{
    github::GitHubClient,
    mapping::IssueMapping,
    syncer::IssueSyncer,
};
use octocrab::Octocrab;
use std::fs;
use tempfile::TempDir;

fn create_test_issue_file(dir: &std::path::Path, filename: &str, title: &str, labels: &[&str]) {
    let labels_yaml = labels
        .iter()
        .map(|l| format!("  - {}", l))
        .collect::<Vec<_>>()
        .join("\n");

    let content = format!(
        r#"---
title: "{}"
labels:
{}
assignees: []
---

This is the issue body for {}.
"#,
        title, labels_yaml, filename
    );

    fs::write(dir.join(filename), content).unwrap();
}

async fn create_test_syncer(temp_dir: &TempDir) -> IssueSyncer {
    let issues_dir = temp_dir.path().join("issues");
    fs::create_dir(&issues_dir).unwrap();

    let mapping_file = issues_dir.join(".issue-mapping.json");

    let client = Octocrab::builder().build().unwrap();
    let github = GitHubClient::new(client, "owner".to_string(), "repo".to_string());

    IssueSyncer::new(github, issues_dir, mapping_file)
        .unwrap()
        .with_dry_run(true)
}

#[tokio::test]
async fn test_sync_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let mut syncer = create_test_syncer(&temp_dir).await;

    let report = syncer.sync_all().await.unwrap();

    assert_eq!(report.created, 0);
    assert_eq!(report.updated, 0);
    assert_eq!(report.deleted, 0);
}

#[tokio::test]
async fn test_push_new_issues() {
    let temp_dir = TempDir::new().unwrap();
    let mut syncer = create_test_syncer(&temp_dir).await;

    // Create test issue files
    let issues_dir = temp_dir.path().join("issues");
    create_test_issue_file(&issues_dir, "FEAT_test1.md", "Feature 1", &["enhancement"]);
    create_test_issue_file(&issues_dir, "BUG_test2.md", "Bug Fix", &["bug", "urgent"]);

    let report = syncer.push().await.unwrap();

    assert_eq!(report.created, 2);
    assert_eq!(report.updated, 0);
}

#[tokio::test]
async fn test_push_with_existing_mapping() {
    let temp_dir = TempDir::new().unwrap();
    let issues_dir = temp_dir.path().join("issues");
    fs::create_dir(&issues_dir).unwrap();

    // Create mapping file
    let mapping_file = issues_dir.join(".issue-mapping.json");
    let mut mapping = IssueMapping::default();
    mapping.add("FEAT_test1.md".to_string(), 42);
    mapping.save(&mapping_file).unwrap();

    // Create issue file
    create_test_issue_file(&issues_dir, "FEAT_test1.md", "Updated Feature", &["enhancement"]);

    let client = Octocrab::builder().build().unwrap();
    let github = GitHubClient::new(client, "owner".to_string(), "repo".to_string());

    let mut syncer = IssueSyncer::new(github, issues_dir, mapping_file)
        .unwrap()
        .with_dry_run(true);

    let report = syncer.push().await.unwrap();

    assert_eq!(report.created, 0);
    assert_eq!(report.updated, 1);
}

#[tokio::test]
async fn test_scan_multiple_issue_types() {
    let temp_dir = TempDir::new().unwrap();
    let mut syncer = create_test_syncer(&temp_dir).await;

    let issues_dir = temp_dir.path().join("issues");
    create_test_issue_file(&issues_dir, "FEAT_feature.md", "Feature", &["enhancement"]);
    create_test_issue_file(&issues_dir, "BUG_bugfix.md", "Bug", &["bug"]);
    create_test_issue_file(&issues_dir, "DOCS_documentation.md", "Docs", &["documentation"]);
    create_test_issue_file(&issues_dir, "REFACTOR_cleanup.md", "Refactor", &["refactor"]);

    let report = syncer.push().await.unwrap();

    assert_eq!(report.created, 4);
}

#[tokio::test]
async fn test_mapping_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let mapping_file = temp_dir.path().join(".issue-mapping.json");

    // Create and save mapping
    let mut mapping1 = IssueMapping::default();
    mapping1.add("FEAT_test.md".to_string(), 42);
    mapping1.add("BUG_error.md".to_string(), 43);
    mapping1.save(&mapping_file).unwrap();

    // Load mapping
    let mapping2 = IssueMapping::load(&mapping_file).unwrap();

    assert_eq!(mapping2.get_issue("FEAT_test.md"), Some(42));
    assert_eq!(mapping2.get_issue("BUG_error.md"), Some(43));
    assert_eq!(mapping2.len(), 2);
}

#[tokio::test]
async fn test_bidirectional_mapping_lookup() {
    let mut mapping = IssueMapping::default();
    mapping.add("FEAT_auth.md".to_string(), 100);
    mapping.add("BUG_crash.md".to_string(), 101);

    // Forward lookup (file -> issue)
    assert_eq!(mapping.get_issue("FEAT_auth.md"), Some(100));
    assert_eq!(mapping.get_issue("BUG_crash.md"), Some(101));

    // Reverse lookup (issue -> file)
    assert_eq!(mapping.get_file(100), Some("FEAT_auth.md".to_string()));
    assert_eq!(mapping.get_file(101), Some("BUG_crash.md".to_string()));
}

#[tokio::test]
async fn test_mapping_removal() {
    let mut mapping = IssueMapping::default();
    mapping.add("FEAT_test.md".to_string(), 42);

    // Remove by file
    let removed = mapping.remove_by_file("FEAT_test.md");
    assert_eq!(removed, Some(42));
    assert!(!mapping.contains_file("FEAT_test.md"));

    // Add back and remove by issue
    mapping.add("FEAT_test.md".to_string(), 42);
    let removed = mapping.remove_by_issue(42);
    assert_eq!(removed, Some("FEAT_test.md".to_string()));
    assert!(!mapping.contains_issue(42));
}

#[tokio::test]
async fn test_issue_file_with_complex_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let issues_dir = temp_dir.path().join("issues");
    fs::create_dir(&issues_dir).unwrap();

    let content = r#"---
title: "Complex Issue with Special Characters: @mentions, #tags"
labels:
  - enhancement
  - high-priority
  - "special-label"
assignees:
  - user1
  - user2
---

# Issue Description

This is a **complex** issue with:
- Multiple lines
- Markdown formatting
- Code blocks

```rust
fn main() {
    println!("Hello");
}
```
"#;

    fs::write(issues_dir.join("COMPLEX.md"), content).unwrap();

    let client = Octocrab::builder().build().unwrap();
    let github = GitHubClient::new(client, "owner".to_string(), "repo".to_string());

    let mut syncer = IssueSyncer::new(
        github,
        issues_dir,
        temp_dir.path().join(".mapping.json"),
    )
    .unwrap()
    .with_dry_run(true);

    let report = syncer.push().await.unwrap();
    assert_eq!(report.created, 1);
    assert_eq!(report.errors, 0);
}

#[tokio::test]
async fn test_skip_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    let mut syncer = create_test_syncer(&temp_dir).await;

    let issues_dir = temp_dir.path().join("issues");

    // Create normal and hidden files
    create_test_issue_file(&issues_dir, "FEAT_visible.md", "Visible", &["enhancement"]);
    fs::write(
        issues_dir.join(".issue-mapping.json"),
        "{}",
    )
    .unwrap();
    fs::write(issues_dir.join(".hidden.md"), "hidden content").unwrap();

    let report = syncer.push().await.unwrap();

    // Should only process the visible file
    assert_eq!(report.created, 1);
}
