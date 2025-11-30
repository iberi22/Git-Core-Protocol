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
