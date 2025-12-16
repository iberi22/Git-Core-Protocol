---
title: "Git-Core Protocol - README"
type: DOCUMENTATION
id: "doc-readme"
created: 2025-12-01
updated: 2025-12-13
agent: copilot
model: claude-sonnet-4
requested_by: system
summary: |
  Project overview, quick start guide, and core principles of the Git-Core Protocol.
  Optimized with Rust-powered agents and simplified architecture.
keywords: [git-core, protocol, ai-agent, template, llm, copilot, claude, gemini, grok, rust]
tags: ["#documentation", "#readme", "#core", "#v3.3.0"]
project: Git-Core-Protocol
version: 3.3.0
---

# 🧠 Git-Core Protocol

[![Use this template](https://img.shields.io/badge/Use%20this-template-blue?style=for-the-badge)](https://github.com/iberi22/Git-Core-Protocol/generate)
[![Version](https://img.shields.io/badge/Version-3.3.0-green?style=for-the-badge)](https://github.com/iberi22/Git-Core-Protocol/releases/tag/v3.3.0)
[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?style=for-the-badge)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![AI Code Review](https://img.shields.io/badge/AI%20Review-CodeRabbit%20%2B%20Gemini-purple?style=for-the-badge)](https://github.com/marketplace/coderabbit)

<div align="center" style="display: flex; align-items: center; justify-content: center; gap: 20px; flex-wrap: wrap;">

<img src="logo.png" alt="Git-Core Protocol Logo" width="200">

<div style="flex: 1; min-width: 500px;">

### ⚡ Performance-First AI Development

**Git-Core Protocol** is a living standard for AI-assisted development, now powered by **Rust-native agents** for 10-30x speedup. It provides a structured workflow where **Human ↔ AI Agent ↔ GitHub** communicate seamlessly.

- **How it helps:** Eliminates context loss, enforces architectural consistency, and automates documentation at blazing speed.
- **Workflow:** Issues are state, Commits are atomic, PRs are auto-analyzed, and agents work in sub-second time.
- **Status:** Production-ready with self-healing CI/CD and zero-overhead state management.

| Git-Core Protocol | Compliance | Detail                                      |
|-------------------|------------|---------------------------------------------|
| Defined rules     | ✅          | AGENTS.md, copilot-instructions.md           |
| Syntax            | ✅          | Issue format, conventional commits           |
| Semantics         | ✅          | Labels, states, workflows                    |
| Synchronization   | ✅          | GitHub Issues as state, workflows            |

</div>

</div>

---

## 🌍 Language | Idioma | Língua | Sprache | Langue | 言語 | 语言

<div align="center">

| Flag | Language | Link |
|:----:|----------|------|
| 🇺🇸 | **English** | [Jump to English](#-english) |
| 🇪🇸 | **Español** | [Ir a Español](#-español) |
| 🇧🇷 | **Português** | [Ir para Português](#-português) |
| 🇩🇪 | **Deutsch** | [Zu Deutsch](#-deutsch) |
| 🇫🇷 | **Français** | [Aller au Français](#-français) |
| 🇯🇵 | **日本語** | [日本語へ](#-日本語) |
| 🇨🇳 | **中文** | [跳转到中文](#-中文) |

</div>

---

# 🇺🇸 English

## 🚀 Quick Start - Just Copy This Prompt

> **Copy this prompt to your AI coding agent (Copilot, Cursor, Claude, etc.) to auto-setup:**

Initialize this project with Git-Core Protocol.

# Method 1: Rust CLI (Recommended)
 cargo install --path . # If local
 # OR
 gc init

# Method 2: Shell Scripts (Legacy)
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex

After setup, read .✨/ARCHITECTURE.md and start with: gh issue list
```

**For AI Agents (auto mode):**

For Windows:

```
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
Then execute: .\scripts\init_project.ps1 -Auto
```

For Linux/Mac:

```
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
Then execute: ./scripts/init_project.sh
```

---

## ✨ Features

### 🤖 Context Injector

Dynamic system to "equip" your AI agent with specialized roles on demand.

**Available Roles:**
- 🏗️ Software Engineering | 🔬 Research | 🛡️ Cybersecurity
- 🎨 UI/UX Design | ⛓️ Blockchain | 🤖 AI Research | 🎓 AI Training

```powershell
# Load a specialized persona
./scripts/equip-agent.ps1 -Role "security"
```

---

### 🧠 Model-Specific Agents

> ⚠️ **VS Code Only** - Requires GitHub Copilot Chat extension

Custom Copilot agents optimized for different LLM models:

| Agent | Model | Best For |
|-------|-------|----------|
| `@protocol-claude` | Claude Sonnet 4 | Standard tasks |
| `@architect` | Claude Opus 4.5 | Architecture decisions |
| `@quick` | Claude Haiku 4.5 | Fast responses |
| `@protocol-gemini` | Gemini 3 Pro | Large context (1M+) |
| `@protocol-grok` | Grok Code Fast 1 | Massive analysis (2M) |

---

### 🔄 Workflow Agents

> ⚠️ **VS Code Only** - Requires GitHub Copilot Chat extension

| Agent | Purpose |
|-------|---------|
| `@context-loader` | Auto-discovers project state |
| `@workflow-manager` | Orchestrates multi-step workflows |
| `@commit-helper` | Fast atomic commits |
| `@pr-creator` | Creates well-formatted PRs |

---

---

### 📊 AI Report Generation

Automated PR analysis using Gemini or Copilot CLI:

```powershell
./scripts/ai-report.ps1 -PrNumber 42          # Full report
./scripts/ai-report.ps1 -ReportType gemini    # Gemini only
./scripts/ai-report.ps1 -DryRun               # Preview
```

---

### 📤 Session Export

Continue work in a new chat **without losing context**:

```powershell
./scripts/export-session.ps1 -Summary "OAuth implementation" -Topic "oauth"
```

Exports: Git status, recent commits, open issues, and pending tasks.

---

---

### 🎯 Adaptive Workflows

Automatic detection of repository type (public/private) to optimize GitHub Actions:

| Mode | Schedules | Cost |
|------|-----------|------|
| 🟢 **Public** | Every 6 hours | $0 (unlimited) |
| 🟡 **Private** | Event-based only | Free tier |

---

### 🧠 Agent State Protocol

Agents persist state in GitHub Issues using XML blocks for pausable/resumable workflows:

```bash
./scripts/agent-state.ps1 read -IssueNumber 42
```

---

### 🛡️ High-Stakes Operations

Critical operations (deletions, deploys) require explicit confirmation:

> ⚠️ **HIGH STAKES ACTION DETECTED**. Respond 'Proceder' to continue.

---

### 🧬 Self-Healing CI/CD

Automated error classification and retry for transient failures:
- **Transient errors** (timeouts, rate limits): Auto-retry
- **Dependency errors**: Creates issue for review
- **Test failures**: Creates issue with diagnosis

---

### 📦 Dependency Quarantine

14-day quarantine for new dependencies with AI analysis before adoption.

📖 **Full changelog:** [CHANGELOG.md](CHANGELOG.md)

---

## 🤝 Feedback

This protocol evolves through user feedback. Report:

- **Friction points**: Where did the agent get stuck?
- **Missing recipes**: What role did you need?
- **Workflow bugs**: Did state get out of sync?

👉 **[Open a Discussion](https://github.com/iberi22/Git-Core-Protocol/discussions)** or create an Issue with `feedback` label.

---

## Why This Approach?

| Problem | Git-Core Solution |
|---------|-------------------|
| AI "forgets" task state | State in GitHub Issues (persistent) |
| Context grows = more tokens = more cost | Only load current issue + architecture |
| Messy TODO.md files | Organized GitHub board |
| Ecosystem dependency (NPM, etc.) | Language-agnostic bash/PowerShell scripts |

## 📦 Installation Options

**🔐 Trust & Transparency:** Before installing, read [docs/CLI_TRUST.md](docs/CLI_TRUST.md) to understand exactly what each method does.

### Option 1: Shell Scripts (🚀 Transparent - Recommended)

Scripts are **visible code** you can read before running:

```bash
# View the code BEFORE running:
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh

# Linux/macOS - If you trust it, run:
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows - View code first:
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1" | Select-Object -ExpandProperty Content

# Windows - Then run:
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

### Option 2: Git-Core CLI (🦀 Full Features)

The official CLI provides the best management experience:

```bash
# 🦀 Cargo (compiles from source on YOUR machine)
# Before installing, read: docs/CLI_TRUST.md
# Source code: https://github.com/iberi22/Git-Core-Protocol/tree/main/tools/git-core-cli
cargo install git-core-cli

# 🔨 Or build from source (maximum trust)
git clone https://github.com/iberi22/Git-Core-Protocol
cd Git-Core-Protocol/tools/git-core-cli
cargo build --release
./target/release/git-core install
```

**CLI Commands:**

```bash
# Install protocol in current project
git-core install

# Initialize a new project
git-core init my-project

# Upgrade existing installation
git-core upgrade

# Check protocol integrity
git-core check

# Migrate from .ai/ to .✨/
git-core migrate
```

### Option 3: Use as Template

1. Click **"Use this template"** above
2. Clone your new repository
3. Run: `curl -fsSL .../install.sh | bash` or `git-core install`

**Method Comparison:**

<div align="center">

| Method | Trust Level | Speed | Features |
|--------|-------------|-------|----------|
| **Rust CLI** | ⭐⭐⭐⭐ (compiled) | **Fastest** | **Integrated** |
| Shell Scripts | ⭐⭐⭐⭐⭐ (visible code) | Fast | Basic |
| Cargo install | ⭐⭐⭐⭐ (compiles locally) | Medium | Full |
| Build from source | ⭐⭐⭐⭐⭐ (maximum control) | Slow | Full |

</div>

## 📂 Structure

```
/
├── .✨/
│   ├── ARCHITECTURE.md       # 📖 System context
│   ├── AGENT_INDEX.md        # 🎭 Agent roles and routing
│   └── CONTEXT_LOG.md        # 📝 Ephemeral session notes
├── .github/
│   ├── agents/               # 🤖 Model-specific agents (NEW!)
│   │   ├── protocol-claude.agent.md
│   │   ├── protocol-gemini.agent.md
│   │   ├── protocol-codex.agent.md
│   │   ├── protocol-grok.agent.md
│   │   ├── architect.agent.md
│   │   ├── quick.agent.md
│   │   ├── router.agent.md
│   │   └── workflow-*.agent.md  # Workflow agents
│   ├── instructions/         # 📋 Model-specific instructions
│   │   ├── claude-tools.instructions.md
│   │   ├── gemini-tools.instructions.md
│   │   ├── codex-tools.instructions.md
│   │   ├── grok-tools.instructions.md
│   │   └── fallback-system.instructions.md
│   ├── copilot-instructions.md  # 🤖 GitHub Copilot rules
│   └── ISSUE_TEMPLATE/       # 📋 Issue templates
├── scripts/
│   ├── init_project.sh       # 🐧 Linux/Mac initializer
│   ├── init_project.ps1      # 🪟 Windows initializer
│   ├── equip-agent.ps1       # 🎭 Recipe loader (Windows)
│   ├── equip-agent.sh        # 🎭 Recipe loader (Linux/Mac)
│   ├── install-cli.sh        # 🛠️ CLI installer (Linux/macOS)
│   └── install-cli.ps1       # 🛠️ CLI installer (Windows)
├── tools/
│   └── git-core-cli/         # 🦀 Official Rust CLI source
├── AGENTS.md                 # 🤖 All AI agents config
├── .cursorrules              # 🎯 Cursor rules
└── .windsurfrules            # 🌊 Windsurf rules
```

## 🔄 The Workflow Loop

```
┌─────────────────────────────────────────────────────────┐
│                    THE LOOP                              │
├─────────────────────────────────────────────────────────┤
│   1. READ: cat .✨/ARCHITECTURE.md                      │
│           gh issue list --assignee "@me"                │
│   2. ACT:  gh issue edit <id> --add-assignee "@me"      │
│           git checkout -b feat/issue-<id>               │
│   3. UPDATE: git commit -m "feat: ... (closes #<id>)"   │
│             gh pr create --fill                         │
│   ↺ Repeat                                               │
└─────────────────────────────────────────────────────────┘
```

## 📊 Issue Lifecycle & Progress Tracking

**Issues stay OPEN** while they have pending tasks. They **close automatically** when a commit includes `closes #X`.

```
┌─────────────────────────────────────────────────────────┐
│  OPEN                                                   │
│  ├── 📋 Backlog: No one assigned, waiting               │
│  ├── 🔄 In Progress: Someone assigned, working          │
│  └── ⏸️ Blocked: Waiting for dependency                 │
└─────────────────────────────────────────────────────────┘
                         │
                         │ Commit with "closes #X"
                         ▼
┌─────────────────────────────────────────────────────────┐
│  CLOSED                                                 │
│  └── ✅ Completed: All tasks done                       │
└─────────────────────────────────────────────────────────┘
```

**Progress Tracking:** Use an **EPIC issue** with checkboxes to track overall progress. GitHub automatically calculates the percentage. No local files needed!

```markdown
# Example EPIC Issue
- [x] Task 1 completed
- [x] Task 2 completed
- [ ] Task 3 pending
- [ ] Task 4 pending
# GitHub shows: ██████░░░░ 50%
```

## 🤖 Compatible AI Agents

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT | ✅ Any LLM with terminal access

## 🤝 Credits & Inspiration

This protocol is inspired by and builds upon the excellent work of:

- **[HumanLayer](https://github.com/humanlayer/humanlayer)**: For their pioneering work on "12-Factor Agents" and "Context Engineering".
- **[CodeLayer](https://humanlayer.dev/code)**: For demonstrating advanced agent orchestration.
- **Context7**: For the initial concepts of context management.
- **[Git](https://git-scm.com/)**: To be free to use.
- **[GitHub](https://github.com/)**: for shered the infrastructure, and all his community of developers.
- **[anthropic](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents/)**:

We acknowledge their contributions to the field of AI-assisted development.
