# Email Handler Agent (Fallback)

> ⚠️ **NOTA:** Este agente es un **método de fallback**. El método principal recomendado es el workflow `.github/workflows/self-healing.yml` que usa eventos `workflow_run` nativos de GitHub Actions.

Este agente monitorea tu bandeja de entrada de Gmail en busca de notificaciones de fallos de CI/CD (GitHub, Vercel) y desencadena acciones de reparación automática.

## ⚡ Cuándo Usar Este Método

- ✅ GitHub Actions está experimentando downtime
- ✅ Necesitas monitorear notificaciones de servicios externos (Vercel, Netlify)
- ✅ Quieres un backup independiente de GitHub

## 🚫 Cuándo NO Usar Este Método

- ❌ Solo tienes workflows de GitHub Actions (usa `self-healing.yml` en su lugar)
- ❌ Quieres latencia baja (< 1 minuto)
- ❌ No quieres configurar credenciales de Gmail

## 🔧 Configuración

### Requisitos Previos

- Python 3.8+
- Cuenta de Google Cloud (proyecto gratuito)
- Gmail API habilitada

### 1. Google Cloud Console

Para usar la API de Gmail, necesitas credenciales:

1. Ve a [Google Cloud Console](https://console.cloud.google.com/).
2. Crea un nuevo proyecto (ej. `git-core-email-bot`).
3. Habilita la **Gmail API**.
4. Ve a "Credenciales" > "Crear Credenciales" > "ID de cliente de OAuth".
5. Configura la pantalla de consentimiento (User Type: External, Test Users: tu email).
6. Descarga el JSON de credenciales y guárdalo como `credentials.json` en esta carpeta (`tools/email-handler/`).

### 2. Instalación

```bash
cd tools/email-handler
pip install -r requirements.txt
```

### 3. Uso

```bash
python src/main.py
```

El script abrirá una ventana del navegador la primera vez para autorizar el acceso a tu cuenta de Gmail.

## 🎯 Características Actuales

- ✅ Lectura de correos no leídos de `notifications@github.com`
- ✅ Filtrado por subject: `"Run failed"`
- ✅ Extracción de información: repo, workflow, commit
- ✅ Clasificación de errores (transient/dependency/code)
- ⚠️ Auto-acciones: **Próximamente** (actualmente solo detecta)

## 🚀 Roadmap

- [ ] Integrar con `gh run rerun` para reintentos automáticos
- [ ] Crear issues automáticamente para errores de código
- [ ] Archivar/eliminar correos después de resolver el problema
- [ ] Modo watch (ejecutar cada N minutos)
- [ ] Soporte para notificaciones de Vercel/Netlify

## 📊 Comparación: Email vs workflow_run

| Aspecto | Email Handler | workflow_run Event |
|---------|---------------|-------------------|
| Latencia | 5-60 minutos | < 1 minuto |
| Setup | OAuth complejo | Archivo YAML simple |
| Costo | Gmail API quota | $0 (GitHub Actions) |
| Escalabilidad | 1 script/cuenta | Multi-repo nativo |
| **Recomendación** | ⚠️ Fallback | ✅ **Método Principal** |

## 🔗 Documentación Relacionada

- [RESEARCH_SELFHEALING_CICD.md](../../docs/agent-docs/RESEARCH_SELFHEALING_CICD.md) - Comparación exhaustiva de métodos
- [.github/workflows/self-healing.yml](../../.github/workflows/self-healing.yml) - Implementación recomendada
