//! Issue Mapping Storage
//!
//! Manages bidirectional mapping between local .md files and GitHub Issue numbers.
//!
//! # File Format
//! ```json
//! {
//!   "FEAT_my-feature.md": 42,
//!   "BUG_login-error.md": 43
//! }
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Bidirectional mapping between files and issue numbers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssueMapping {
    #[serde(flatten)]
    file_to_issue: HashMap<String, u64>,
}

impl IssueMapping {
    /// Load mapping from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .context("Failed to read mapping file")?;

        let mapping: Self = serde_json::from_str(&content)
            .context("Failed to parse mapping JSON")?;

        Ok(mapping)
    }

    /// Save mapping to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.file_to_issue)
            .context("Failed to serialize mapping")?;

        std::fs::write(path.as_ref(), json)
            .with_context(|| format!("Failed to write mapping to {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Get issue number for a file
    pub fn get_issue(&self, file: &str) -> Option<u64> {
        self.file_to_issue.get(file).copied()
    }

    /// Get file path for an issue number
    pub fn get_file(&self, issue_number: u64) -> Option<String> {
        self.file_to_issue
            .iter()
            .find(|(_, &num)| num == issue_number)
            .map(|(file, _)| file.clone())
    }

    /// Add a new mapping
    pub fn add(&mut self, file: String, issue_number: u64) {
        self.file_to_issue.insert(file, issue_number);
    }

    /// Remove a mapping by file name
    pub fn remove_by_file(&mut self, file: &str) -> Option<u64> {
        self.file_to_issue.remove(file)
    }

    /// Remove a mapping by issue number
    pub fn remove_by_issue(&mut self, issue_number: u64) -> Option<String> {
        let file = self.get_file(issue_number)?;
        self.file_to_issue.remove(&file);
        Some(file)
    }

    /// Check if a file is mapped
    pub fn contains_file(&self, file: &str) -> bool {
        self.file_to_issue.contains_key(file)
    }

    /// Check if an issue is mapped
    pub fn contains_issue(&self, issue_number: u64) -> bool {
        self.file_to_issue.values().any(|&num| num == issue_number)
    }

    /// Get all mapped files
    pub fn files(&self) -> Vec<String> {
        self.file_to_issue.keys().cloned().collect()
    }

    /// Get all mapped issue numbers
    pub fn issues(&self) -> Vec<u64> {
        self.file_to_issue.values().copied().collect()
    }

    /// Get the number of mappings
    pub fn len(&self) -> usize {
        self.file_to_issue.len()
    }

    /// Check if mapping is empty
    pub fn is_empty(&self) -> bool {
        self.file_to_issue.is_empty()
    }

    /// Clear all mappings
    pub fn clear(&mut self) {
        self.file_to_issue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_add_and_get() {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);

        assert_eq!(mapping.get_issue("FEAT_test.md"), Some(42));
        assert_eq!(mapping.get_file(42), Some("FEAT_test.md".to_string()));
    }

    #[test]
    fn test_remove_by_file() {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);

        let removed = mapping.remove_by_file("FEAT_test.md");
        assert_eq!(removed, Some(42));
        assert!(!mapping.contains_file("FEAT_test.md"));
    }

    #[test]
    fn test_remove_by_issue() {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);

        let removed = mapping.remove_by_issue(42);
        assert_eq!(removed, Some("FEAT_test.md".to_string()));
        assert!(!mapping.contains_issue(42));
    }

    #[test]
    fn test_save_and_load() {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);
        mapping.add("BUG_error.md".to_string(), 43);

        let temp_file = NamedTempFile::new().unwrap();
        mapping.save(temp_file.path()).unwrap();

        let loaded = IssueMapping::load(temp_file.path()).unwrap();
        assert_eq!(loaded.get_issue("FEAT_test.md"), Some(42));
        assert_eq!(loaded.get_issue("BUG_error.md"), Some(43));
    }

    #[test]
    fn test_bidirectional_lookup() {
        let mut mapping = IssueMapping::default();
        mapping.add("FEAT_test.md".to_string(), 42);

        // Forward lookup
        assert_eq!(mapping.get_issue("FEAT_test.md"), Some(42));

        // Reverse lookup
        assert_eq!(mapping.get_file(42), Some("FEAT_test.md".to_string()));
    }
}
