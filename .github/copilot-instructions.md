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
- ✅ `.✨/ARCHITECTURE.md` (system architecture ONLY)
- ✅ `.✨/AGENT_INDEX.md` (agent routing ONLY)
- ✅ `CONTRIBUTING.md`, `LICENSE.md` (standard repo files)

---

**🚨 BEFORE creating ANY document, STOP and ask yourself:**
> "Can this be a GitHub Issue instead?" → **YES, it can. Create an issue.**
> "Can this be a comment in an existing issue?" → **YES, it can. Add a comment.**
> "Is this a summary/checklist/guide?" → **NO. Use GitHub Issues or comments.**

---

## 🎯 Intent Detection - Issue Creation Flow

**When user says ANY of these (or similar), trigger the Issue Creation Flow:**

| User Intent | Trigger Phrases | Action |
|-------------|-----------------|--------|
| New task | "necesito", "hay que", "deberíamos", "want to", "need to" | → Create issue file |
| Bug report | "bug", "error", "falla", "no funciona", "broken" | → Create `BUG_*.md` |
| Feature request | "agregar", "nueva funcionalidad", "add feature" | → Create `FEAT_*.md` |
| Planning | "planear", "plan", "roadmap", "siguiente paso" | → Create multiple issue files |
| Task tracking | "tarea", "task", "todo", "pendiente" | → Create `TASK_*.md` |

### Issue Creation Flow (MANDATORY)

When you detect issue-creation intent:

```
1. ANNOUNCE: "Voy a crear un issue para trackear esto."

2. ASK (if unclear):
   - "¿Qué título le ponemos al issue?"
   - "¿Alguna etiqueta específica? (bug, enhancement, ai-plan)"

3. CREATE FILE in `.github/issues/`:
   - Format: `{TYPE}_{descripcion-corta}.md`
   - Types: FEAT, BUG, TASK, DOCS, REFACTOR, TEST, CHORE

4. SHOW the user what you created:
   "✅ Issue creado: `.github/issues/FEAT_nueva-funcionalidad.md`
    Se sincronizará con GitHub automáticamente en el próximo push,
    o ejecuta: `./scripts/sync-issues.ps1`"
```

### Example Interaction

**User:** "Necesito agregar autenticación OAuth"

**Agent Response:**
```
Voy a crear un issue para trackear esto.

📝 Creando: `.github/issues/FEAT_oauth-authentication.md`

---
title: "Implementar autenticación OAuth"
labels:
  - enhancement
  - auth
assignees: []
---

## Descripción
Agregar autenticación OAuth al sistema.

## Tareas
- [ ] Configurar proveedor OAuth
- [ ] Implementar flujo de login
- [ ] Agregar tests

---

✅ Issue creado. Se sincronizará automáticamente o ejecuta:
`./scripts/sync-issues.ps1`

¿Quieres que empiece a trabajar en esto ahora?
```

### DO NOT Skip This Flow

❌ **WRONG:** Start coding immediately without creating an issue
❌ **WRONG:** Create a TODO.md or TASKS.md file
❌ **WRONG:** Just acknowledge and forget

✅ **RIGHT:** Always create an issue file first, then work on it

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
# 1. Check the Agent Index to see if you need a specific Role
cat .✨/AGENT_INDEX.md

# 2. Read Living Research Context (CRITICAL for dependencies)
cat docs/agent-docs/RESEARCH_STACK_CONTEXT.md

# 3. If a Role fits the task, EQUIP IT:
# ./scripts/equip-agent.ps1 -Role "RoleName"
# cat .✨/CURRENT_CONTEXT.md

# 4. Read architecture
cat .✨/ARCHITECTURE.md

# 5. Check your assigned issues
gh issue list --assignee "@me"
```

### 3. Dependency Quarantine Rule (NEW)
When working with dependencies:
1. **Check quarantine status**: `gh pr list --label "quarantine"`
2. **Never use bleeding-edge**: Versions < 2 weeks old are in quarantine
3. **Check RESEARCH_STACK_CONTEXT.md**: Contains known issues for current versions
4. **Prefer stable patterns**: Use the "Intelligent Patterns" section, not latest docs

**Workflow Integration:**
- Dependabot creates PRs → Auto-labeled "quarantine"
- After 14 days → Auto-promoted to "ready-to-adopt"
- On merge → Context Research Agent updates documentation

### 3.1 Living Context Protocol (Context7 Replacement)

This protocol replaces external tools like Context7 with a native, self-hosted solution.

**How it works:**
1. **Context Research Agent** analyzes dependencies and creates `RESEARCH_STACK_CONTEXT.md`
2. **Living Context PR** is always open at `living-context/main` branch
3. **AI Agents** (Gemini, CodeRabbit) automatically review the context for accuracy
4. **Merge** updates the context, which all AI agents should read

**Usage for AI Agents:**
```bash
# ALWAYS read this before working with dependencies
cat docs/agent-docs/RESEARCH_STACK_CONTEXT.md

# Check for the Living Context PR (latest updates)
gh pr list --head "living-context/main"

# If context seems outdated, trigger refresh
gh workflow run living-context.yml
```

**What's in RESEARCH_STACK_CONTEXT.md:**
- **Current Stack**: All dependencies with exact versions
- **Known Anomalies**: Bugs, quirks, breaking changes
- **Intelligent Patterns**: Recommended usage patterns
- **Quarantine Status**: Dependencies awaiting community feedback

**Why this beats Context7:**
| Context7 | Living Context Protocol |
|----------|------------------------|
| Generic docs | Project-specific context |
| External API | Self-hosted in repo |
| Manual trigger | Automated via workflows |
| No risk analysis | 14-day quarantine + AI review |

### 4. Architecture First Rule
Before implementing ANY infrastructure feature:
1. Run: `grep -A 20 'CRITICAL DECISIONS' .✨/ARCHITECTURE.md`
2. Check CRITICAL DECISIONS table
3. If conflict with issue, ARCHITECTURE wins

**Why this matters:** A critical error occurred when Vercel was implemented despite ARCHITECTURE.md specifying GitHub Pages. Issues may mention multiple options, but architecture decisions are final.

**Related Documentation:**
- `.✨/ARCHITECTURE.md` - CRITICAL DECISIONS table
- `.✨/AGENT_INDEX.md` - Agent roles and routing
- `AGENTS.md` - Architecture Verification Rule

### 5. Development Flow
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

### 5. Planning Mode
When asked to plan, generate `gh issue create` commands instead of documents:
```bash
gh issue create --title "TASK: Description" --body "Details..." --label "ai-plan"
```

**❌ WRONG:** Creating a `PLAN.md` or `ROADMAP.md` file
**✅ RIGHT:** Running multiple `gh issue create` commands

**ALTERNATIVE (RECOMMENDED):** Create issue files in `.github/issues/`:
```bash
# Create a file instead of running gh issue create
# File: .github/issues/FEAT_mi-feature.md

---
title: "My Feature Description"
labels:
  - ai-plan
  - enhancement
---

## Description
Details here...
```

The workflow `sync-issues.yml` will automatically create the GitHub Issue.

### 6. Progress Updates
When you need to document progress:
```bash
# Add comment to existing issue
gh issue comment <id> --body "Progress: Completed X, working on Y"
```

**❌ WRONG:** Creating `PROGRESS.md` or updating a tracking file
**✅ RIGHT:** Adding comments to the relevant GitHub Issue

### 7. User-Requested Documents (agent-docs)

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

### 8. YAML Frontmatter Meta Tags (REQUIRED for agent-docs)

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

### 9. Extended Commit Messages

Use AI-Context section for complex changes:

```text
feat(auth): implement OAuth2 login #42

Adds OAuth2 authentication with Google and GitHub providers.
Includes refresh token rotation and session management.

AI-Context: Uses passport.js. Config in src/config/auth.ts.
Test credentials in .env.example.

Closes #42
```

### 10. Code Standards
- Follow existing code style
- Write tests for new features
- Use Conventional Commits (see docs/COMMIT_STANDARD.md)
- Keep PRs focused and small

### 11. Communication
- Be concise in commit messages
- Reference issues in all commits
- Use AI-Context for complex changes
- Update issue comments for significant progress

### 12. Codex CLI Integration

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

### 13. GitHub Copilot Coding Agent

**⚠️ Copilot is GitHub-only - NO CLI available.**

**Assign issues to Copilot:**
```bash
# Method 1: Add label
gh issue edit <number> --add-label "copilot"

# Method 2: Assign directly
gh issue edit <number> --add-assignee "Copilot"
```

**Trigger in PRs:**
- Mention `@copilot` in PR comments for specific tasks
- Copilot creates branches named `copilot/*`

**Monitor Copilot work:**
```bash
gh pr list --head "copilot/"
```

### 14. Google Jules Agent

**⚠️ Jules has TWO methods: GitHub Label OR CLI. Tags like `@jules-google` do NOT work.**

**Method 1: GitHub Label (requires Jules GitHub App)**
```bash
# Add label "jules" (case insensitive) - Jules auto-responds
gh issue edit <number> --add-label "jules"
```

**Method 2: Jules CLI (Recommended for automation)**
```bash
# Install
npm install -g @google/jules
jules login

# Create session from current repo
jules new "implement feature X"

# Create session for specific repo
jules new --repo owner/repo "write unit tests"

# Parallel sessions (1-5)
jules new --parallel 3 "optimize queries"

# Create session from GitHub issue
gh issue view 42 --json title,body | jq -r '.title + "\n\n" + .body' | jules new
```

**Jules CLI Commands:**
```bash
jules                           # Interactive TUI dashboard
jules new "task"                # Create new session
jules remote list --session     # List all sessions
jules remote list --repo        # List connected repos
jules remote pull --session ID  # Get session results
jules remote pull --session ID --apply  # Pull and apply patch
```

### 15. Agent Load Balancing

**Auto-dispatch to available agents:**
```bash
# Add ai-agent label for automatic distribution
gh issue edit <number> --add-label "ai-agent"

# Or trigger workflow manually
gh workflow run agent-dispatcher.yml
```

### 16. AI Code Review Bots (CodeRabbit + Gemini)

**Automated AI reviews on every PR using two complementary bots:**

| Bot | Free For | Commands |
|-----|----------|----------|
| **CodeRabbit** | OSS (Pro free) | Auto-reviews every PR |
| **Gemini Code Assist** | Everyone (100%) | `/gemini review`, `/gemini summary` |

**Workflow:**
1. Create PR → CodeRabbit auto-reviews
2. Address suggestions → Use `/gemini review` for second opinion
3. Human approves → Merge ✅

**Configuration files:**
- `.coderabbit.yaml` - CodeRabbit rules
- `.gemini/config.yaml` - Gemini style guide

### 17. Commits Atómicos (OBLIGATORIO)

**UN commit = UN cambio lógico. NUNCA mezclar concerns.**

#### Antes de hacer `git add .`, pregúntate:
1. ¿Todos los archivos son del mismo módulo/scope?
2. ¿Es un solo tipo de cambio (feat/fix/docs/ci)?
3. ¿Puedo describirlo en < 72 caracteres?
4. ¿Revertirlo afectaría solo una funcionalidad?

Si alguna respuesta es "NO" → **SEPARAR EN MÚLTIPLES COMMITS**

#### Flujo correcto:
```bash
# ❌ NUNCA
git add .
git commit -m "feat: big update with everything"

# ✅ SIEMPRE
git add src/migrations/
git commit -m "feat(db): add user sessions table"

git add src/api/auth/
git commit -m "feat(auth): implement session endpoint"

git add docs/
git commit -m "docs: add authentication guide"
```

#### Herramientas:
```bash
# Si ya tienes muchos archivos staged
git-atomize --analyze    # Ver sugerencias de separación
git-atomize --interactive  # Separar interactivamente
```

---

## 🛠️ Git-Core CLI (RECOMMENDED)

### Overview

`git-core` es el CLI oficial para gestionar el Git-Core Protocol. **SIEMPRE usa el CLI** como método principal para instalar, actualizar y verificar el protocolo.

### Installation

**🔐 Trust & Transparency:** Before installing, read [docs/CLI_TRUST.md](../docs/CLI_TRUST.md) to understand exactly what the CLI does and verify the source code.

```bash
# 🚀 OPTION 1: Shell Scripts (código visible, puedes leerlo antes)
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex

# 🦀 OPTION 2: Cargo (compila desde código fuente en TU máquina)
# Antes de instalar, lee: docs/CLI_TRUST.md
# Código fuente: https://github.com/iberi22/Git-Core-Protocol/tree/main/tools/git-core-cli
cargo install git-core-cli

# 🔨 OPTION 3: Build from source (máxima confianza)
git clone https://github.com/iberi22/Git-Core-Protocol
cd Git-Core-Protocol/tools/git-core-cli
cargo build --release
./target/release/git-core install
```

### Commands Reference

| Command | Description | When to Use |
|---------|-------------|-------------|
| `git-core install` | Install protocol in current directory | New projects |
| `git-core install --force` | Force reinstall, overwrite all | Reset corrupted installation |
| `git-core upgrade` | Upgrade protocol (preserves ARCHITECTURE.md) | Update to latest version |
| `git-core upgrade --force` | Upgrade everything (overwrites ARCHITECTURE.md) | Full reset |
| `git-core migrate` | Migrate `.ai/` → `.✨/` | Legacy projects |
| `git-core migrate --remove-old` | Migrate and delete `.ai/` | Clean migration |
| `git-core check` | Verify protocol integrity | Troubleshooting |
| `git-core check --fix` | Auto-fix issues | Repair installation |
| `git-core status` | Show current protocol status | Quick overview |
| `git-core init [name]` | Initialize new project with protocol | New projects |
| `git-core version` | Show/bump version | Version management |
| `git-core self-update` | Update the CLI itself | Keep CLI updated |

### AI Agent Usage

**When bootstrapping a new project:**
```bash
# Step 1: Install protocol (scripts are visible and auditable)
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Or if CLI is available:
git-core init my-project

# Step 2: Verify installation
git-core check
```

**When upgrading existing project:**
```bash
# Safe upgrade (preserves your ARCHITECTURE.md)
git-core upgrade

# Check what changed
git-core status
```

**When troubleshooting:**
```bash
# Check integrity
git-core check

# Auto-fix issues
git-core check --fix

# Full status report
git-core status
```

### Legacy Scripts (Alternative)

Los scripts shell son **código visible** que puedes leer antes de ejecutar:

```bash
# Ver el código ANTES de ejecutar:
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh

# Si confías, entonces ejecuta:
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows - ver código primero:
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1" | Select-Object -ExpandProperty Content

# Luego ejecutar:
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Comparación de métodos:**
| Método | Confianza | Velocidad | Funcionalidades |
|--------|-----------|-----------|-----------------|
| Shell Scripts | ⭐⭐⭐⭐⭐ (código visible) | Rápido | Básico |
| Cargo install | ⭐⭐⭐⭐ (compila local) | Medio | Completo |
| Build from source | ⭐⭐⭐⭐⭐ (máximo control) | Lento | Completo |
| Pre-built binary | ⭐⭐⭐ (verificar checksum) | Muy rápido | Completo |



