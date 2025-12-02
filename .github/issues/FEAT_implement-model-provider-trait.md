---
github_issue: 33
title: "Implement ModelProvider trait with Gemini, Bedrock, CopilotCLI"
labels:
  - ai-plan
  - enhancement
assignees: []
---

## Descripción

Implementar el trait `ModelProvider` que permita conectar con múltiples proveedores de modelos de lenguaje.

## Proveedores a Implementar

1. **Gemini** - Google AI
2. **Bedrock** - AWS
3. **CopilotCLI** - GitHub/OpenAI

## Interface Propuesta

```rust
#[async_trait]
pub trait ModelProvider {
    async fn generate(&self, prompt: &str) -> Result<String>;
    async fn stream(&self, prompt: &str) -> Result<impl Stream<Item = String>>;
    fn name(&self) -> &str;
}
```

## Criterios de Aceptación

- [ ] Trait definido con métodos async
- [ ] Implementación para Gemini
- [ ] Implementación para Bedrock
- [ ] Implementación para CopilotCLI
- [ ] Tests de integración

