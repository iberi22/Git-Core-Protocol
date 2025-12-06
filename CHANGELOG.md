# Changelog

All notable changes to the **Git-Core Protocol** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
