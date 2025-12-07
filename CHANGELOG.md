# Changelog

All notable changes to the **Git-Core Protocol** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2025-12-06

### Added

- **🚀 Protocol Propagation System**: Automatic distribution of protocol updates to all managed repos
  - `protocol-propagation.yml` workflow triggers on version tags
  - `release-protocol.ps1` script for creating new versions
  - `.propagation.json` configuration for customizing target repos
  - Creates PRs or Issues automatically in all target repositories
  - Supports selective updates (workflows, agents, scripts, or full)
  - Priority-based rollout for staged deployments

- **🛡️ Self-Healing CI/CD Automation**: Automatic workflow failure detection and repair
  - `self-healing.yml` workflow monitors all workflow runs
  - Auto-classifies errors (transient/dependency/lint/test/code)
  - Auto-retry for transient errors (timeouts, rate limits)
  - Auto-fix for dependency issues (creates PR with lockfile updates)
  - Auto-fix for linting issues (runs formatters, creates PR)
  - Creates issues for code/test failures requiring manual intervention
  - `deploy-self-healing.ps1` script for multi-repo deployment

- **📧 Email Handler**: Gmail integration for workflow failure notifications
  - OAuth2 authentication with Gmail API
  - Detects workflow failures from email notifications
  - Archives emails automatically when workflows are fixed
  - Fallback polling method for environments without workflow_run support

### Changed

- Updated `.github/issues/` syncing to handle protocol update PRs
- Improved error handling in workflow file syntax validation

## [3.2.0-alpha] - 2025-12-06 📊 "Session Analytics"

### Added

- **📚 Agent Docs Structure**: New organized folder structure in `docs/agent-docs/`:
  - `specs/` - Technical specifications
  - `prompts/` - Reusable prompts for agents
  - `research/` - Technical investigations
  - `sessions/` - Archived sessions with metrics
  - `reports/` - Generated reports
  - `analysis/` - Optimization analyses
  - `archive/` - Obsolete documents

- **📊 Session Analytics**:
  - Enhanced `export-session.ps1` v2.0 with full metrics
  - New `archive-sessions.ps1` for organizing old sessions
  - New `generate-session-metrics.ps1` for monthly retrospectives

- **📈 Metrics Tracking**:
  - Session ID for traceability
  - Duration, model, files modified, commits made
  - Issues touched and accomplishments
  - Monthly aggregated METRICS.json

### Changed

- **Session Export**: Now includes accomplishments, next actions, and efficiency metrics
- **Documentation**: `docs/agent-docs/README.md` completely rewritten with archiving workflow

---

## [3.1.0-alpha] - 2025-12-06 🧪 "Context Intelligence"

### Experimental

- **🧠 Context-Driven Decision Engine**: Introduction of Semantic Risk Analysis for Guardian Agent.
- **🗺️ Risk Map**: New `.✨/risk-map.json` configuration to define risk scores per file path.
- **Shadow Mode**: Guardian Agent now calculates `semantic_risk_score` in logs without blocking merges (data collection phase).
- **🏗️ Hybrid Dispatcher**: `agent-dispatcher.yml` is now a thin wrapper around `scripts/dispatcher-core.ps1`.
- **🚦 Risk-Based Routing**: Dispatcher now routes high-risk issues (from `risk-map.json`) to Human/Senior Review automatically.

---

## [3.0.0] - 2025-12-06 🚀 "Full Autonomy"

### Added

- **🧠 Planner Agent**: New `planner-agent.yml` workflow that reads `ARCHITECTURE.md` and generates atomic issues automatically.
- **🛡️ Guardian Agent**: New `guardian-agent.yml` workflow with confidence scoring for auto-merge decisions.
- **Autonomous Cycle**: Complete development cycle without human intervention (except high-stakes operations).
- **Features Tracking**: New `.✨/features.json` template for tracking feature status.
- **New Labels**: `high-stakes`, `needs-human`, `auto-merged`, `ai-plan`, `planner-generated`.

### Changed

- **AGENTS.md**: Major update with v3.0 autonomous agent documentation.
- **Dispatcher Enhanced**: `agent-dispatcher.yml` now supports skill-matching strategy (planned).
- **Version Bump**: Protocol version updated to `3.0.0`.

### Breaking Changes

- **Required Files**: Projects using v3.0 should create `.✨/features.json` for Planner Agent.
- **Auto-Merge**: PRs meeting Guardian criteria (70%+ confidence) will be auto-merged.
- **New Labels Required**: Run `setup-labels.yml` to create v3.0 labels.

---

## [1.4.0] - 2025-12-04

### Added

- **Real Quarantine Logic**: `context-research-agent` now queries NPM, Crates.io, and PyPI APIs to verify package release dates.
- **Binary Automation**: New `build-tools.yml` workflow automatically compiles Rust agents and commits binaries to `bin/`.
- **Recursive Workflow Protection**: `workflow-validator` now detects and prevents infinite recursion loops.
- **Unified Versioning**: All protocol files now reference v1.4.0.

### Changed

- **Installer Update**: `install.ps1` and `install.sh` now include the `bin/` directory for pre-compiled tools.
- **Cleanup**: Removed deprecated `tools/deprecated/git-core-cli` folder.
- **Docs**: Updated `AGENTS.md` and `README.md` to reflect v1.4.0 capabilities.

### Fixed

- **CI Spam**: Fixed a bug where `workflow-validator` would trigger itself, creating hundreds of branches.
- **Metadata Inconsistency**: Unified version tags across all documentation files.

## [1.3.0] - 2025-11-01

- Initial stable release of the Git-Core Protocol.
- Added `context-research-agent`.
- Added `workflow-orchestrator`.
