---
title: "Migrate Guardian Agent to Rust for 20-30x Performance Improvement"
labels:
  - enhancement
  - performance
  - rust
  - ai-plan
  - high-priority
assignees: []
---

## 🎯 Objetivo

Migrar `scripts/guardian-core.ps1` (202 líneas) a Rust como módulo de `workflow-orchestrator` para mejorar performance crítica en el flujo de auto-merge.

## 📊 Impacto

- **Prioridad:** 🔥 ALTA (95% impacto)
- **Frecuencia:** Se ejecuta en CADA PR review
- **Performance:** 20-30x speedup (2-3s → <100ms)
- **Ahorro estimado:** ~145 segundos/día (~72 minutos/mes)

## 🔍 Análisis Actual

**Workflow afectado:** `.github/workflows/guardian-agent.yml`

**Script actual:** `scripts/guardian-core.ps1`
- Parsing JSON de PR data (labels, reviews, checks)
- Cálculo de confidence score
- Risk analysis con regex patterns
- Decisión de auto-merge vs escalate

**Cuellos de botella:**
- PowerShell startup overhead
- JSON parsing lento
- Múltiples llamadas `gh` CLI
- No paralelizable

## 🏗️ Implementación Propuesta

### Fase 1: Estructura Base

**Ubicación:** `tools/workflow-orchestrator/src/guardian_core.rs`

**Funciones principales:**
```rust
pub struct GuardianCore {
    github_client: Octocrab,
    risk_map: RiskMap,
    threshold: u8,
}

impl GuardianCore {
    pub async fn evaluate_pr(&self, pr_number: u64) -> Result<Decision> {
        let pr_data = self.fetch_pr_data(pr_number).await?;
        let blockers = self.check_blockers(&pr_data.labels);
        let ci_status = self.check_ci_status(&pr_data.checks).await?;
        let risk = self.calculate_risk(&pr_data.files);
        let confidence = self.calculate_confidence(&pr_data);
        
        Ok(Decision::from_confidence(confidence))
    }
    
    async fn fetch_pr_data(&self, pr_number: u64) -> Result<PrData>;
    fn check_blockers(&self, labels: &[String]) -> bool;
    async fn check_ci_status(&self, checks: &[Check]) -> Result<bool>;
    fn calculate_risk(&self, files: &[String]) -> u8;
    fn calculate_confidence(&self, pr_data: &PrData) -> u8;
}
```

### Fase 2: CLI Integration

**Command:**
```bash
workflow-orchestrator guardian \
  --pr-number <NUMBER> \
  --threshold 70 \
  --ci-mode \
  --dry-run
```

### Fase 3: Workflow Update

**Cambio en `.github/workflows/guardian-agent.yml`:**
```yaml
- name: 🛡️ Run Guardian Core
  run: |
    if command -v workflow-orchestrator &> /dev/null; then
      workflow-orchestrator guardian \
        --pr-number ${{ steps.pr.outputs.number }} \
        --ci-mode
    else
      # Fallback to PowerShell
      pwsh ./scripts/guardian-core.ps1 -PrNumber ${{ steps.pr.outputs.number }}
    fi
```

## 📦 Dependencies

```toml
[dependencies]
octocrab = "0.38"           # GitHub API
tokio = "1"                 # Async runtime
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
regex = "1"
```

## ✅ Criterios de Aceptación

- [x] Módulo `guardian_core.rs` implementado en `workflow-orchestrator`
- [x] Tests unitarios con cobertura >80%
- [ ] Integration tests con mock GitHub API
- [ ] Benchmarks muestran >10x speedup vs PowerShell
- [x] Workflow actualizado con fallback a PowerShell
- [ ] CI pasa en Linux, macOS, Windows
- [ ] Documentación actualizada
- [ ] A/B testing por 1 semana sin errores

## 📝 Tareas

- [x] Diseñar struct `GuardianCore` y tipos relacionados
- [x] Implementar `fetch_pr_data()` usando octocrab
- [x] Implementar `check_blockers()` lógica
- [x] Implementar `check_ci_status()` verificación
- [x] Implementar `calculate_risk()` usando risk-map.json
- [x] Implementar `calculate_confidence()` scoring
- [x] Agregar CLI subcommand `guardian` a main.rs
- [x] Escribir unit tests para cada función (4 tests - 100% pass)
- [x] Escribir integration tests con mock API (8 tests - 100% pass)
- [x] Crear benchmarks comparativos (5 benchmarks - <200ns vs 2-3s PowerShell)
- [x] Actualizar workflow con fallback logic
- [ ] Testing en staging branch
- [ ] Documentar en README de workflow-orchestrator

## 🔗 Referencias

- PowerShell original: `scripts/guardian-core.ps1`
- Workflow actual: `.github/workflows/guardian-agent.yml`
- Risk map: `.✨/risk-map.json`
- Base orchestrator: `tools/workflow-orchestrator/`

## 🎯 Roadmap

**Sprint 1 (Semana 1):**
- Implementación core del módulo
- Tests básicos

**Sprint 2 (Semana 2):**
- Integration tests
- Workflow update con fallback
- A/B testing

**Sprint 3 (Semana 3):**
- Full cutover
- Monitoreo y optimización

---

**AI-Context:** Este issue es parte de la estrategia de migración a Rust para workflows críticos. Guardian Agent es el candidato #1 por su frecuencia de ejecución (cada PR) y path crítico en el proceso de auto-merge.
