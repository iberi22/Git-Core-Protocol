---
title: "System Architecture"
type: ARCHITECTURE
id: "arch-system"
created: 2025-12-01
updated: 2025-12-01
agent: copilot
model: gemini-3-pro
requested_by: system
summary: |
  Critical architectural decisions and system design.
keywords: [architecture, design, decisions, system]
tags: ["#architecture", "#design", "#critical"]
project: Git-Core-Protocol
---

# 🏗️ Architecture

## 🚨 CRITICAL DECISIONS - READ FIRST

> ⚠️ **STOP!** Before implementing ANY feature, verify against this table.
> These decisions are NON-NEGOTIABLE.

| # | Category | Decision | Rationale | ❌ NEVER Use |
|---|----------|----------|-----------|--------------|
| 1 | Hosting | GitHub Pages | Zero cost, Git-native | Vercel, Netlify, AWS |
| 2 | Backend | Supabase | BaaS, realtime | Firebase, custom API |
| 3 | State | GitHub Issues | Token economy | TODO.md, JIRA |

### How to use this table:
1. **Before ANY implementation**, check if it conflicts with decisions above
2. If issue mentions alternatives (e.g., "Vercel/GitHub Pages"), the decision above WINS
3. When in doubt, ASK - don't assume

**Related Documentation:**
- `AGENTS.md` - Architecture Verification Rule
- `.github/copilot-instructions.md` - Architecture First Rule

---

## Stack
- **Language:** TBD
- **Framework:** TBD
- **Database:** TBD
- **Infrastructure:** TBD

## Key Decisions

### Decision 1: [Title]
- **Date:** YYYY-MM-DD
- **Context:** Why this decision was needed
- **Decision:** What was decided
- **Consequences:** Impact and trade-offs

### Decision 2: Telemetry Migration to Rust
- **Date:** 2025-12-16
- **Context:** Telemetry logic was isolated in PowerShell, causing fragmentation and platform dependencies.
- **Decision:** Migrated client-side telemetry to `gc telemetry` command in Rust.
- **Consequences:** Unified toolchain in `gc-cli`, removed PowerShell dependency for telemetry submission. Legacy script `send-telemetry.ps1` is deprecated.

### Decision 3: CLI Unification
- **Date:** 2025-12-16
- **Context:** Multiple PowerShell scripts (`init_project.ps1`, `equip-agent.ps1`, `ai-report.ps1`) created maintenance overhead and platform lock-in.
- **Decision:** Consolidated all core workflows into `gc-cli` Rust binary (`gc init`, `gc context`, `gc report`, `gc ci-detect`).
- **Consequences:** All legacy PowerShell scripts are deprecated. Future development focuses solely on `gc-cli`.

## Project Structure
```
/
├── src/          # Source code
├── tests/        # Test files
├── docs/         # Documentation
└── scripts/      # Utility scripts
```

## Dependencies
| Package | Version | Purpose |
|---------|---------|----------|
| TBD     | x.x.x   | TBD      |

## Integration Points
- [ ] External API 1
- [ ] External API 2

## Security Considerations
- TBD

---
*Last updated by AI Agent: Never*

