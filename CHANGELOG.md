# Changelog

All notable changes to the **Git-Core Protocol** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.2.1] - 2025-12-08 ⚡ "Performance & Simplification"

### Changed

- **⚡ Updated Protocol Tagline**: "Inteligente, rápida y minimalista - Rust-powered, sub-second execution"
  - Reflects the 10-30x performance improvements from Rust migrations
  - Emphasizes speed and minimalism over complexity
  - Updated in README.md, AGENTS.md, and copilot-instructions.md

- **🧹 Repository Cleanup**:
  - Removed 43 merged/obsolete branches (93.6% reduction)
  - Enabled auto-delete on PR merge
  - Only 3 branches remain: `main`, `living-context/main`

### Removed

- **📧 Email Handler Fallback** (463 lines):
  - Removed `.github/workflows/email-cleanup.yml`
  - Removed `tools/email-handler/` directory
  - Redundant with native `workflow_run` events in `self-healing.yml`
  - Eliminates Gmail API dependency and configuration complexity

### Fixed

- **✅ CI Failures Resolved** (#67, #70, #71):
  - Root cause: `gh` CLI warnings causing exit code 1
  - Fix: `$ErrorActionPreference = 'SilentlyContinue'` in `detect-repo-config.ps1`
  - All affected workflows now passing

### Architecture

- **Simplified Self-Healing Stack**:
  - Native GitHub events only (no external polling)
  - Reduced token consumption
  - Eliminated fallback methods requiring external credentials

### Documentation

- Version bumped to 3.2.1 across all files
- Updated performance metrics in documentation

## [3.2.0] - 2025-12-07 📚 "Diátaxis Documentation System"

### Added

- **📚 Diátaxis Framework Adoption**: Complete documentation reorganization
  - `docs/tutorials/` - Learning-oriented guides (learn by doing)
  - `docs/how-to/` - Task-oriented guides (solve problems)
  - `docs/reference/` - Information-oriented docs (look up facts)
  - `docs/explanation/` - Understanding-oriented content (understand "why")
  - Each quadrant has README.md with clear principles and guidelines
  - `docs/DOCUMENTATION_SYSTEM.md` - Complete explanation of the new system
  - `docs/wiki/Home.md` - GitHub Wiki homepage ready for publication

### Changed

- **Documentation Structure Overhaul**:
  - Migrated `ATOMIC_COMMITS.md` → `tutorials/` (learning-oriented)
  - Migrated `SESSION_EXPORT.md` → `how-to/` (task-oriented)
  - Migrated `COMMIT_STANDARD.md` → `reference/` (information-oriented)
  - Migrated `CLI_TRUST.md` → `explanation/` (understanding-oriented)
  - Updated `docs/README.md` with Diátaxis visual map and navigation
  - Kept `agent-docs/` separate for AI agent technical specifications

### Documentation

- **Separation of Concerns**:
  - Humans: Organized by learning needs (4 Diátaxis quadrants)
  - AI Agents: Technical specs in `agent-docs/` unchanged
  - Clear distinction between tutorials, how-tos, reference, and explanations

- **GitHub Wiki Ready**:
  - `wiki/Home.md` with complete navigation structure
  - Quick start guides by role (Developer, AI Agent, Architect, New User)
  - Links to all documentation organized by type

### Benefits

- ✅ Clear navigation - Users know exactly where to look
- ✅ No content confusion - Tutorials don't mix with reference
- ✅ Scalable structure - Easy to add new docs without ambiguity
- ✅ Industry standard - Diátaxis used by Google, Cloudflare, Gatsby
- ✅ Wiki-ready - GitHub Wiki integration prepared

### Migration Guide

See `docs/explanation/DOCUMENTATION_MIGRATION.md` for how to handle existing project docs.

## [3.1.0] - 2025-12-07 🎯 "Adaptive Workflows"

### Added

- **🎯 Adaptive Workflow System**: Intelligent resource optimization for GitHub Actions
  - `scripts/detect-repo-config.ps1` - PowerShell detector for Windows
  - `scripts/detect-repo-config.sh` - Bash detector for Linux/macOS
  - `.github/workflows/_repo-config.yml` - Reusable configuration workflow
  - Automatic detection of repository visibility (public/private)
  - Three-tier scheduling modes:
    - **AGGRESSIVE** (public repos): Full schedules, multi-repo monitoring, unlimited
    - **MODERATE** (private main repo): 6-hourly schedules, ~3,000 min/month
    - **CONSERVATIVE** (other private): Event-based only, ~600 min/month
  - Zero-configuration for users - fully automatic adaptation
  - Cross-platform support (PowerShell + Bash)

### Changed

- **Optimized `global-self-healing.yml`**:
  - Migrated primary trigger from `schedule` to `workflow_run` (FREE - only runs when needed)
  - Added adaptive scheduling (every 30min/6h/off based on repo type)
  - Reduced consumption from 18,000 to 600 min/month for private repos (97% savings)
  - Smart repository list selection based on schedule mode

- **Optimized `email-cleanup.yml`**:
  - Variable frequency: hourly (aggressive) / 6-hourly (moderate) / daily (conservative)
  - Repository type detection before execution
  - Reduced from 120 to 5 min/day for private repos

- **Optimized `copilot-meta-analysis.yml`**:
  - Schedule reduced from hourly to 6-hourly
  - Disabled schedules for private repos (event-based only)
  - Added pre-flight check to skip unnecessary runs

- **All workflows**:
  - Added `timeout-minutes` to prevent hanging jobs consuming resources
  - Repository type detection at job start
  - Smart skip logic for private repos in conservative mode

### Performance

- **Public repositories**: No change (unlimited Actions minutes) ✅
- **Private main repository**: 83% reduction (18,000 → 3,000 min/month)
- **Other private repositories**: 97% reduction (18,000 → 600 min/month) ✅
- Maintains 100% functionality via intelligent event-based triggers

### Documentation

- Added `docs/ADAPTIVE_WORKFLOWS.md` - Complete guide with:
  - System architecture and flow diagrams
  - Schedule mode details and comparisons
  - Installation instructions for new and existing projects
  - Troubleshooting guide
  - Usage examples and best practices

## [3.0.0] - 2025-12-06

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
