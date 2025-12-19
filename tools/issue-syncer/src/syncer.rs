//! Core Issue Syncer Logic
//!
//! Bidirectional sync between `.github/issues/*.md` files and GitHub Issues.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};
use walkdir::WalkDir;

use crate::github::GitHubClient;
use crate::mapping::IssueMapping;
use crate::parser::parse_issue_file;

/// Sync report with statistics
#[derive(Debug, Default)]
pub struct SyncReport {
    pub created: usize,
    pub updated: usize,
    pub deleted: usize,
    pub skipped: usize,
    pub errors: usize,
}

impl SyncReport {
    pub fn total_operations(&self) -> usize {
        self.created + self.updated + self.deleted
    }
}

/// Issue syncer core
pub struct IssueSyncer {
    github: GitHubClient,
    issues_dir: PathBuf,
    mapping_file: PathBuf,
    mapping: IssueMapping,
    dry_run: bool,
}

impl IssueSyncer {
    /// Create a new issue syncer
    pub fn new(
        github: GitHubClient,
        issues_dir: PathBuf,
        mapping_file: PathBuf,
    ) -> Result<Self> {
        // Load or create mapping
        let mapping = if mapping_file.exists() {
            IssueMapping::load(&mapping_file)
                .context("Failed to load issue mapping")?
        } else {
            IssueMapping::default()
        };

        Ok(Self {
            github,
            issues_dir,
            mapping_file,
            mapping,
            dry_run: false,
        })
    }

    /// Enable dry-run mode (no actual API calls)
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn github(&self) -> &GitHubClient {
        &self.github
    }

    /// Sync all: push local changes + pull closed issues
    pub async fn sync_all(&mut self) -> Result<SyncReport> {
        info!("Starting bidirectional sync");

        let push_report = self.push().await?;
        let pull_report = self.pull().await?;

        Ok(SyncReport {
            created: push_report.created,
            updated: push_report.updated,
            deleted: pull_report.deleted,
            skipped: push_report.skipped + pull_report.skipped,
            errors: push_report.errors + pull_report.errors,
        })
    }

    /// Push: Sync local .md files to GitHub Issues
    pub async fn push(&mut self) -> Result<SyncReport> {
        info!("Pushing local files to GitHub");

        let mut report = SyncReport::default();
        let files = self.scan_issue_files()?;

        for file_path in files {
            let filename = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Skip mapping file
            if filename.starts_with('.') {
                continue;
            }

            // Parse file
            let issue_data = match parse_issue_file(&file_path) {
                Ok(data) => data,
                Err(e) => {
                    warn!("Failed to parse {}: {}", filename, e);
                    report.errors += 1;
                    continue;
                }
            };

            // Check if mapped
            if let Some(issue_number) = self.mapping.get_issue(filename) {
                // Update existing issue
                if !self.dry_run {
                    match self.github.update_issue(issue_number, &issue_data).await {
                        Ok(_) => {
                            info!("Updated issue #{} from {}", issue_number, filename);
                            report.updated += 1;
                        }
                        Err(e) => {
                            warn!("Failed to update issue #{}: {}", issue_number, e);
                            report.errors += 1;
                        }
                    }
                } else {
                    info!("[DRY RUN] Would update issue #{}", issue_number);
                    report.updated += 1;
                }
            } else {
                // Create new issue
                if !self.dry_run {
                    match self.github.create_issue(&issue_data).await {
                        Ok(number) => {
                            info!("Created issue #{} from {}", number, filename);
                            self.mapping.add(filename.to_string(), number);
                            report.created += 1;
                        }
                        Err(e) => {
                            warn!("Failed to create issue from {}: {}", filename, e);
                            report.errors += 1;
                        }
                    }
                } else {
                    info!("[DRY RUN] Would create issue from {}", filename);
                    report.created += 1;
                }
            }
        }

        // Save mapping if not dry run
        if !self.dry_run {
            self.save_mapping()?;
        }

        Ok(report)
    }

    /// Pull: Delete local files for closed issues
    pub async fn pull(&mut self) -> Result<SyncReport> {
        info!("Pulling closed issues from GitHub");

        let mut report = SyncReport::default();
        let closed_issues = if !self.dry_run {
            self.github.fetch_closed_issues().await?
        } else {
            vec![]
        };

        for issue in closed_issues {
            if let Some(filename) = self.mapping.get_file(issue.number) {
                let file_path = self.issues_dir.join(&filename);

                if file_path.exists() {
                    if !self.dry_run {
                        match std::fs::remove_file(&file_path) {
                            Ok(_) => {
                                info!("Deleted {} for closed issue #{}", filename, issue.number);
                                self.mapping.remove_by_issue(issue.number);
                                report.deleted += 1;
                            }
                            Err(e) => {
                                warn!("Failed to delete {}: {}", filename, e);
                                report.errors += 1;
                            }
                        }
                    } else {
                        info!("[DRY RUN] Would delete {} for closed issue #{}", filename, issue.number);
                        report.deleted += 1;
                    }
                }
            }
        }

        // Save mapping if not dry run
        if !self.dry_run {
            self.save_mapping()?;
        }

        Ok(report)
    }

    /// Scan issues directory for .md files
    fn scan_issue_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.issues_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// Save mapping to disk
    fn save_mapping(&self) -> Result<()> {
        self.mapping.save(&self.mapping_file)
            .context("Failed to save mapping")
    }

    /// Get current mapping
    pub fn mapping(&self) -> &IssueMapping {
        &self.mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use octocrab::Octocrab;

    fn create_test_syncer() -> (IssueSyncer, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let issues_dir = temp_dir.path().join("issues");
        std::fs::create_dir(&issues_dir).unwrap();

        let mapping_file = issues_dir.join(".issue-mapping.json");

        let client = Octocrab::builder().build().unwrap();
        let github = GitHubClient::new(client, "owner".to_string(), "repo".to_string());

        let syncer = IssueSyncer::new(github, issues_dir, mapping_file)
            .unwrap()
            .with_dry_run(true);

        (syncer, temp_dir)
    }

    #[tokio::test]
    async fn test_syncer_creation() {
        let (syncer, _temp) = create_test_syncer();
        assert!(syncer.mapping().is_empty());
    }

    #[tokio::test]
    async fn test_scan_empty_directory() {
        let (syncer, _temp) = create_test_syncer();
        let files = syncer.scan_issue_files().unwrap();
        assert!(files.is_empty());
    }

    #[tokio::test]
    async fn test_scan_with_files() {
        let (syncer, _temp) = create_test_syncer();

        // Create a test file
        let test_file = syncer.issues_dir.join("TEST_issue.md");
        std::fs::write(&test_file, "---\ntitle: Test\n---\nBody").unwrap();

        let files = syncer.scan_issue_files().unwrap();
        assert_eq!(files.len(), 1);
    }
}
