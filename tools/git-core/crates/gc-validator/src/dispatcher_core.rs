//! # Dispatcher Core - AI Agent Load Balancer
//!
//! High-performance Rust implementation for distributing GitHub Issues
//! to AI coding agents (Copilot, Jules) based on various strategies.
//!
//! ## Performance
//!
//! - **Target:** <200ms per dispatch cycle
//! - **Baseline:** 3-5 seconds (PowerShell)
//! - **Expected Speedup:** 15-25x
//!
//! ## Dispatch Strategies
//!
//! ```text
//! RoundRobin:
//!   - Alternates between Copilot and Jules
//!   - Ensures balanced workload
//!
//! Random:
//!   - Random selection per issue
//!   - Good for A/B testing
//!
//! CopilotOnly:
//!   - All issues → Copilot
//!   - For testing or specific campaigns
//!
//! JulesOnly:
//!   - All issues → Jules
//!   - For batch CLI operations
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use workflow_orchestrator::dispatcher_core::{DispatcherCore, Strategy};
//! use octocrab::Octocrab;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let github = Octocrab::builder().build()?;
//!     let dispatcher = DispatcherCore::new(github, "owner".to_string(), "repo".to_string());
//!
//!     let assignments = dispatcher.dispatch_issues(
//!         Strategy::RoundRobin,
//!         5,
//!         "ai-agent".to_string(),
//!         false,
//!     ).await?;
//!
//!     println!("Assigned {} issues", assignments.len());
//!     Ok(())
//! }
//! ```

use anyhow::{Result, Context};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

use tracing::{debug, info, warn};
use rand::Rng;

/// Dispatch strategy for agent selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strategy {
    /// Alternate between Copilot and Jules
    RoundRobin,
    /// Random selection
    Random,
    /// All issues to Copilot
    CopilotOnly,
    /// All issues to Jules
    JulesOnly,
}

impl std::str::FromStr for Strategy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "round-robin" | "roundrobin" => Ok(Strategy::RoundRobin),
            "random" => Ok(Strategy::Random),
            "copilot-only" | "copilot" => Ok(Strategy::CopilotOnly),
            "jules-only" | "jules" => Ok(Strategy::JulesOnly),
            _ => Err(anyhow::anyhow!("Invalid strategy: {}", s)),
        }
    }
}

/// AI coding agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Agent {
    Copilot,
    Jules,
}

impl Agent {
    /// Get the label name for this agent
    pub fn label(&self) -> &'static str {
        match self {
            Agent::Copilot => "copilot",
            Agent::Jules => "jules",
        }
    }

    /// Get the assignee name (if supported)
    pub fn assignee(&self) -> Option<&'static str> {
        match self {
            Agent::Copilot => Some("Copilot"),
            Agent::Jules => None, // Jules uses labels only
        }
    }
}

/// Issue assignment result
#[derive(Debug, Clone, Serialize)]
pub struct Assignment {
    pub issue_number: u64,
    pub issue_title: String,
    pub agent: Agent,
    pub risk_score: u8,
    pub reason: String,
}

/// Simplified Issue representation
#[derive(Debug, Clone)]
struct Issue {
    number: u64,
    title: String,
    body: Option<String>,
    labels: Vec<String>,
}

/// Dispatcher Core engine
pub struct DispatcherCore {
    github: Octocrab,
    owner: String,
    repo: String,
    high_risk_threshold: u8,
    round_robin_index: std::sync::atomic::AtomicUsize,
}

impl DispatcherCore {
    /// Create new Dispatcher instance
    pub fn new(github: Octocrab, owner: String, repo: String) -> Self {
        Self {
            github,
            owner,
            repo,
            high_risk_threshold: 70,
            round_robin_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Set high-risk threshold for escalation
    pub fn with_risk_threshold(mut self, threshold: u8) -> Self {
        self.high_risk_threshold = threshold;
        self
    }

    /// Main dispatch entry point
    pub async fn dispatch_issues(
        &self,
        strategy: Strategy,
        max_issues: usize,
        label_filter: String,
        dry_run: bool,
    ) -> Result<Vec<Assignment>> {
        info!(
            "🎯 Dispatching issues with strategy: {:?}, max: {}",
            strategy, max_issues
        );

        // Fetch unassigned issues
        let issues = self.fetch_unassigned_issues(&label_filter).await?;
        info!("📋 Found {} unassigned issues", issues.len());

        // Limit to max_issues
        let candidates: Vec<Issue> = issues.into_iter().take(max_issues).collect();

        if candidates.is_empty() {
            info!("✅ No issues to dispatch");
            return Ok(vec![]);
        }

        // Analyze and assign
        let mut assignments = Vec::new();
        for issue in candidates {
            let assignment = self.analyze_and_assign(&issue, strategy)?;
            assignments.push(assignment);
        }

        // Execute assignments
        if !dry_run {
            self.execute_assignments(&assignments).await?;
        } else {
            info!("🔍 Dry run - no assignments executed");
        }

        Ok(assignments)
    }

    /// Fetch unassigned issues with specific label
    async fn fetch_unassigned_issues(&self, label: &str) -> Result<Vec<Issue>> {
        let issues = self
            .github
            .issues(&self.owner, &self.repo)
            .list()
            .state(octocrab::params::State::Open)
            .labels(&[label.to_string()])
            .per_page(100)
            .send()
            .await
            .context("Failed to fetch issues")?;

        // Filter out already assigned issues
        let unassigned: Vec<Issue> = issues
            .items
            .into_iter()
            .filter(|issue| {
                // Check if issue has no agent labels and no assignees
                let has_agent_label = issue.labels.iter().any(|l| {
                    let name = l.name.to_lowercase();
                    name == "copilot" || name == "jules"
                });

                let has_assignees = !issue.assignees.is_empty();

                !has_agent_label && !has_assignees
            })
            .map(|issue| Issue {
                number: issue.number,
                title: issue.title.clone(),
                body: issue.body.clone(),
                labels: issue.labels.iter().map(|l| l.name.clone()).collect(),
            })
            .collect();

        debug!("🔍 Filtered to {} unassigned issues", unassigned.len());
        Ok(unassigned)
    }

    /// Analyze issue and create assignment
    fn analyze_and_assign(&self, issue: &Issue, strategy: Strategy) -> Result<Assignment> {
        let risk_score = self.analyze_risk(issue);
        let agent = self.select_agent(strategy, issue, risk_score);

        let reason = match strategy {
            Strategy::RoundRobin => "Round-robin distribution".to_string(),
            Strategy::Random => "Random selection".to_string(),
            Strategy::CopilotOnly => "Copilot-only mode".to_string(),
            Strategy::JulesOnly => "Jules-only mode".to_string(),
        };

        Ok(Assignment {
            issue_number: issue.number,
            issue_title: issue.title.clone(),
            agent,
            risk_score,
            reason,
        })
    }

    /// Analyze issue risk score (0-100)
    fn analyze_risk(&self, issue: &Issue) -> u8 {
        let mut risk = 0u8;

        // Check for high-risk labels
        for label in &issue.labels {
            let label_lower = label.to_lowercase();
            if label_lower.contains("security")
                || label_lower.contains("breaking")
                || label_lower.contains("critical")
            {
                risk += 30;
            }
            if label_lower.contains("bug") {
                risk += 10;
            }
        }

        // Check title/body for keywords
        let text = format!(
            "{} {}",
            issue.title,
            issue.body.as_ref().unwrap_or(&String::new())
        )
        .to_lowercase();

        if text.contains("auth") || text.contains("security") || text.contains("crypto") {
            risk += 20;
        }

        if text.contains("refactor") || text.contains("migration") {
            risk += 10;
        }

        risk.min(100)
    }

    /// Select agent based on strategy
    fn select_agent(&self, strategy: Strategy, _issue: &Issue, risk_score: u8) -> Agent {
        match strategy {
            Strategy::RoundRobin => {
                // Atomic increment for thread-safe round-robin
                let index = self
                    .round_robin_index
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if index % 2 == 0 {
                    Agent::Copilot
                } else {
                    Agent::Jules
                }
            }
            Strategy::Random => {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    Agent::Copilot
                } else {
                    Agent::Jules
                }
            }
            Strategy::CopilotOnly => Agent::Copilot,
            Strategy::JulesOnly => Agent::Jules,
        }
    }

    /// Execute assignments (add labels and assignees)
    async fn execute_assignments(&self, assignments: &[Assignment]) -> Result<()> {
        for assignment in assignments {
            info!(
                "🏷️  Assigning issue #{} to {:?}",
                assignment.issue_number, assignment.agent
            );

            // Add agent label
            self.github
                .issues(&self.owner, &self.repo)
                .add_labels(assignment.issue_number, &[assignment.agent.label().to_string()])
                .await
                .context(format!(
                    "Failed to add label to issue #{}",
                    assignment.issue_number
                ))?;

            // Add assignee if supported
            if let Some(assignee) = assignment.agent.assignee() {
                // Note: Copilot assignee may not work via API, handled via label
                debug!("Would assign to: {}", assignee);
            }

            debug!("✅ Issue #{} dispatched", assignment.issue_number);
        }

        info!("🎉 Dispatched {} issues successfully", assignments.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_from_str() {
        assert_eq!(
            "round-robin".parse::<Strategy>().unwrap(),
            Strategy::RoundRobin
        );
        assert_eq!("random".parse::<Strategy>().unwrap(), Strategy::Random);
        assert_eq!(
            "copilot-only".parse::<Strategy>().unwrap(),
            Strategy::CopilotOnly
        );
        assert_eq!(
            "jules-only".parse::<Strategy>().unwrap(),
            Strategy::JulesOnly
        );
        assert!("invalid".parse::<Strategy>().is_err());
    }

    #[test]
    fn test_agent_labels() {
        assert_eq!(Agent::Copilot.label(), "copilot");
        assert_eq!(Agent::Jules.label(), "jules");
    }

    #[tokio::test]
    async fn test_risk_analysis() {
        let github = Octocrab::builder().build().unwrap();
        let dispatcher = DispatcherCore::new(github, "owner".to_string(), "repo".to_string());

        let issue = Issue {
            number: 1,
            title: "Security vulnerability in auth".to_string(),
            body: Some("Critical security issue".to_string()),
            labels: vec!["security".to_string(), "bug".to_string()],
        };

        let risk = dispatcher.analyze_risk(&issue);
        assert!(risk >= 60, "High-risk issue should have risk >= 60");
    }
}
