---
title: "[Protocol v3.0] Full Autonomy - Autonomous Agent Cycle"
labels:
  - enhancement
  - ai-plan
  - protocol
  - breaking-change
assignees: []
milestone: "v3.0.0"
---

## 🎯 Objetivo

Evolucionar el Git-Core Protocol de v1.5.0 (v2.1 conceptual) a v3.0.0 "Full Autonomy", agregando agentes autónomos que eliminan la intervención humana del ciclo de desarrollo excepto para operaciones high-stakes.

## 🔄 Nuevo Ciclo Autónomo

```
┌─────────────────────────────────────────────────────────────────────────────┐
│              FLUJO v3.0 - "FULL AUTONOMY"                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  🧠 PLANNER  ──▶  🎯 ROUTER  ──▶  🛠️ EXECUTOR  ──▶  🔍 REVIEWER           │
│       ▲           (Dispatcher)    (Copilot/Jules)  (CodeRabbit)            │
│       │                                                    │                │
│       │                                                    ▼                │
│       └────────────────────  🛡️ GUARDIAN  ◀───────────────┘                │
│                             (Auto-Merge)                                    │
│                                                                             │
│  ⚡ Human intervention: ONLY for `high-stakes` labeled items               │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 🤖 Nuevos Agentes

| Agent | Workflow | Función |
|-------|----------|---------|
| **🧠 Planner** | `planner-agent.yml` | Lee ARCHITECTURE.md → Genera issues atómicos |
| **🛡️ Guardian** | `guardian-agent.yml` | Auto-merge con scoring de confianza (70%+) |
| **🎯 Router** (mejorado) | `agent-dispatcher.yml` | Skill-matching además de round-robin |

## ✅ Tareas

### Fase 1: Workflows Core
- [ ] Agregar `guardian-agent.yml`
- [ ] Agregar `planner-agent.yml`
- [ ] Agregar labels requeridos a `setup-labels.yml`

### Fase 2: Documentación
- [ ] Actualizar `AGENTS.md` con sección v3.0 Autonomous Agents
- [ ] Actualizar `README.md` con nuevo diagrama
- [ ] Crear template `.✨/features.json`

### Fase 3: Instalador
- [ ] Actualizar `install.ps1` para incluir nuevos workflows
- [ ] Actualizar `install.sh` para incluir nuevos workflows
- [ ] Bump versión a `3.0.0`

### Fase 4: Testing
- [ ] Probar ciclo completo en repo de ejemplo
- [ ] Documentar breaking changes

## 📦 Breaking Changes

1. **Nuevo archivo requerido**: `.✨/features.json` para Planner
2. **Nuevos labels**: `high-stakes`, `needs-human`, `automation`
3. **Auto-merge**: PRs que cumplan criterios se mergean automáticamente

## 🔗 Origen

Cambios portados desde implementación en `synapse-protocol`:
- `guardian-agent.yml` - Scoring de confianza para auto-merge
- `planner-agent.yml` - Generación automática de issues
- `AGENT_INDEX.md` v3.0 - Documentación de agentes autónomos

## 📚 Referencias

- [Anthropic: Effective harnesses for long-running agents](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- [12-Factor Agents](https://12factoragents.com)
