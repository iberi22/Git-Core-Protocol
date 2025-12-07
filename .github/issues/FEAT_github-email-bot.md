---
github_issue: 63
title: "Self-Healing CI/CD Automation"
labels:
  - enhancement
  - ai-agent
  - automation
assignees: []
---

## Descripción

Sistema de auto-reparación para detectar, analizar y resolver fallos de CI/CD automáticamente sin intervención humana.

## 🎯 Solución Implementada

### Método Principal: `workflow_run` Events (Recomendado)

✅ **Latencia:** < 1 minuto
✅ **Costo:** $0 (GitHub Actions gratuito)
✅ **Sin email polling:** Event-driven nativo
✅ **Escalable:** Multi-repo compatible

**Archivo:** `.github/workflows/self-healing.yml`

### Método Fallback: Email Handler (Opcional)

- **Ubicación:** `tools/email-handler/`
- **Uso:** Solo como backup si GitHub Actions tiene downtime
- **Estado:** Implementado pero no activo por defecto

## 📋 Capacidades de Auto-Reparación

| Tipo de Error | Acción Automática |
|---------------|-------------------|
| **Transient** (ETIMEDOUT, 429) | Re-ejecutar workflow automáticamente |
| **Dependency** (npm/pip/yarn) | Crear PR con lockfiles actualizados |
| **Linting** (ESLint, Prettier) | Aplicar auto-fix y crear PR |
| **Tests/Code** | Crear issue + asignar a AI agent |

## 🔧 Configuración

### 1. Activar Self-Healing Workflow

```bash
# Ya incluido en el protocolo
git pull origin main
```

### 2. Desactivar Notificaciones de Email (Opcional)

**GitHub UI:**

1. Ve a: <https://github.com/settings/notifications>
2. Desactiva: `Actions → Failed workflows`
3. Mantén activo: `Security alerts`

**O vía API:**

```bash
gh api --method PATCH /user/settings/notifications \
  -f actions_failed_workflows=false
```

### 3. Configurar Permisos del Repo

Verifica que GitHub Actions tenga permisos para:

- ✅ `actions: write` (para re-ejecutar workflows)
- ✅ `issues: write` (para crear issues)
- ✅ `pull-requests: write` (para PRs de fix)

**Settings → Actions → General → Workflow permissions:**

- Selecciona: "Read and write permissions"

## 📊 Métricas Esperadas

| Métrica | Objetivo |
|---------|----------|
| Auto-repair rate | > 60% |
| Time to action | < 5 min |
| False positives | < 5% |
| Email reduction | 90% |

## 🔗 Documentación Relacionada

- [RESEARCH_SELFHEALING_CICD.md](../../docs/agent-docs/RESEARCH_SELFHEALING_CICD.md) - Investigación completa de alternativas
- [Self-Healing Workflow](../../.github/workflows/self-healing.yml) - Implementación

## Tareas

- [x] Investigar alternativas (Email vs Webhooks vs workflow_run)
- [x] Crear workflow `self-healing.yml`
- [x] Implementar clasificación de errores
- [x] Auto-retry para errores transitorios
- [x] Auto-fix para dependencias
- [x] Auto-fix para linting
- [x] Crear issues para errores de código
- [x] Fix: Prevenir loop infinito del workflow
- [x] Implementar archivado automático de emails
- [x] Verificar estado de workflows antes de actuar
- [ ] Monitorear métricas (1 semana)
- [ ] Desplegar self-healing.yml a otros repos (software-factory, domus-otec, etc.)
- [ ] Refinar patrones de detección
- [ ] Documentar en README principal

## 📊 Estado Actual (2025-12-06)

### Deployment

- ✅ Workflow `self-healing.yml` desplegado en Git-Core-Protocol
- ✅ Fix aplicado: Prevención de auto-monitoreo
- ⏳ Pendiente: Verificar ejecución exitosa del workflow

### Email Handler

- ✅ Lógica de archivado implementada
- ✅ Verificación de estado de workflows
- ⏳ Pendiente: Ejecutar en modo watch para limpieza continua

### Correos Detectados

- 📧 **94 correos de fallos** en inbox
- Repos afectados: software-factory, domus-otec, less-colegio, synapse-protocol, etc.
- Workflows comunes fallando:
  - `Sync Issues from Files`
  - `Copilot Auto-Implementation`
  - `User Notification System`
  - `CI` (varios repos)
  - `E2E Tests` (less-colegio)

### Próximos Pasos

1. Verificar que self-healing.yml se ejecute correctamente
2. Monitorear si auto-repara fallos transitorios
3. Copiar self-healing.yml a repos críticos
4. Ejecutar email-handler en modo watch para limpieza continua

- [ ] Implementar borrado de correos post-fix.
