---
title: "Evolution Protocol - Continuous Improvement System"
type: SPECIFICATION
id: "spec-evolution-protocol"
created: 2025-12-05
updated: 2025-12-05
agent: protocol-gemini
model: gemini-3-pro
requested_by: user
summary: |
  Weekly evolution mechanism for Git-Core Protocol. Defines metrics,
  evaluation cycles, and improvement workflows to ensure continuous
  logical adaptation of the orchestration system.
keywords: [evolution, metrics, improvement, kaizen, feedback-loop]
tags: ["#evolution", "#metrics", "#continuous-improvement"]
protocol_version: 1.4.0
project: Git-Core-Protocol
---

# 🧬 Evolution Protocol (Git-Core v2.1+)

> **"Un sistema que no evoluciona, muere."**

Este protocolo define el mecanismo de **evolución semanal** del Git-Core Protocol, asegurando que cada eslabón del proceso de orquestación mejore continuamente mediante análisis de métricas, feedback, y adaptación lógica.

## 1. Filosofía: Kaizen Automatizado

```
┌─────────────────────────────────────────────────────────────┐
│           CICLO DE EVOLUCIÓN SEMANAL                        │
├─────────────────────────────────────────────────────────────┤
│   1. MEDIR    → Recolectar métricas de Issues/PRs/Workflows │
│   2. ANALIZAR → Identificar patrones y fricción             │
│   3. PROPONER → Generar hipótesis de mejora                 │
│   4. IMPLEMENTAR → Aplicar cambios incrementales            │
│   5. VALIDAR  → Verificar impacto positivo                  │
│   ↺ Repetir cada semana                                      │
└─────────────────────────────────────────────────────────────┘
```

## 2. Taxonomía de Métricas

Las métricas se clasifican en **3 órdenes** según su nivel de abstracción:

### Orden 1: Métricas Operativas (Diarias)

| Métrica | Fuente | Objetivo |
|---------|--------|----------|
| `issues_opened` | GitHub API | Volumen de trabajo entrante |
| `issues_closed` | GitHub API | Throughput del sistema |
| `avg_issue_age_hours` | Calculado | Velocidad de resolución |
| `prs_merged` | GitHub API | Features entregadas |
| `workflow_failures` | Actions API | Estabilidad del CI/CD |

### Orden 2: Métricas de Calidad (Semanales)

| Métrica | Fuente | Objetivo |
|---------|--------|----------|
| `agent_state_usage_%` | Issue comments | Adopción del Context Protocol |
| `atomic_commit_ratio` | Commit history | Calidad de commits |
| `human_intervention_rate` | Labels + comments | Autonomía del agente |
| `high_stakes_blocked_%` | HumanLayer events | Seguridad cumplida |
| `context_handoff_success_%` | Session exports | Continuidad entre sesiones |

### Orden 3: Métricas de Evolución (Mensuales)

| Métrica | Fuente | Objetivo |
|---------|--------|----------|
| `protocol_version_adoption` | AGENTS.md diffs | Velocidad de upgrade |
| `new_recipes_created` | agents-flows-recipes repo | Expansión del ecosistema |
| `friction_reports` | Issues con label `friction` | Puntos de dolor identificados |
| `improvement_proposals` | Issues con label `evolution` | Ideas de mejora |
| `breaking_changes` | CHANGELOG | Estabilidad de la API |

## 3. Flujo de Evaluación Semanal

### 3.1 Trigger: Workflow Automatizado

```yaml
# .github/workflows/evolution-cycle.yml
name: Weekly Evolution Cycle
on:
  schedule:
    - cron: '0 9 * * 1'  # Lunes 9:00 UTC
  workflow_dispatch:

jobs:
  collect-metrics:
    # Recolecta métricas de la semana anterior

  analyze-patterns:
    # Detecta patrones y anomalías

  generate-report:
    # Crea issue con reporte de evolución

  propose-improvements:
    # Sugiere cambios basados en datos
```

### 3.2 Recolección de Datos

El script `scripts/evolution-metrics.ps1` debe:

1. **Consultar GitHub API:**
   - Issues abiertos/cerrados en los últimos 7 días
   - PRs merged y su tiempo de ciclo
   - Comentarios con bloques `<agent-state>`

2. **Analizar logs de Actions:**
   - Workflows fallidos y causas
   - Tiempos de ejecución

3. **Parsear Issue comments:**
   - Contar uso de bloques XML válidos
   - Identificar `<input_request>` pendientes

### 3.3 Análisis de Patrones

El análisis busca:

| Patrón | Indicador | Acción Sugerida |
|--------|-----------|-----------------|
| **Bucle de muerte** | `tool_calls > 20` en `<metrics>` | Escalamiento a humano |
| **Abandono de protocolo** | < 30% de issues usan `<agent-state>` | Reforzar docs/training |
| **Bloqueo frecuente** | > 50% issues con label `blocked` | Revisar dependencias |
| **Fricción en script** | Múltiples reportes de error en `agent-state.ps1` | Hotfix inmediato |
| **Baja atomicidad** | < 70% commits son atómicos | Aumentar validación CI |

### 3.4 Generación de Reporte

Se crea automáticamente un issue:

```markdown
---
title: "[Evolution] Weekly Report - Semana 49 (2025)"
labels: [evolution, weekly-report]
---

## 📊 Métricas de la Semana

| Métrica | Valor | Tendencia | Objetivo |
|---------|-------|-----------|----------|
| Issues cerrados | 12 | 📈 +20% | > 10 |
| Uso de agent-state | 67% | 📈 +15% | > 80% |
| Commits atómicos | 82% | ➡️ = | > 75% |
| Intervención humana | 23% | 📉 -5% | < 30% |

## 🔍 Patrones Detectados

1. **Positivo:** Aumento en adopción de Context Protocol.
2. **Atención:** 3 issues con bucle de muerte detectado.
3. **Fricción:** Script `agent-state.ps1` no soporta `<plan>` inline.

## 💡 Propuestas de Mejora

- [ ] #55 Añadir soporte para `-PlanItems` en agent-state.ps1
- [ ] #56 Implementar auto-escalamiento cuando tool_calls > 15
- [ ] #57 Crear tutorial interactivo para nuevos usuarios

## 📅 Próximo Ciclo

- **Foco:** Reducir fricción en scripts
- **Meta:** agent-state usage > 80%
```

## 4. Tipos de Mejora

Las mejoras se categorizan para priorización:

| Tipo | Urgencia | Ejemplo |
|------|----------|---------|
| 🔴 **Hotfix** | Inmediato | Bug crítico en workflow |
| 🟠 **Optimización** | Esta semana | Mejorar rendimiento de script |
| 🟡 **Feature** | Próximo sprint | Nuevo campo en agent-state |
| 🟢 **Evolución** | Próximo mes | Nuevo protocolo o integración |
| 🔵 **Investigación** | Backlog | Explorar nuevos patrones |

## 5. Feedback Loop con Agentes

### 5.1 Auto-Reporte de Fricción

Los agentes deben reportar fricción automáticamente:

```xml
<agent-state>
  <step>blocked</step>
  <friction>
    <component>agent-state.ps1</component>
    <issue>No support for inline plan items</issue>
    <severity>medium</severity>
    <suggestion>Add -PlanItems parameter accepting JSON array</suggestion>
  </friction>
</agent-state>
```

### 5.2 Etiquetado Semántico de Issues

| Label | Significado | Usado por |
|-------|-------------|-----------|
| `friction` | Problema de usabilidad | Agentes + Humanos |
| `evolution` | Propuesta de mejora | Sistema de evolución |
| `metrics` | Requiere medición | Workflow de evolución |
| `experimental` | Feature en prueba | Desarrolladores |

## 6. Integración con Context Protocol

El Evolution Protocol se integra con el Context Protocol v2.1:

```xml
<agent-state>
  <!-- Campos estándar v2.1 -->
  <intent>improve_protocol</intent>
  <step>analyzing</step>

  <!-- Campos de Evolución (opcional) -->
  <evolution>
    <cycle>49</cycle>
    <focus>script_usability</focus>
    <experiments>
      <experiment id="exp-001" status="active">
        <hypothesis>Añadir -PlanItems reducirá fricción 30%</hypothesis>
        <metric>friction_reports</metric>
      </experiment>
    </experiments>
  </evolution>
</agent-state>
```

## 7. Cronograma de Evolución

| Día | Actividad | Automatizado |
|-----|-----------|--------------|
| Lunes | Recolección de métricas | ✅ Workflow |
| Martes | Análisis de patrones | ✅ Workflow |
| Miércoles | Generación de reporte | ✅ Issue automático |
| Jueves-Viernes | Implementación de mejoras | 🔄 Agentes + Humanos |
| Fin de semana | Validación en staging | ⏸️ Opcional |

## 8. Governance de Cambios

### Breaking Changes

Cualquier cambio que modifique:
- Formato de `<agent-state>` → Requiere RFC (Issue con label `rfc`)
- Scripts públicos (`agent-state.ps1`) → Requiere deprecation period
- AGENTS.md core rules → Requiere review de 2 humanos

### Non-Breaking Improvements

- Nuevos campos opcionales → PR directo
- Mejoras de documentación → PR directo
- Nuevos labels/templates → PR directo

## 9. Métricas de Éxito del Evolution Protocol

| Métrica | Baseline | Meta Q1 2026 |
|---------|----------|--------------|
| Semanas con reporte generado | 0% | 100% |
| Mejoras implementadas / propuestas | 0% | > 60% |
| Reducción de fricción reportada | N/A | -30% MoM |
| Adopción de nuevas features | N/A | > 50% en 2 semanas |

---

## 10. Implementación Inmediata

### Fase 1 (Esta semana)
- [ ] Crear script `evolution-metrics.ps1`
- [ ] Crear workflow `evolution-cycle.yml`
- [ ] Definir labels de evolución

### Fase 2 (Próxima semana)
- [ ] Primer ciclo de recolección
- [ ] Primer reporte automático
- [ ] Ajustar umbrales basados en datos reales

### Fase 3 (Mes 1)
- [ ] Dashboard visual de métricas
- [ ] Alertas automáticas de anomalías
- [ ] Integración con notificaciones (Slack/Email)
