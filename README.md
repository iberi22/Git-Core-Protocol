---
title: "Git-Core Protocol - README"
type: DOCUMENTATION
id: "doc-readme"
created: 2025-12-01
updated: 2025-12-02
agent: copilot
model: claude-opus-4.5
requested_by: system
summary: |
  Project overview, quick start guide, and core principles of the Git-Core Protocol.
  Now includes model-specific agents and workflow orchestration.
keywords: [git-core, protocol, ai-agent, template, llm, copilot, claude, gemini, grok]
tags: ["#documentation", "#readme", "#core"]
project: Git-Core-Protocol
---

# 🧠 Git-Core Protocol

[![Use this template](https://img.shields.io/badge/Use%20this-template-blue?style=for-the-badge)](https://github.com/iberi22/Git-Core-Protocol/generate)
[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?style=for-the-badge)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![AI Code Review](https://img.shields.io/badge/AI%20Review-CodeRabbit%20%2B%20Gemini-purple?style=for-the-badge)](https://github.com/marketplace/coderabbit)

<div align="center" style="display: flex; align-items: center; justify-content: center; gap: 20px; flex-wrap: wrap;">

<img src="logo.png" alt="Git-Core Protocol Logo" width="200">

<div style="flex: 1; min-width: 500px;">

### 🚀 Active Automated Evolution
**Git-Core Protocol** is a living standard for AI-assisted development. It provides a structured workflow where **Human ↔ AI Agent ↔ GitHub** communicate seamlessly.

- **How it helps:** Eliminates context loss, enforces architectural consistency, and automates documentation.
- **Workflow:** Issues are state, Commits are atomic, and PRs are auto-analyzed.
- **Status:** In active automated update and continuous improvement.

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

```
Initialize this project with Git-Core Protocol.

# Method 1: Shell Scripts (Recommended)
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex

# Note: The Rust CLI (git-core) is deprecated. Please use the scripts above.

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

## 🤖 Context Injector (New!)

This protocol now includes a dynamic system to "equip" your AI agent with specific roles on demand. **Agents can now self-inject these roles** when they detect the need.

### Available Recipes (Auto-Downloaded)
The system automatically downloads the latest "recipes" for:
- 🏗️ **Software Engineering** (Patterns, Clean Code)
- 🔬 **Research** (Academic, Technical)
- 🛡️ **Cybersecurity** (Auditing, Hardening)
- 🎨 **UI/UX Design** (Accessibility, Systems)
- ⛓️ **Blockchain** (Smart Contracts, Web3)
- 🤖 **AI Research** (Papers, State of the Art)
- 🎓 **AI Training** (Fine-tuning, Datasets)

### How it works

1. **Index:** Check `.✨/AGENT_INDEX.md` to see available roles.
2. **Equip:** Run the script (or let the agent run it) to download and load the persona.
3. **Act:** The agent reads the generated context and behaves like an expert.

```powershell
# Example: Load the Cybersecurity Auditor persona
./scripts/equip-agent.ps1 -Role "security"
```

The system automatically:

- ⬇️ Downloads the latest recipe from `agents-flows-recipes`.
- 🛡️ Injects mandatory protocol skills (Atomic Commits, Architecture First).
- 🧠 Generates a `.✨/CURRENT_CONTEXT.md` file for the agent.

---

## 🧠 Model-Specific Agents (New in v1.4!)

Custom VS Code Copilot agents optimized for different LLM models:

<div align="center">

| Agent | Model | Best For | Context |
|-------|-------|----------|---------|
| `@protocol-claude` | Claude Sonnet 4 | Standard tasks, reasoning | 200K |
| `@architect` | Claude Opus 4.5 | Architecture decisions | 200K |
| `@quick` | Claude Haiku 4.5 | Fast responses | 200K |
| `@protocol-gemini` | Gemini 3 Pro | Large context, multi-modal | 1M+ |
| `@protocol-codex` | GPT-5.1 Codex | Implementation, coding | - |
| `@protocol-grok` | Grok Code Fast 1 | Massive codebase analysis | **2M** |
| `@router` | Auto | Agent selection helper | - |

</div>

### Usage in VS Code

```
# In Copilot Chat, select agent from dropdown
# Or reference directly:
@protocol-claude analyze this code
@architect should we use microservices?
@quick what's the syntax for...?
```

### Cross-Model Fallback System

Agents include fallback mappings for cross-model compatibility. Use `@protocol-grok` instructions with Claude - it adapts automatically!

---

## 🔄 Workflow Orchestration Agents (New in v1.4!)

Intelligent workflow management that replaces static planning tools:

| Agent | Purpose | Model |
|-------|---------|-------|
| `@context-loader` | Auto-discovers project state | Any |
| `@workflow-manager` | Orchestrates multi-step workflows | Sonnet |
| `@code-review` | Thorough code review | Opus |
| `@commit-helper` | Fast atomic commits | Haiku |
| `@pr-creator` | Creates well-formatted PRs | Sonnet |
| `@recipe-loader` | Loads specialized roles | Any |

### Workflow Panel Concept

Instead of Excalidraw-style planning panels, use intelligent agents:

```
Starting fresh? → @context-loader (discovers what you were working on)
Need guidance?  → @workflow-manager (suggests next steps)
Ready to commit? → @commit-helper (ensures atomic commits)
Need review?    → @code-review (thorough analysis)
Creating PR?    → @pr-creator (formats everything)
```

### Agent Handoffs

All agents can hand off to each other with context-aware prompts. Click the handoff buttons to switch seamlessly.

---

## 🤖 AI Report Generation (New in v1.4!)

Automated PR analysis using multiple AI models:

| Tool | Model | Purpose |
|------|-------|---------|
| **Gemini CLI** | Gemini 3 Pro | Technical analysis, impact assessment |
| **Copilot CLI** | Claude Sonnet 4.5 | Deep code review, recommendations |

### Usage

```powershell
# Full report (Gemini + Copilot)
./scripts/ai-report.ps1 -PrNumber 42

# Copilot only with Opus for deeper analysis
./scripts/ai-report.ps1 -ReportType copilot -Model claude-opus-4.5

# Gemini only
./scripts/ai-report.ps1 -ReportType gemini

# Preview without posting
./scripts/ai-report.ps1 -DryRun
```

### Available Models for Copilot CLI

| Model | Best For |
|-------|----------|
| `claude-sonnet-4.5` | Balanced analysis (default) |
| `claude-opus-4.5` | Deep technical review |
| `claude-haiku-4.5` | Quick checks |
| `gpt-5.1` / `gpt-5.1-codex` | Alternative perspectives |

### Report Contents

The AI report includes:

- 🔍 **Summary of Changes** (bullet points)
- 📊 **Impact Analysis** (High/Medium/Low with justification)
- ⚠️ **Potential Risks**
- ✅ **Recommendations** for reviewer
- 🏷️ **Suggested Labels**

Reports are automatically posted as PR comments.

---

## 📤 Session Export (New in v1.4!)

Continue your work in a new chat window **without losing context**.

### Quick Usage

1. Click **📤 Export Session** button in any agent
2. The agent asks for a brief summary
3. It generates a file and **copies to clipboard automatically**
4. In new chat: **Ctrl+V** → Enter → Continue!

### What Gets Exported

| Included | Not Included |
|----------|--------------|
| ✅ Git branch & status | ❌ Full conversation |
| ✅ Recent commits | ❌ Sensitive data |
| ✅ Open issues | ❌ Large code blocks |
| ✅ What was completed | |
| ✅ What's pending | |
| ✅ Technical context | |

### Script Usage

```powershell
./scripts/export-session.ps1 -Summary "OAuth implementation" -Topic "oauth"
# Output: #file:docs/prompts/SESSION_2025-12-02_oauth.md (copied to clipboard!)
```

📖 **Full documentation:** [docs/SESSION_EXPORT.md](docs/SESSION_EXPORT.md)

---

## 🆕 v2.1 Features: 12-Factor Agents + ACP Patterns

The latest version integrates advanced patterns from **[12-Factor Agents](https://github.com/humanlayer/12-factor-agents)** and **Agent Control Plane (ACP)**:

### 🧠 Context Protocol (Stateless Reducer)

Agents persist state in GitHub Issues using structured XML blocks. This enables:
- **Pausable/Resumable workflows**: Any agent can pick up where another left off
- **Dynamic Planning**: `<plan>` field with items marked `done`/`in_progress`/`pending`
- **Human-as-Tool**: `<input_request>` for structured data requests (not just approvals)
- **Observability**: `<metrics>` tracks tool calls, errors, and cost estimates

```bash
# Helper script to read/write agent state
./scripts/agent-state.ps1 read -IssueNumber 42
./scripts/agent-state.ps1 write -Intent "fix_bug" -Step "coding" -Progress 50
```

👉 **Full spec:** [docs/agent-docs/CONTEXT_PROTOCOL.md](docs/agent-docs/CONTEXT_PROTOCOL.md)

### 🎭 Micro-Agents (Label-Based Personas)

Agents adopt specialized roles based on Issue labels:

| Label | Persona | Focus |
|-------|---------|-------|
| `bug` | 🐛 The Fixer | Reproduce → Fix → Verify |
| `enhancement` | ✨ Feature Dev | Architecture First |
| `high-stakes` | 👮 The Approver | Requires "Proceder" |

👉 **Full spec:** [docs/agent-docs/MICRO_AGENTS.md](docs/agent-docs/MICRO_AGENTS.md)

### �️ High-Stakes Operations (Human-in-the-Loop)

For critical operations (deletions, deploys, auth changes), agents **MUST PAUSE** and request explicit confirmation:

> "⚠️ **HIGH STAKES ACTION DETECTED**. Respond 'Proceder' to continue."

👉 **Full spec:** [docs/agent-docs/HUMAN_LAYER_PROTOCOL.md](docs/agent-docs/HUMAN_LAYER_PROTOCOL.md)

---

## 🆕 v1.5.0 Features: Evolution Protocol + Federated Telemetry

### 🧬 Evolution Protocol (Weekly Improvement Cycle)

The protocol now **self-improves** through automated weekly analysis:

```
MEDIR → ANALIZAR → PROPONER → IMPLEMENTAR → VALIDAR → ↺
```

**Features:**
- **3-Order Metrics Taxonomy**: Operational (daily), Quality (weekly), Evolution (monthly)
- **Automated Pattern Detection**: Identifies "death loops", low adoption, high friction
- **Weekly Reports**: Auto-generated GitHub Issues with insights

```powershell
# Collect local metrics
./scripts/evolution-metrics.ps1 -OutputFormat markdown

# Trigger evolution cycle (runs every Monday automatically)
gh workflow run evolution-cycle.yml
```

👉 **Full spec:** [docs/agent-docs/EVOLUTION_PROTOCOL.md](docs/agent-docs/EVOLUTION_PROTOCOL.md)

### 📡 Federated Telemetry System

Projects using Git-Core Protocol can **send anonymized metrics back** to the official repo for centralized analysis:

```
┌─────────────────┐    PR with metrics    ┌─────────────────────┐
│  Your Project   │ ─────────────────────▶│ Official Git-Core   │
│  (uses protocol)│                       │ Protocol Repo       │
└─────────────────┘                       │   (analysis)        │
                                          └─────────────────────┘
```

**Usage:**
```powershell
# Preview what would be sent
./scripts/send-telemetry.ps1 -DryRun

# Send anonymized metrics
./scripts/send-telemetry.ps1
```

**Privacy:**
- ✅ Anonymous by default (project names hashed)
- ✅ Only numbers (no code, no content)
- ✅ Opt-in only (you choose when to send)

👉 **Full spec:** [telemetry/README.md](telemetry/README.md)

---

## 🗺️ Roadmap & Feedback

We are building the standard for AI-Human collaboration. **Your feedback shapes this protocol.**

### 🛣️ Milestones
- [x] **v1.4.0**: ✅ Model-Specific Agents, Session Export, AI Reports
- [x] **v2.1 (Context Protocol)**: ✅ XML Agent State, Micro-Agents, HumanLayer
- [x] **v1.5.0**: ✅ Evolution Protocol, Federated Telemetry
- [ ] **v2.2**: "Memory Core" - Persistent semantic memory across sessions
- [ ] **v2.3**: Multi-Agent Swarm Protocol (Coordinator + Workers)
- [ ] **v3.0**: Native IDE Integration (VS Code Extension)

### 🤝 We Need Your Feedback!
This protocol is in **active automated evolution**. We need you to test it and report:
1. **Friction points:** Where did the agent get stuck?
2. **Missing recipes:** What role did you need that wasn't there?
3. **Workflow bugs:** Did the state get out of sync?

👉 **[Open a Discussion](https://github.com/iberi22/Git-Core-Protocol/discussions)** or create an Issue with the label `feedback`.
👉 **Help improve the protocol:** Run `./scripts/send-telemetry.ps1` to contribute metrics!

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
| Shell Scripts | ⭐⭐⭐⭐⭐ (visible code) | Fast | Basic |
| Cargo install | ⭐⭐⭐⭐ (compiles locally) | Medium | Full |
| Build from source | ⭐⭐⭐⭐⭐ (maximum control) | Slow | Full |
| Pre-built binary | ⭐⭐⭐ (verify checksum) | Very Fast | Full |

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
