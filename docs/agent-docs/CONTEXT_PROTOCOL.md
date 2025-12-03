---
title: "Context Protocol Specification"
type: SPECIFICATION
id: "spec-context-protocol"
created: 2025-12-03
updated: 2025-12-03
agent: protocol-gemini
model: gemini-3-pro
requested_by: system
summary: |
  Standardized protocol for agents to persist state and context in GitHub Issues,
  enabling stateless, pausable, and resumable workflows (12-Factor Agents).
keywords: [context, protocol, state, xml, 12-factor]
tags: ["#protocol", "#context", "#state"]
project: Git-Core-Protocol
---

# 🧠 Context Protocol (Git-Core v2.0)

> **"Own Your Context Window & Unify State"**

Este protocolo define cómo los agentes de IA deben persistir su estado de ejecución y contexto dentro de los GitHub Issues. Esto transforma a los agentes en **Stateless Reducers** (Factor 12) y permite flujos de trabajo pausables y resumibles.

## 1. Principio Fundamental

**El Agente es una Función Pura:**
`Agente(Historial del Issue + Contexto del Repo) -> Siguiente Acción`

No debe existir "memoria" oculta en el chat. Todo el estado necesario para continuar una tarea debe estar visible en el Issue.

## 2. Bloque de Estado del Agente (`<agent-state>`)

Cada vez que un agente realiza una acción significativa o pausa su trabajo, DEBE dejar un comentario en el Issue con un bloque `<agent-state>`.

### Formato XML

```xml
<agent-state>
  <!-- Intención actual del agente -->
  <intent>implement_feature</intent>

  <!-- Paso actual en el plan -->
  <step>waiting_for_review</step>

  <!-- Progreso (0-100) -->
  <progress>60</progress>

  <!-- Memoria a corto plazo (JSON) -->
  <memory>
    {
      "last_file_edited": "src/auth.ts",
      "test_status": "failed",
      "attempt_count": 2
    }
  </memory>

  <!-- Siguiente acción recomendada -->
  <next_action>run_tests_again</next_action>
</agent-state>
```

### Campos Permitidos

| Campo | Descripción | Valores Ejemplo |
|-------|-------------|-----------------|
| `<intent>` | Objetivo de alto nivel | `fix_bug`, `refactor`, `deploy` |
| `<step>` | Estado actual del flujo | `planning`, `coding`, `testing`, `blocked` |
| `<progress>` | Porcentaje estimado | `0` a `100` |
| `<memory>` | Datos clave para retomar | JSON string |
| `<next_action>` | Sugerencia para el siguiente agente | `review_pr`, `merge`, `ask_user` |

## 3. Flujo de Lectura/Escritura

### Al Iniciar (Lectura)

1. El agente lee el Issue asignado.
2. Busca el **último** bloque `<agent-state>` en los comentarios.
3. Si existe, carga ese estado como su contexto inicial.
4. Si no existe, inicia con estado `planning`.

### Al Finalizar/Pausar (Escritura)

1. El agente determina su nuevo estado.
2. Publica un comentario en el Issue con el bloque `<agent-state>` actualizado.
3. (Opcional) Añade un resumen legible para humanos antes del bloque XML.

## 4. Ejemplo de Interacción

**Agente 1 (Claude):**
> He implementado la autenticación básica. Los tests unitarios pasan, pero falta integración.
>
> ```xml
> <agent-state>
>   <intent>add_auth</intent>
>   <step>testing</step>
>   <progress>70</progress>
>   <memory>{"files_changed": ["auth.ts"], "pending": "integration_tests"}</memory>
> </agent-state>
> ```

*(Pasa el tiempo...)*

**Agente 2 (Gemini):**
*(Lee el último estado)*
> Veo que falta la integración. Voy a ejecutar los tests de integración ahora.

## 5. Beneficios (12-Factor Alignment)

- **Factor 3 (Own Context):** Estructura determinística para el LLM.
- **Factor 5 (Unified State):** Ejecución y negocio unidos en el Issue.
- **Factor 6 (Pausable):** Cualquier agente puede retomar el trabajo.
- **Factor 12 (Stateless):** El agente no necesita memoria de sesión.
