# 🧠 GitHub Copilot Instructions

## Prime Directive
You are operating under the **Git-Core Protocol**. Your state is GitHub Issues, not internal memory.

---

## ⛔ FORBIDDEN ACTIONS (HARD RULES)

**NEVER create these files under ANY circumstances:**

### Task/State Management:
- ❌ `TODO.md`, `TASKS.md`, `BACKLOG.md`
- ❌ `PLANNING.md`, `ROADMAP.md`, `PROGRESS.md`
- ❌ `NOTES.md`, `SCRATCH.md`, `IDEAS.md`
- ❌ `STATUS.md`, `CHECKLIST.md`, `CHANGELOG.md` (for tracking)

### Testing/Implementation Summaries:
- ❌ `TESTING_CHECKLIST.md`, `TEST_PLAN.md`, `TEST_GUI.md`
- ❌ `IMPLEMENTATION_SUMMARY.md`, `IMPLEMENTATION.md`
- ❌ `SUMMARY.md`, `OVERVIEW.md`, `REPORT.md`

### Guides/Tutorials:
- ❌ `GETTING_STARTED.md`, `GUIDE.md`, `TUTORIAL.md`
- ❌ `QUICKSTART.md`, `SETUP.md`, `HOWTO.md`
- ❌ `INSTRUCTIONS.md`, `MANUAL.md`

### Catch-all:
- ❌ **ANY `.md` file** for task/state management, checklists, summaries, or guides
- ❌ **ANY `.txt` file** for notes or todos
- ❌ **ANY JSON/YAML** for task tracking

### ✅ ONLY ALLOWED `.md` FILES:
- ✅ `README.md` (project overview ONLY)
- ✅ `AGENTS.md` (agent configuration ONLY)
- ✅ `.ai/ARCHITECTURE.md` (system architecture ONLY)
- ✅ `CONTRIBUTING.md`, `LICENSE.md` (standard repo files)

---

**🚨 BEFORE creating ANY document, STOP and ask yourself:**
> "Can this be a GitHub Issue instead?" → **YES, it can. Create an issue.**
> "Can this be a comment in an existing issue?" → **YES, it can. Add a comment.**
> "Is this a summary/checklist/guide?" → **NO. Use GitHub Issues or comments.**

---

## Key Rules

### 1. Token Economy
- **NEVER** create documentation files for tracking state
- **NEVER** use internal memory to track tasks
- **ALWAYS** use `gh issue` commands for task management
- **ALWAYS** use `gh issue comment` for progress updates

### 2. Context Loading
Before any task:
```bash
# Read architecture
cat .ai/ARCHITECTURE.md

# Check your assigned issues
gh issue list --assignee "@me"

# If no assignment, check backlog
gh issue list --limit 5
```

### 3. Development Flow
```bash
# Take a task
gh issue edit <id> --add-assignee "@me"

# Create branch
git checkout -b feat/issue-<id>

# After coding, commit with reference
git commit -m "feat: description (closes #<id>)"

# Create PR
gh pr create --fill
```

### 4. Planning Mode
When asked to plan, generate `gh issue create` commands instead of documents:
```bash
gh issue create --title "TASK: Description" --body "Details..." --label "ai-plan"
```

**❌ WRONG:** Creating a `PLAN.md` or `ROADMAP.md` file
**✅ RIGHT:** Running multiple `gh issue create` commands

### 5. Progress Updates
When you need to document progress:
```bash
# Add comment to existing issue
gh issue comment <id> --body "Progress: Completed X, working on Y"
```

**❌ WRONG:** Creating `PROGRESS.md` or updating a tracking file
**✅ RIGHT:** Adding comments to the relevant GitHub Issue

### 6. User-Requested Documents (agent-docs)

When the user **explicitly requests** a document (prompt, research, strategy, etc.):

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

### 7. YAML Frontmatter Meta Tags (REQUIRED for agent-docs)

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

### 8. Extended Commit Messages

Use AI-Context section for complex changes:

```text
feat(auth): implement OAuth2 login #42

Adds OAuth2 authentication with Google and GitHub providers.
Includes refresh token rotation and session management.

AI-Context: Uses passport.js. Config in src/config/auth.ts.
Test credentials in .env.example.

Closes #42
```

### 9. Code Standards
- Follow existing code style
- Write tests for new features
- Use Conventional Commits (see docs/COMMIT_STANDARD.md)
- Keep PRs focused and small

### 10. Communication
- Be concise in commit messages
- Reference issues in all commits
- Use AI-Context for complex changes
- Update issue comments for significant progress

### 11. Codex CLI Integration

**Installation:**
```bash
npm i -g @openai/codex
export OPENAI_API_KEY=your-api-key
```

**Usage modes:**
```bash
codex                      # Interactive mode
codex "explain this code"  # Quick query
codex exec "..."           # Headless automation
```

**Trigger via GitHub:**
- Add label `codex-review` to PR for automated review
- Comment `/codex-review` for on-demand review
- Comment `/codex-analyze` for codebase analysis

### 12. Copilot Coding Agent (Jules)

**Assign issues to Copilot:**
```bash
gh issue edit <number> --add-assignee "Copilot"
```

**Trigger in PRs:**
- Mention `@copilot` in PR comments for specific tasks
- Copilot creates branches named `copilot/*`

**Monitor Copilot work:**
```bash
gh pr list --head "copilot/"
```

