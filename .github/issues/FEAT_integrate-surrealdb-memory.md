---
github_issue: 34
title: "Integrate SurrealDB as unified memory system (vectors + graphs)"
labels:
  - ai-plan
  - enhancement
assignees: []
---

## Descripción

Integrar SurrealDB como sistema de memoria unificado que soporte tanto vectores como grafos.

## Motivación

SurrealDB ofrece:
- **Vectores**: Embeddings para búsqueda semántica
- **Grafos**: Relaciones entre conceptos y entidades
- **SQL-like**: Queries familiares
- **Tiempo real**: Suscripciones a cambios

## Arquitectura

```
┌─────────────────────────────────────┐
│         Research Agent              │
├─────────────────────────────────────┤
│  ┌─────────┐  ┌─────────────────┐   │
│  │ Vectors │  │ Knowledge Graph │   │
│  │ (embed) │  │   (relations)   │   │
│  └────┬────┘  └────────┬────────┘   │
│       │                │            │
│       └───────┬────────┘            │
│               ▼                     │
│        ┌──────────────┐             │
│        │   SurrealDB  │             │
│        └──────────────┘             │
└─────────────────────────────────────┘
```

## Tareas

- [ ] Setup SurrealDB connection
- [ ] Definir schema para vectores
- [ ] Definir schema para grafos
- [ ] Implementar queries híbridas
- [ ] Tests de rendimiento

