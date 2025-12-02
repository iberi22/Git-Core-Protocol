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

> **"Inteligente, sofisticada pero minimalista en complejidad"**
>
> *AI-Driven Project Management Template — By Devs, For Devs*

[![Use this template](https://img.shields.io/badge/Use%20this-template-blue?style=for-the-badge)](https://github.com/iberi22/Git-Core-Protocol/generate)
[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?style=for-the-badge)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![AI Code Review](https://img.shields.io/badge/AI%20Review-CodeRabbit%20%2B%20Gemini-purple?style=for-the-badge)](https://github.com/marketplace/coderabbit)

---

## 🌍 Language | Idioma | Língua | Sprache | Langue | 言語 | 语言

| Flag | Language | Link |
|:----:|----------|------|
| 🇺🇸 | **English** | [Jump to English](#-english) |
| 🇪🇸 | **Español** | [Ir a Español](#-español) |
| 🇧🇷 | **Português** | [Ir para Português](#-português) |
| 🇩🇪 | **Deutsch** | [Zu Deutsch](#-deutsch) |
| 🇫🇷 | **Français** | [Aller au Français](#-français) |
| 🇯🇵 | **日本語** | [日本語へ](#-日本語) |
| 🇨🇳 | **中文** | [跳转到中文](#-中文) |

---

# 🇺🇸 English

## 🚀 Quick Start - Just Copy This Prompt

> **Copy this prompt to your AI coding agent (Copilot, Cursor, Claude, etc.) to auto-setup:**

```
Initialize this project with Git-Core Protocol.

# Method 1: Shell Scripts (transparent, visible code)
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex

# Method 2: CLI (if available)
git-core init
git-core check

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

## 🤖 Agent "Dressing Room" (New!)

This protocol now includes a dynamic system to "equip" your AI agent with specific roles (Backend Architect, UX Researcher, etc.) on demand.

### How it works

1. **Index:** Check `.✨/AGENT_INDEX.md` to see available roles.
2. **Equip:** Run the script to download and load the persona.
3. **Act:** The agent reads the generated context and behaves like an expert.

```powershell
# Example: Load the Backend Architect persona
./scripts/equip-agent.ps1 -Role "backend"
```

The system automatically:

- ⬇️ Downloads the latest recipe from `agents-flows-recipes`.
- 🛡️ Injects mandatory protocol skills (Atomic Commits, Architecture First).
- 🧠 Generates a `.✨/CURRENT_CONTEXT.md` file for the agent.

---

## 🧠 Model-Specific Agents (New in v1.4!)

Custom VS Code Copilot agents optimized for different LLM models:

| Agent | Model | Best For | Context |
|-------|-------|----------|---------|
| `@protocol-claude` | Claude Sonnet 4 | Standard tasks, reasoning | 200K |
| `@architect` | Claude Opus 4.5 | Architecture decisions | 200K |
| `@quick` | Claude Haiku 4.5 | Fast responses | 200K |
| `@protocol-gemini` | Gemini 3 Pro | Large context, multi-modal | 1M+ |
| `@protocol-codex` | GPT-5.1 Codex | Implementation, coding | - |
| `@protocol-grok` | Grok Code Fast 1 | Massive codebase analysis | **2M** |
| `@router` | Auto | Agent selection helper | - |

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

## 🤖 AI Report Generation (New in v1.5!)

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

| Method | Trust Level | Speed | Features |
|--------|-------------|-------|----------|
| Shell Scripts | ⭐⭐⭐⭐⭐ (visible code) | Fast | Basic |
| Cargo install | ⭐⭐⭐⭐ (compiles locally) | Medium | Full |
| Build from source | ⭐⭐⭐⭐⭐ (maximum control) | Slow | Full |
| Pre-built binary | ⭐⭐⭐ (verify checksum) | Very Fast | Full |

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

---

# 🇪🇸 Español

## 🚀 Inicio Rápido - ¡Solo Copia Este Prompt

> **Copia este prompt a tu agente de código IA (Copilot, Cursor, Claude, etc.) para auto-configurar:**

```
Inicializa este proyecto con Git-Core Protocol. Ejecuta:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
Luego ejecuta: .\scripts\init_project.ps1 -Auto
Después del setup, lee .✨/ARCHITECTURE.md y empieza con el primer issue de: gh issue list
```

Para Linux/Mac:

```
Inicializa este proyecto con Git-Core Protocol. Ejecuta:
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
Luego ejecuta: ./scripts/init_project.sh
Después del setup, lee .✨/ARCHITECTURE.md y empieza con el primer issue de: gh issue list
```

---

## 🤔 ¿Por Qué Este Enfoque?

| Problema | Solución Git-Core |
|----------|-------------------|
| La IA "olvida" el estado de tareas | Estado en GitHub Issues (persistente) |
| Contexto crece = más tokens = más costo | Solo cargar issue actual + arquitectura |
| Archivos TODO.md desordenados | Tablero GitHub organizado |
| Dependencia de ecosistema (NPM, etc.) | Scripts bash/PowerShell agnósticos |

## 📦 Opciones de Instalación

### Opción 1: Instalación Remota (⚡ Nivel Dios)

**Windows PowerShell:**

```powershell
# En tu carpeta de proyecto
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex

# Modo automático (para AI Agents)
$env:GIT_CORE_AUTO = "1"; $env:GIT_CORE_ORGANIZE = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Linux/Mac:**

```bash
# En tu carpeta de proyecto
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Modo automático (para AI Agents)
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto --organize
```

### Opción 2: Usar como Template

1. Click en **"Use this template"** arriba
2. Clona tu nuevo repositorio
3. Ejecuta: `./scripts/init_project.sh` o `.\scripts\init_project.ps1`

## 🗂️ Organización Automática

| Tipo de archivo | Destino |
|-----------------|---------|
| `*.md` (excepto README, AGENTS) | `docs/archive/` |
| `test_*.py`, `*.test.js` | `tests/` |
| `*.sh`, `*.bat` (scripts sueltos) | `scripts/` |

## 🏷️ Etiquetas Semánticas

| Label | Uso |
|-------|-----|
| `ai-plan` | Tareas de planificación |
| `ai-context` | Información crítica |
| `ai-blocked` | Requiere intervención humana |
| `in-progress` | Tarea en desarrollo |

## 📊 Ciclo de Vida de Issues y Seguimiento de Progreso

**Los issues permanecen OPEN** mientras tengan tareas pendientes. Se **cierran automáticamente** cuando un commit incluye `closes #X`.

```text
┌─────────────────────────────────────────────────────────┐
│  OPEN (Abierto)                                         │
│  ├── 📋 Backlog: Nadie asignado, esperando              │
│  ├── 🔄 In Progress: Alguien asignado, trabajando       │
│  └── ⏸️ Blocked: Esperando dependencia                  │
└─────────────────────────────────────────────────────────┘
                         │
                         │ Commit con "closes #X"
                         ▼
┌─────────────────────────────────────────────────────────┐
│  CLOSED (Cerrado)                                       │
│  └── ✅ Completado: Todas las tareas hechas             │
└─────────────────────────────────────────────────────────┘
```

**Seguimiento de Progreso:** Usa un **EPIC issue** con checkboxes para rastrear el progreso general. GitHub calcula el porcentaje automáticamente. ¡No se necesitan archivos locales!

---

## 🤖 Generación de Reportes AI (Nuevo en v1.5!)

Análisis automatizado de PRs usando múltiples modelos de IA:

| Herramienta | Modelo | Propósito |
|-------------|--------|-----------|
| **Gemini CLI** | Gemini 3 Pro | Análisis técnico, evaluación de impacto |
| **Copilot CLI** | Claude Sonnet 4.5 | Revisión profunda, recomendaciones |

### Uso

```powershell
# Reporte completo (Gemini + Copilot)
./scripts/ai-report.ps1 -PrNumber 42

# Solo Copilot con Opus para análisis profundo
./scripts/ai-report.ps1 -ReportType copilot -Model claude-opus-4.5

# Solo Gemini
./scripts/ai-report.ps1 -ReportType gemini

# Preview sin publicar
./scripts/ai-report.ps1 -DryRun
```

### Modelos Disponibles para Copilot CLI

| Modelo | Mejor Para |
|--------|------------|
| `claude-sonnet-4.5` | Análisis balanceado (default) |
| `claude-opus-4.5` | Revisión técnica profunda |
| `claude-haiku-4.5` | Verificaciones rápidas |
| `gpt-5.1` / `gpt-5.1-codex` | Perspectivas alternativas |

### Contenido del Reporte

El reporte AI incluye:

- 🔍 **Resumen de Cambios** (puntos clave)
- 📊 **Análisis de Impacto** (Alto/Medio/Bajo con justificación)
- ⚠️ **Posibles Riesgos**
- ✅ **Recomendaciones** para el reviewer
- 🏷️ **Etiquetas Sugeridas**

Los reportes se publican automáticamente como comentarios en el PR.

---

# 🇧🇷 Português

## 🚀 Início Rápido - Apenas Copie Este Prompt

> **Copie este prompt para seu agente de código IA (Copilot, Cursor, Claude, etc.) para auto-configurar:**

```
Inicialize este projeto com Git-Core Protocol. Execute:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
Depois execute: .\scripts\init_project.ps1 -Auto
Após o setup, leia .✨/ARCHITECTURE.md e comece com a primeira issue de: gh issue list
```

Para Linux/Mac:

```
Inicialize este projeto com Git-Core Protocol. Execute:
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
Depois execute: ./scripts/init_project.sh
Após o setup, leia .✨/ARCHITECTURE.md e comece com a primeira issue de: gh issue list
```

---

## 🤔 Por Que Esta Abordagem?

| Problema | Solução Git-Core |
|----------|------------------|
| IA "esquece" o estado das tarefas | Estado no GitHub Issues (persistente) |
| Contexto cresce = mais tokens = mais custo | Carregar apenas issue atual + arquitetura |
| Arquivos TODO.md desorganizados | Quadro GitHub organizado |
| Dependência de ecossistema (NPM, etc.) | Scripts bash/PowerShell agnósticos |

## 📦 Opções de Instalação

### Opção 1: Instalação Remota (⚡ Modo Deus)

**Windows PowerShell:**

```powershell
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Linux/Mac:**

```bash
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
```

## 🤖 Agentes IA Compatíveis

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

## 📊 Ciclo de Vida de Issues e Acompanhamento de Progresso

**Issues permanecem OPEN** enquanto tiverem tarefas pendentes. São **fechados automaticamente** quando um commit inclui `closes #X`.

**Acompanhamento de Progresso:** Use um **EPIC issue** com checkboxes para rastrear o progresso geral. GitHub calcula a porcentagem automaticamente. Não são necessários arquivos locais!

---

# 🇩🇪 Deutsch

## 🚀 Schnellstart - Kopiere Einfach Diesen Prompt

> **Kopiere diesen Prompt zu deinem KI-Coding-Agenten (Copilot, Cursor, Claude, etc.) für Auto-Setup:**

```
Initialisiere dieses Projekt mit Git-Core Protocol. Führe aus:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
Dann führe aus: .\scripts\init_project.ps1 -Auto
Nach dem Setup, lies .✨/ARCHITECTURE.md und beginne mit dem ersten Issue von: gh issue list
```

Für Linux/Mac:

```
Initialisiere dieses Projekt mit Git-Core Protocol. Führe aus:
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
Dann führe aus: ./scripts/init_project.sh
Nach dem Setup, lies .✨/ARCHITECTURE.md und beginne mit dem ersten Issue von: gh issue list
```

---

## 🤔 Warum Dieser Ansatz?

| Problem | Git-Core Lösung |
|---------|-----------------|
| KI "vergisst" Aufgabenstatus | Status in GitHub Issues (persistent) |
| Kontext wächst = mehr Tokens = mehr Kosten | Nur aktuelles Issue + Architektur laden |
| Unordentliche TODO.md Dateien | Organisiertes GitHub Board |
| Ökosystem-Abhängigkeit (NPM, etc.) | Sprachunabhängige bash/PowerShell Skripte |

## 📦 Installationsoptionen

**Windows PowerShell:**

```powershell
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Linux/Mac:**

```bash
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
```

## 🤖 Kompatible KI-Agenten

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

## 📊 Issue-Lebenszyklus & Fortschrittsverfolgung

**Issues bleiben OPEN** solange sie ausstehende Aufgaben haben. Sie werden **automatisch geschlossen** wenn ein Commit `closes #X` enthält.

**Fortschrittsverfolgung:** Verwende ein **EPIC Issue** mit Checkboxen um den Gesamtfortschritt zu verfolgen. GitHub berechnet den Prozentsatz automatisch. Keine lokalen Dateien nötig!

---

# 🇫🇷 Français

## 🚀 Démarrage Rapide - Copiez Simplement Ce Prompt

> **Copiez ce prompt vers votre agent de code IA (Copilot, Cursor, Claude, etc.) pour auto-configurer:**

```
Initialise ce projet avec Git-Core Protocol. Exécute:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
Puis exécute: .\scripts\init_project.ps1 -Auto
Après le setup, lis .✨/ARCHITECTURE.md et commence avec la première issue de: gh issue list
```

Pour Linux/Mac:

```
Initialise ce projet avec Git-Core Protocol. Exécute:
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
Puis exécute: ./scripts/init_project.sh
Après le setup, lis .✨/ARCHITECTURE.md et commence avec la première issue de: gh issue list
```

---

## 🤔 Pourquoi Cette Approche?

| Problème | Solution Git-Core |
|----------|-------------------|
| L'IA "oublie" l'état des tâches | État dans GitHub Issues (persistant) |
| Contexte grandit = plus de tokens = plus de coût | Charger seulement l'issue actuelle + architecture |
| Fichiers TODO.md désordonnés | Tableau GitHub organisé |
| Dépendance d'écosystème (NPM, etc.) | Scripts bash/PowerShell agnostiques |

## 📦 Options d'Installation

**Windows PowerShell:**

```powershell
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Linux/Mac:**

```bash
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
```

## 🤖 Agents IA Compatibles

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

## 📊 Cycle de Vie des Issues & Suivi du Progrès

**Les issues restent OPEN** tant qu'elles ont des tâches en attente. Elles sont **fermées automatiquement** quand un commit inclut `closes #X`.

**Suivi du Progrès:** Utilisez un **EPIC issue** avec des checkboxes pour suivre le progrès global. GitHub calcule le pourcentage automatiquement. Pas de fichiers locaux nécessaires!

---

# 🇯🇵 日本語

## 🚀 クイックスタート - このプロンプトをコピーするだけ

> **AIコーディングエージェント（Copilot、Cursor、Claudeなど）にこのプロンプトをコピーして自動セットアップ：**

```
Git-Core Protocolでこのプロジェクトを初期化してください。実行：
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
次に実行: .\scripts\init_project.ps1 -Auto
セットアップ後、.✨/ARCHITECTURE.mdを読み、gh issue listから最初のissueを始めてください
```

Linux/Macの場合:

```
Git-Core Protocolでこのプロジェクトを初期化してください。実行：
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
次に実行: ./scripts/init_project.sh
セットアップ後、.✨/ARCHITECTURE.mdを読み、gh issue listから最初のissueを始めてください
```

---

## 🤔 なぜこのアプローチ？

| 問題 | Git-Core ソリューション |
|------|------------------------|
| AIがタスク状態を「忘れる」 | GitHub Issuesで状態管理（永続的） |
| コンテキスト増加 = トークン増 = コスト増 | 現在のissue + アーキテクチャのみロード |
| 乱雑なTODO.mdファイル | 整理されたGitHubボード |
| エコシステム依存（NPMなど） | 言語非依存のbash/PowerShellスクリプト |

## 📦 インストールオプション

**Windows PowerShell:**

```powershell
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Linux/Mac:**

```bash
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
```

## 🤖 対応AIエージェント

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

## 📊 Issueライフサイクルと進捗追跡

**Issueは未完了タスクがある間OPEN**のままです。コミットに`closes #X`が含まれると**自動的にクローズ**されます。

**進捗追跡:** チェックボックス付きの**EPIC issue**を使用して全体の進捗を追跡します。GitHubが自動的にパーセンテージを計算します。ローカルファイル不要！

---

# 🇨🇳 中文

## 🚀 快速开始 - 只需复制这个提示词

> **将此提示词复制到您的AI编程助手（Copilot、Cursor、Claude等）以自动设置：**

```
使用Git-Core Protocol初始化此项目。执行：
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
然后执行: .\scripts\init_project.ps1 -Auto
设置完成后，阅读.✨/ARCHITECTURE.md并从gh issue list开始第一个issue
```

Linux/Mac:

```
使用Git-Core Protocol初始化此项目。执行：
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash -s -- --auto
然后执行: ./scripts/init_project.sh
设置完成后，阅读.✨/ARCHITECTURE.md并从gh issue list开始第一个issue
```

---

## 🤔 为什么选择这种方法？

| 问题 | Git-Core 解决方案 |
|------|-------------------|
| AI"忘记"任务状态 | 状态存储在GitHub Issues（持久化） |
| 上下文增长 = 更多token = 更多成本 | 仅加载当前issue + 架构 |
| 混乱的TODO.md文件 | 有组织的GitHub看板 |
| 生态系统依赖（NPM等） | 语言无关的bash/PowerShell脚本 |

## 📦 安装选项

**Windows PowerShell:**

```powershell
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

**Linux/Mac:**

```bash
curl -sL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
```

## 🤖 兼容的AI助手

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

## 📊 Issue生命周期与进度跟踪

**Issue在有待处理任务时保持OPEN**状态。当commit包含`closes #X`时会**自动关闭**。

**进度跟踪:** 使用带有复选框的**EPIC issue**来跟踪整体进度。GitHub自动计算百分比。不需要本地文件！

---

## 📋 Requirements | Requisitos | Requisitos | Anforderungen | Prérequis | 要件 | 要求

- [Git](https://git-scm.com/)
- [GitHub CLI](https://cli.github.com/) (`gh`) - authenticated | autenticado | authentifié | 認証済み | 已认证

---

## 📄 License | Licencia | Licença | Lizenz | Licence | ライセンス | 许可证

**CC BY-NC-SA 4.0** - Attribution-NonCommercial-ShareAlike

✅ **You CAN:**

- Use it for personal/educational projects
- Modify and adapt it
- Share it with attribution

❌ **You CANNOT:**

- Sell it or use it commercially without permission
- Remove attribution to Git-Core Protocol

**Attribution Required:** "This project uses Git-Core Protocol by @iberi22"

For commercial use, contact: <https://github.com/iberi22>

Full license: [LICENSE](LICENSE)

---

**Created with 🧠 by [@iberi22](https://github.com/iberi22)**
