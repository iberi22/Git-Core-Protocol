# 🤖 AGENTS.md - AI Agent Configuration

## Overview
This repository follows the **Git-Core Protocol** for AI-assisted development.

---

## ⛔ FORBIDDEN FILES (HARD RULES)

**NEVER create these files under ANY circumstances:**

### Task/State Management:
```
❌ TODO.md, TASKS.md, BACKLOG.md
❌ PLANNING.md, ROADMAP.md, PROGRESS.md
❌ NOTES.md, SCRATCH.md, IDEAS.md
❌ STATUS.md, CHECKLIST.md, CHANGELOG.md (for tracking)
```

### Testing/Implementation Summaries:
```
❌ TESTING_CHECKLIST.md, TEST_PLAN.md, TEST_GUI.md
❌ IMPLEMENTATION_SUMMARY.md, IMPLEMENTATION.md
❌ SUMMARY.md, OVERVIEW.md, REPORT.md
```

### Guides/Tutorials:
```
❌ GETTING_STARTED.md, GUIDE.md, TUTORIAL.md
❌ QUICKSTART.md, SETUP.md, HOWTO.md
❌ INSTRUCTIONS.md, MANUAL.md
```

### Catch-all:
```
❌ ANY .md file for task/state management
❌ ANY .md file for checklists or summaries
❌ ANY .md file for guides or tutorials
❌ ANY .txt file for notes or todos
❌ ANY JSON/YAML for task tracking
```

### ✅ ONLY ALLOWED `.md` FILES:
```
✅ README.md (project overview ONLY)
✅ AGENTS.md (agent configuration ONLY)
✅ .ai/ARCHITECTURE.md (system architecture ONLY)
✅ CONTRIBUTING.md, LICENSE.md (standard repo files)
```

**🚨 STOP! Before creating ANY document, ask yourself:**
> "Can this be a GitHub Issue?" → **YES. Always yes. Create an issue.**
> "Can this be a comment in an existing issue?" → **YES. Add a comment.**
> "Is this a summary/checklist/guide?" → **NO. Use GitHub Issues or comments.**

---

## For All AI Agents (Copilot, Cursor, Windsurf, Claude, etc.)

### 🎯 Prime Directive: Token Economy
```
Your state is GitHub Issues. Not memory. Not files. GitHub Issues.
```

### 📖 Required Reading Before Any Task
1. `.ai/ARCHITECTURE.md` - Understand the system
2. `gh issue list --assignee "@me"` - Your current task
3. `gh issue list --limit 5` - Available backlog

---

## 🔄 The Loop (Workflow)

### Phase 1: READ (Context Loading)
```bash
# Always start here
cat .ai/ARCHITECTURE.md
gh issue list --assignee "@me" --state open
```

### Phase 2: ACT (Development)
```bash
# Claim a task
gh issue edit <ISSUE_NUMBER> --add-assignee "@me"

# Create feature branch
git checkout -b feat/issue-<ISSUE_NUMBER>

# Write code + tests
# ...

# Commit with Conventional Commits
git add .
git commit -m "feat(scope): description (closes #<ISSUE_NUMBER>)"
```

### Phase 3: UPDATE (Close the Loop)
```bash
# Push and create PR
git push -u origin HEAD
gh pr create --fill --base main

# DO NOT manually close issues - let Git do it via commit message
```

---

## 🚫 Anti-Patterns (NEVER DO THIS)

| ❌ Don't | ✅ Do Instead |
|----------|---------------|
| Create TODO.md files | Use `gh issue create` |
| Create PLANNING.md | Use `gh issue create` with label `ai-plan` |
| Create PROGRESS.md | Use `gh issue comment <id> --body "..."` |
| Create NOTES.md | Add notes to relevant issue comments |
| Track tasks in memory | Query `gh issue list` |
| Write long planning docs | Create multiple focused issues |
| Forget issue references | Always include `#<number>` in commits |
| Close issues manually | Use `closes #X` in commit message |
| Create any .md for tracking | **ALWAYS use GitHub Issues** |

---

## ✅ What You CAN Create

| ✅ Allowed | Purpose |
|------------|----------|
| Source code (`.py`, `.js`, `.ts`, etc.) | The actual project |
| Tests (in `tests/` folder) | Quality assurance |
| Config files (docker, CI/CD, linters) | Infrastructure |
| `.ai/ARCHITECTURE.md` | System architecture (ONLY this file) |
| `README.md` | Project documentation |
| `docs/agent-docs/*.md` | **ONLY when user explicitly requests** |
| GitHub Issues | **EVERYTHING ELSE** |

---

## 📄 User-Requested Documentation (agent-docs)

When the user **explicitly requests** a persistent document (prompt, research, strategy, etc.):

```bash
# Create in docs/agent-docs/ with proper prefix
# Prefixes: PROMPT_, RESEARCH_, STRATEGY_, SPEC_, GUIDE_, REPORT_, ANALYSIS_

# Example: User says "Create a prompt for Jules"
docs/agent-docs/PROMPT_JULES_AUTH_SYSTEM.md

# Commit with docs(agent) scope
git commit -m "docs(agent): add PROMPT for Jules auth implementation"
```

**✅ ONLY create files when user says:**

- "Save this as a document"
- "Create a prompt file for..."
- "Document this strategy"
- "Write a spec for..."
- "I need this as a reference"

**❌ DO NOT create files, just respond in chat:**

- "Explain how to..."
- "Summarize this..."
- "What's the best approach..."

---

## 🏷️ YAML Frontmatter Meta Tags (REQUIRED for agent-docs)

When creating documents in `docs/agent-docs/`, **ALWAYS** include YAML frontmatter for rapid AI scanning:

```yaml
---
title: "Authentication System Prompt"
type: PROMPT
id: "prompt-jules-auth"
created: 2025-11-29
updated: 2025-11-29
agent: copilot
model: claude-opus-4
requested_by: user
summary: |
  Prompt for Jules to implement OAuth2 authentication
  with Google and GitHub providers.
keywords: [oauth, auth, jules, security]
tags: ["#auth", "#security", "#jules"]
topics: [authentication, ai-agents]
related_issues: ["#42"]
project: my-project
module: auth
language: typescript
priority: high
status: approved
confidence: 0.92
token_estimate: 800
complexity: moderate
---
```

**Why?** AI agents can read metadata without parsing entire documents. See `docs/agent-docs/README.md` for full spec.

---

## 📝 Commit Standard

Follow Extended Conventional Commits (see `docs/COMMIT_STANDARD.md`):

```text
<type>(<scope>): <description> #<issue>

[optional body]

[optional AI-Context footer]
```

**AI-Context Footer** (for complex decisions):

```text
AI-Context: architecture | Chose event-driven over REST for real-time requirements
AI-Context: trade-off | Sacrificed DRY for performance in hot path
AI-Context: dependency | Selected library X over Y due to bundle size
```

---

## 📋 Planning Mode

When asked to plan a feature, output executable commands:

```bash
# Example: Planning a user authentication feature
gh issue create --title "SETUP: Configure auth library" \
  --body "Install and configure authentication package" \
  --label "ai-plan"

gh issue create --title "FEAT: Implement login endpoint" \
  --body "Create POST /auth/login with JWT" \
  --label "ai-plan"

gh issue create --title "FEAT: Implement logout endpoint" \
  --body "Create POST /auth/logout" \
  --label "ai-plan"

gh issue create --title "TEST: Auth integration tests" \
  --body "Write e2e tests for auth flow" \
  --label "ai-plan"
```

---

## 🏷️ Label System

| Label | Purpose | Color |
|-------|---------|-------|
| `ai-plan` | High-level planning tasks | 🟢 Green |
| `ai-context` | Critical context information | 🟡 Yellow |
| `bug` | Bug reports | 🔴 Red |
| `enhancement` | Feature requests | 🔵 Blue |
| `blocked` | Waiting on dependencies | ⚫ Gray |
| `codex-review` | Trigger Codex AI review | 🟣 Purple |
| `copilot` | Assigned to GitHub Copilot Agent | 🔵 Blue |
| `jules` | Assigned to Google Jules Agent | 🟠 Orange |

---

## 🤖 AI Coding Agents (Copilot & Jules)

This protocol supports **two autonomous coding agents** that can work on issues and PRs:

| Agent | Provider | Trigger | Branch Pattern |
|-------|----------|---------|----------------|
| **Copilot** | GitHub/Microsoft | Label `copilot` or assign "Copilot" | `copilot/*` |
| **Jules** | Google | Label `jules` (case insensitive) | Creates PR directly |

### Choosing an Agent

You can assign tasks to either agent based on preference or workload:

```bash
# Assign to GitHub Copilot
gh issue edit <number> --add-label "copilot"
# OR
gh issue edit <number> --add-assignee "Copilot"

# Assign to Google Jules  
gh issue edit <number> --add-label "jules"
```

### Load Balancing (Auto-Distribution)

Use the workflow `.github/workflows/agent-dispatcher.yml` to automatically distribute issues:

```bash
# Manual trigger - dispatches unassigned issues to available agents
gh workflow run agent-dispatcher.yml

# Or add label to auto-dispatch
gh issue edit <number> --add-label "ai-agent"
```

---

### GitHub Copilot Coding Agent

GitHub's autonomous coding agent that works directly on your repository.

**Trigger Methods:**

```bash
# Method 1: Add label
gh issue edit <number> --add-label "copilot"

# Method 2: Assign directly
gh issue edit <number> --add-assignee "Copilot"

# Method 3: Mention in PR comments
# Comment "@copilot fix this bug" in any PR
```

**Monitor Copilot:**

```bash
# List Copilot branches
gh pr list --head "copilot/"

# Check Copilot's work
gh pr view <number>
```

**Environment Setup:**
The repository includes `.github/copilot-setup-steps.yml` for Copilot sessions.

---

### Google Jules Coding Agent

Google's asynchronous coding agent with CLI and GitHub integration.

**Installation:**

```bash
# Install Jules CLI
# Visit: https://jules.google/docs/

# Login
jules login
```

**Trigger Methods:**

```bash
# Method 1: Add label "jules" to any issue (GitHub App required)
gh issue edit <number> --add-label "jules"

# Method 2: CLI - Create session from issue
gh issue list --assignee @me --limit 1 --json title | jq -r '.[0].title' | jules new

# Method 3: CLI - Direct task
jules new "implement user authentication"

# Method 4: CLI - Specific repo
jules new --repo owner/repo "write unit tests"
```

**CLI Commands:**

```bash
# Launch TUI
jules

# Create new session
jules new "task description"

# Parallel sessions (same task, multiple approaches)
jules new --parallel 3 "optimize database queries"

# List sessions
jules remote list --session

# Pull results
jules remote pull --session <id>

# Pull and apply patch
jules remote pull --session <id> --apply
```

**Monitor Jules:**

```bash
# List all sessions
jules remote list --session

# Check specific session
jules remote pull --session <id>
```

**Jules Comments:** When Jules finishes, it comments on the issue with a link to the PR.

---

### Advanced: Batch Processing with Jules CLI

```bash
# Process multiple issues automatically
gh issue list --label "jules" --json number,title | jq -r '.[] | "\(.number): \(.title)"' | while read line; do
  jules new "$line"
done

# Use Gemini to prioritize issues for Jules
gemini -p "find the most tedious issue:\n$(gh issue list --assignee @me)" | jules new
```

---

## 🔄 Codex CLI - Code Review Automation

Codex CLI enables AI-powered code reviews and analysis.

**Installation:**

```bash
npm i -g @openai/codex
export OPENAI_API_KEY=your-api-key
```

**Usage:**

```bash
codex                      # Interactive mode
codex "explain this code"  # Quick query
codex exec "..."           # Headless automation
```

**GitHub Triggers:**

- Add label `codex-review` → automated PR review
- Comment `/codex-review` → on-demand review
- Comment `/codex-analyze` → codebase analysis
- Comment `/codex-fix` → auto-fix suggestions

---

## 🔧 Useful Commands Reference

```bash
# View issues
gh issue list
gh issue list --label "ai-plan"
gh issue view <number>

# Create issues
gh issue create --title "..." --body "..." --label "..."

# Update issues
gh issue edit <number> --add-assignee "@me"
gh issue edit <number> --add-label "in-progress"
gh issue comment <number> --body "Progress update..."

# PRs
gh pr create --fill
gh pr list
gh pr merge <number>
```

---

## 📁 Project Structure Awareness

```text
/
├── .ai/
│   ├── ARCHITECTURE.md    # 📖 READ THIS FIRST
│   └── CONTEXT_LOG.md     # 📝 Session notes only
├── .github/
│   ├── copilot-instructions.md
│   ├── workflows/         # 🔄 CI/CD automation
│   └── ISSUE_TEMPLATE/
├── docs/
│   ├── agent-docs/        # 📄 User-requested documents ONLY
│   └── COMMIT_STANDARD.md # 📝 Commit message standard
├── scripts/
│   └── init_project.sh    # 🚀 Bootstrap script
├── AGENTS.md              # 📋 YOU ARE HERE
└── .cursorrules           # 🎯 Editor rules
```

---

*Protocol Version: 1.1.0*
*Last Updated: 2025*
