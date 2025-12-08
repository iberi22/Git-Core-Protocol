//! YAML Frontmatter Parser
//!
//! Extracts GitHub Issue metadata from Markdown files with YAML frontmatter.
//!
//! # Format
//! ```markdown
//! ---
//! title: "Issue title"
//! labels:
//!   - enhancement
//!   - rust
//! assignees: []
//! ---
//!
//! Issue body content here...
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Parsed GitHub Issue data from frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueData {
    pub title: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    pub body: String,
}

/// Parse a markdown file with YAML frontmatter
///
/// # Example
/// ```no_run
/// # use issue_syncer::parser::parse_issue_file;
/// let issue = parse_issue_file("issues/FEAT_my-feature.md")?;
/// println!("Title: {}", issue.title);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn parse_issue_file<P: AsRef<Path>>(path: P) -> Result<IssueData> {
    let content = std::fs::read_to_string(path.as_ref())
        .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;

    parse_frontmatter(&content)
}

/// Parse frontmatter from markdown content
pub fn parse_frontmatter(content: &str) -> Result<IssueData> {
    // Manual frontmatter extraction
    if !content.starts_with("---") {
        anyhow::bail!("Missing YAML frontmatter marker");
    }

    // Find end of frontmatter
    let rest = &content[3..];
    let end_marker = rest.find("\n---\n")
        .context("Missing frontmatter closing marker")?;

    let yaml_str = &rest[..end_marker];
    let body = rest[end_marker + 5..].trim();

    // Parse YAML
    let frontmatter: FrontmatterData = serde_yaml::from_str(yaml_str)
        .context("Failed to parse YAML frontmatter")?;

    Ok(IssueData {
        title: frontmatter.title,
        labels: frontmatter.labels,
        assignees: frontmatter.assignees,
        body: body.to_string(),
    })
}

/// Frontmatter structure (internal)
#[derive(Debug, Deserialize)]
struct FrontmatterData {
    title: String,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    assignees: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_basic() {
        let content = r#"---
title: "Test Issue"
labels:
  - bug
  - urgent
assignees:
  - john
---

This is the issue body.
"#;

        let issue = parse_frontmatter(content).unwrap();
        assert_eq!(issue.title, "Test Issue");
        assert_eq!(issue.labels, vec!["bug", "urgent"]);
        assert_eq!(issue.assignees, vec!["john"]);
        assert_eq!(issue.body, "This is the issue body.");
    }

    #[test]
    fn test_parse_frontmatter_no_labels() {
        let content = r#"---
title: "Simple Issue"
---

Body content.
"#;

        let issue = parse_frontmatter(content).unwrap();
        assert_eq!(issue.title, "Simple Issue");
        assert!(issue.labels.is_empty());
        assert!(issue.assignees.is_empty());
    }

    #[test]
    fn test_parse_frontmatter_multiline_body() {
        let content = r#"---
title: "Multi-line"
---

Line 1
Line 2
Line 3
"#;

        let issue = parse_frontmatter(content).unwrap();
        assert!(issue.body.contains("Line 1"));
        assert!(issue.body.contains("Line 2"));
        assert!(issue.body.contains("Line 3"));
    }
}
