//! Configuration module for Git-Core CLI

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// GitHub repository information
pub const GITHUB_OWNER: &str = "iberi22";
pub const GITHUB_REPO: &str = "Git-Core-Protocol";
pub const RAW_URL: &str = "https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main";

/// Directory names
pub const NEW_AI_DIR: &str = ".✨";
pub const OLD_AI_DIR: &str = ".ai";
pub const BACKUP_DIR: &str = ".git-core-backup";
pub const TEMP_DIR: &str = ".git-core-temp";

/// Protocol file locations
pub const VERSION_FILE: &str = ".git-core-protocol-version";
pub const ARCHITECTURE_FILE: &str = "ARCHITECTURE.md";
pub const CONTEXT_LOG_FILE: &str = "CONTEXT_LOG.md";
pub const AGENT_INDEX_FILE: &str = "AGENT_INDEX.md";

/// Files that should never be overwritten during upgrade
pub const PRESERVE_FILES: &[&str] = &[
    "ARCHITECTURE.md",
    "CONTEXT_LOG.md",
    ".gitignore",
    "README.md",
];

/// Protocol workflow files
pub const PROTOCOL_WORKFLOWS: &[&str] = &[
    "update-protocol.yml",
    "structure-validator.yml",
    "codex-review.yml",
    "agent-dispatcher.yml",
    "living-context.yml",
];

/// Configuration for the Git-Core CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub installed_at: Option<String>,
    pub last_check: Option<String>,
    pub auto_update: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: String::from("0.0.0"),
            installed_at: None,
            last_check: None,
            auto_update: true,
        }
    }
}

impl Config {
    /// Load configuration from the current directory
    pub fn load() -> anyhow::Result<Self> {
        let version_file = PathBuf::from(VERSION_FILE);

        if version_file.exists() {
            let version = std::fs::read_to_string(&version_file)?
                .trim()
                .to_string();

            Ok(Self {
                version,
                ..Default::default()
            })
        } else {
            Ok(Self::default())
        }
    }

    /// Check if protocol is installed
    pub fn is_installed(&self) -> bool {
        self.version != "0.0.0"
    }

    /// Get the AI directory path (prefers .✨ over .ai)
    pub fn get_ai_dir() -> Option<PathBuf> {
        let new_dir = PathBuf::from(NEW_AI_DIR);
        let old_dir = PathBuf::from(OLD_AI_DIR);

        if new_dir.exists() {
            Some(new_dir)
        } else if old_dir.exists() {
            Some(old_dir)
        } else {
            None
        }
    }

    /// Check if migration from .ai to .✨ is needed
    pub fn needs_migration() -> bool {
        let old_dir = PathBuf::from(OLD_AI_DIR);
        let new_dir = PathBuf::from(NEW_AI_DIR);

        old_dir.exists() && !new_dir.exists()
    }
}

/// Represents a protocol version
#[derive(Debug, Clone)]
pub struct ProtocolVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProtocolVersion {
    pub fn parse(version: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = version.trim_start_matches('v').split('.').collect();

        if parts.len() != 3 {
            anyhow::bail!("Invalid version format: {}", version);
        }

        Ok(Self {
            major: parts[0].parse()?,
            minor: parts[1].parse()?,
            patch: parts[2].parse()?,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
