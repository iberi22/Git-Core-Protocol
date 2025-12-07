//! # Guardian Core - Auto-Merge Decision Engine
//!
//! High-performance Rust implementation of the Guardian Agent for evaluating
//! Pull Requests and making auto-merge decisions based on confidence scoring.
//!
//! **Performance Target:** <100ms per PR evaluation (vs 2-3s PowerShell)

use anyhow::Result;
use octocrab::{Octocrab, models::pulls::ReviewState, params::repos::Commitish};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, info, warn};

const DEFAULT_THRESHOLD: u8 = 70;

/// Decision outcome from PR evaluation
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Decision {
    AutoMerge { confidence: u8 },
    Escalate { reason: String, confidence: u8 },
    Blocked { reason: String },
}

impl Decision {
    pub fn from_confidence(confidence: u8, threshold: u8, blocker: Option<String>) -> Self {
        if let Some(reason) = blocker {
            return Decision::Blocked { reason };
        }

        if confidence >= threshold {
            Decision::AutoMerge { confidence }
        } else {
            Decision::Escalate {
                reason: format!("Confidence {} below threshold {}", confidence, threshold),
                confidence,
            }
        }
    }
}

/// PR data aggregated from GitHub API
#[derive(Debug, Clone)]
pub struct PrData {
    pub number: u64,
    pub labels: Vec<String>,
    pub reviews: Vec<ReviewState>,
    pub additions: u32,
    pub deletions: u32,
    pub changed_files: u32,
    pub files: Vec<String>,
    pub checks_passed: bool,
    pub head_ref: String,
}

/// Risk map configuration loaded from .✨/risk-map.json
#[derive(Debug, Clone, Deserialize)]
pub struct RiskMap {
    pub paths: HashMap<String, PathRisk>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathRisk {
    pub risk: u8,
    pub reason: String,
}

impl RiskMap {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let map = serde_json::from_str(&content)?;
        Ok(map)
    }
}

/// Guardian Core engine
pub struct GuardianCore {
    github: Octocrab,
    owner: String,
    repo: String,
    risk_map: Option<RiskMap>,
    threshold: u8,
}

impl GuardianCore {
    /// Create new Guardian instance
    pub fn new(github: Octocrab, owner: String, repo: String) -> Self {
        Self {
            github,
            owner,
            repo,
            risk_map: None,
            threshold: DEFAULT_THRESHOLD,
        }
    }

    /// Set confidence threshold (default: 70)
    pub fn with_threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold;
        self
    }

    /// Load risk map from file
    pub fn with_risk_map(mut self, path: &str) -> Result<Self> {
        self.risk_map = Some(RiskMap::from_file(path)?);
        info!("✅ Risk map loaded from {}", path);
        Ok(self)
    }

    /// Main evaluation entry point
    pub async fn evaluate_pr(&self, pr_number: u64, dry_run: bool) -> Result<Decision> {
        info!("🛡️ Evaluating PR #{}", pr_number);

        // Fetch all PR data in parallel
        let pr_data = self.fetch_pr_data(pr_number).await?;

        // Check blocking labels
        if let Some(blocker) = self.check_blockers(&pr_data.labels) {
            warn!("⛔ PR blocked: {}", blocker);
            return Ok(Decision::Blocked { reason: blocker });
        }

        // Calculate confidence score
        let ci_ok = pr_data.checks_passed;
        let reviews_ok = self.check_reviews(&pr_data.reviews);
        let risk_score = self.calculate_risk(&pr_data.files);
        let size_penalty = self.calculate_size_penalty(pr_data.additions, pr_data.deletions);

        let mut confidence: u8 = 0;

        // CI checks (required)
        if ci_ok {
            confidence += 40;
            debug!("✅ CI passed: +40 confidence");
        } else {
            warn!("❌ CI failed: 0 confidence");
            return Ok(Decision::Escalate {
                reason: "CI checks failed".to_string(),
                confidence: 0,
            });
        }

        // Reviews (required)
        if reviews_ok {
            confidence += 40;
            debug!("✅ Reviews approved: +40 confidence");
        } else {
            warn!("❌ No approvals or changes requested");
            return Ok(Decision::Escalate {
                reason: "No approved reviews".to_string(),
                confidence,
            });
        }

        // Risk analysis (penalty)
        let risk_penalty = (risk_score / 10).min(10); // Max -10
        confidence = confidence.saturating_sub(risk_penalty);
        debug!("📊 Risk score: {} (penalty: -{})", risk_score, risk_penalty);

        // Size penalty
        confidence = confidence.saturating_sub(size_penalty);
        debug!("📏 Size penalty: -{}", size_penalty);

        // Tests bonus
        if self.has_tests(&pr_data.files) {
            confidence = confidence.saturating_add(15).min(100);
            debug!("🧪 Tests included: +15 confidence");
        }

        // Single scope bonus
        if self.is_single_scope(&pr_data.files) {
            confidence = confidence.saturating_add(10).min(100);
            debug!("🎯 Single scope: +10 confidence");
        }

        info!("📊 Final confidence: {}/{}", confidence, self.threshold);

        let decision = Decision::from_confidence(confidence, self.threshold, None);

        // Execute decision
        if !dry_run {
            self.execute_decision(pr_number, &decision).await?;
        }

        Ok(decision)
    }

    /// Fetch PR data from GitHub API
    async fn fetch_pr_data(&self, pr_number: u64) -> Result<PrData> {
        let pulls = self.github.pulls(&self.owner, &self.repo);
        let pr = pulls.get(pr_number).await?;

        // Fetch reviews
        let reviews = pulls.list_reviews(pr_number).send().await?;
        let review_states: Vec<ReviewState> = reviews
            .items
            .into_iter()
            .filter_map(|r| r.state)
            .collect();

        // Fetch files
        let files_page = pulls.list_files(pr_number).await?;
        let files: Vec<String> = files_page
            .items
            .into_iter()
            .map(|f| f.filename)
            .collect();

        // Fetch checks
        let head_ref = pr.head.ref_field.clone();
        let checks = self
            .github
            .checks(&self.owner, &self.repo)
            .list_check_runs_for_git_ref(Commitish(head_ref))
            .send()
            .await?;

        let checks_passed = checks.check_runs.iter().all(|check| {
            check.conclusion.as_ref().map_or(false, |c| c.as_str() == "success" || c.as_str() == "skipped" || c.as_str() == "neutral")
        });

        Ok(PrData {
            number: pr_number,
            labels: pr.labels.unwrap_or_default().into_iter().map(|l| l.name).collect(),
            reviews: review_states,
            additions: pr.additions.unwrap_or(0) as u32,
            deletions: pr.deletions.unwrap_or(0) as u32,
            changed_files: pr.changed_files.unwrap_or(0) as u32,
            files,
            checks_passed,
            head_ref: pr.head.ref_field,
        })
    }

    /// Check for blocking labels
    fn check_blockers(&self, labels: &[String]) -> Option<String> {
        if labels.iter().any(|l| l == "high-stakes") {
            return Some("high-stakes label detected".to_string());
        }
        if labels.iter().any(|l| l == "needs-human") {
            return Some("needs-human label detected".to_string());
        }
        None
    }

    /// Check review status
    fn check_reviews(&self, reviews: &[ReviewState]) -> bool {
        let approved = reviews.iter().filter(|r| **r == ReviewState::Approved).count();
        let changes_requested = reviews
            .iter()
            .filter(|r| **r == ReviewState::ChangesRequested)
            .count();

        approved > 0 && changes_requested == 0
    }

    /// Calculate risk score from changed files
    fn calculate_risk(&self, files: &[String]) -> u8 {
        let Some(risk_map) = &self.risk_map else {
            return 0;
        };

        let mut max_risk = 0u8;

        for file in files {
            for (pattern, config) in &risk_map.paths {
                // Simple pattern matching (can be improved with glob)
                if self.matches_pattern(file, pattern) {
                    max_risk = max_risk.max(config.risk);
                    debug!("🔥 File {} matches pattern {} (risk: {})", file, pattern, config.risk);
                }
            }
        }

        max_risk
    }

    /// Simple pattern matching
    fn matches_pattern(&self, file: &str, pattern: &str) -> bool {
        // Convert glob-like pattern to regex
        let regex_pattern = pattern
            .replace('.', r"\.")
            .replace('*', ".*")
            .replace('?', ".");

        Regex::new(&format!("^{}$", regex_pattern))
            .ok()
            .and_then(|re| Some(re.is_match(file)))
            .unwrap_or(false)
    }

    /// Calculate size penalty based on diff size
    fn calculate_size_penalty(&self, additions: u32, deletions: u32) -> u8 {
        let total = additions + deletions;
        match total {
            0..=100 => 0,
            101..=300 => 5,
            301..=500 => 10,
            _ => 20,
        }
    }

    /// Check if PR includes tests
    fn has_tests(&self, files: &[String]) -> bool {
        files.iter().any(|f| {
            f.contains("test") || f.contains("spec") || f.starts_with("tests/")
        })
    }

    /// Check if all files are in single scope/module
    fn is_single_scope(&self, files: &[String]) -> bool {
        if files.is_empty() {
            return false;
        }

        // Extract first directory from each file
        let scopes: Vec<&str> = files
            .iter()
            .filter_map(|f| f.split('/').next())
            .collect();

        // Check if all files share the same top-level directory
        scopes.windows(2).all(|w| w[0] == w[1])
    }

    /// Execute the decision (merge or escalate)
    async fn execute_decision(&self, pr_number: u64, decision: &Decision) -> Result<()> {
        match decision {
            Decision::AutoMerge { confidence } => {
                info!("✅ Auto-merging PR #{} (confidence: {})", pr_number, confidence);
                
                // Add comment before merging
                let comment = format!(
                    "🤖 **Guardian Agent**: Auto-merge approved (confidence: {}%)\n\n\
                     All checks passed, reviews approved, and confidence threshold met.",
                    confidence
                );
                
                self.github
                    .issues(&self.owner, &self.repo)
                    .create_comment(pr_number, comment)
                    .await?;

                // Merge the PR
                self.github
                    .pulls(&self.owner, &self.repo)
                    .merge(pr_number)
                    .method(octocrab::params::pulls::MergeMethod::Squash)
                    .send()
                    .await?;

                info!("✅ PR #{} merged successfully", pr_number);
            }
            Decision::Escalate { reason, confidence } => {
                info!("⚠️ Escalating PR #{}: {} (confidence: {})", pr_number, reason, confidence);
                
                let comment = format!(
                    "🤖 **Guardian Agent**: Manual review required\n\n\
                     **Reason:** {}\n\
                     **Confidence:** {}%\n\n\
                     A human reviewer must approve this PR for merge.",
                    reason, confidence
                );
                
                self.github
                    .issues(&self.owner, &self.repo)
                    .create_comment(pr_number, comment)
                    .await?;

                // Add needs-human label
                self.github
                    .issues(&self.owner, &self.repo)
                    .add_labels(pr_number, &[String::from("needs-human")])
                    .await?;
            }
            Decision::Blocked { reason } => {
                warn!("⛔ PR #{} blocked: {}", pr_number, reason);
                
                let comment = format!(
                    "🤖 **Guardian Agent**: PR blocked\n\n\
                     **Reason:** {}\n\n\
                     This PR cannot be auto-merged. Please review the blocking conditions.",
                    reason
                );
                
                self.github
                    .issues(&self.owner, &self.repo)
                    .create_comment(pr_number, comment)
                    .await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_from_confidence() {
        let decision = Decision::from_confidence(80, 70, None);
        assert!(matches!(decision, Decision::AutoMerge { confidence: 80 }));

        let decision = Decision::from_confidence(60, 70, None);
        assert!(matches!(decision, Decision::Escalate { .. }));

        let decision = Decision::from_confidence(80, 70, Some("blocked".to_string()));
        assert!(matches!(decision, Decision::Blocked { .. }));
    }

    #[test]
    fn test_size_penalty() {
        let github = Octocrab::builder().build().unwrap();
        let guardian = GuardianCore::new(github, "owner".to_string(), "repo".to_string());

        assert_eq!(guardian.calculate_size_penalty(50, 50), 0);
        assert_eq!(guardian.calculate_size_penalty(150, 150), 5);
        assert_eq!(guardian.calculate_size_penalty(400, 100), 10);
        assert_eq!(guardian.calculate_size_penalty(600, 600), 20);
    }

    #[test]
    fn test_has_tests() {
        let github = Octocrab::builder().build().unwrap();
        let guardian = GuardianCore::new(github, "owner".to_string(), "repo".to_string());

        let files = vec!["src/main.rs".to_string(), "tests/test_main.rs".to_string()];
        assert!(guardian.has_tests(&files));

        let files = vec!["src/main.rs".to_string()];
        assert!(!guardian.has_tests(&files));
    }

    #[test]
    fn test_is_single_scope() {
        let github = Octocrab::builder().build().unwrap();
        let guardian = GuardianCore::new(github, "owner".to_string(), "repo".to_string());

        let files = vec!["src/main.rs".to_string(), "src/lib.rs".to_string()];
        assert!(guardian.is_single_scope(&files));

        let files = vec!["src/main.rs".to_string(), "tests/test.rs".to_string()];
        assert!(!guardian.is_single_scope(&files));
    }
}
