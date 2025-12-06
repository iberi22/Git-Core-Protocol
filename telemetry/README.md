# 📡 Federated Telemetry System v2

> **Scalable architecture for 1,000+ users**

This directory manages the **federated telemetry system** that collects anonymized metrics from projects using Git-Core Protocol worldwide.

## Architecture (v2 - Discussion-Based)

```
┌─────────────────┐   Discussion (GraphQL)   ┌─────────────────────┐
│  Your Project   │ ─────────────────────────▶│ GitHub Discussions │
│  (uses protocol)│                           │ Category: Telemetry│
└─────────────────┘                           └─────────────────────┘
                                                       │
┌─────────────────┐   Discussion (GraphQL)             │
│ Another Project │ ─────────────────────────▶─────────┤
└─────────────────┘                                    │
                                                       │ Weekly Workflow
┌─────────────────┐   Discussion (GraphQL)             │ (aggregate-telemetry.yml)
│  Project N      │ ─────────────────────────▶─────────┤
└─────────────────┘                                    │
                                                       ▼
                                              ┌─────────────────────┐
                                              │ 1 Issue per Week    │
                                              │ "[Evolution] Week X" │
                                              │ • N projects        │
                                              │ • Aggregated metrics│
                                              │ • Ecosystem patterns│
                                              └─────────────────────┘
```

## Why Discussions Instead of PRs?

| Approach | 10 Users | 1,000 Users | 10,000 Users |
|----------|----------|-------------|--------------|
| **PRs (v1)** | 10 PRs/week | 1,000 PRs/week ❌ | 10,000 PRs/week 💀 |
| **Discussions (v2)** | 10 Discussions | 1,000 Discussions ✅ | 10,000 Discussions ✅ |

**Benefits:**
- ✅ Discussions don't flood the PR feed
- ✅ No Actions minutes consumed for each submission
- ✅ Single aggregated report per week
- ✅ Scales infinitely
- ✅ Transparent and auditable

## Usage

### Send Telemetry (For Protocol Users)

```powershell
# Preview what would be sent
./scripts/send-telemetry.ps1 -DryRun

# Send anonymized metrics (creates a Discussion)
./scripts/send-telemetry.ps1

# Include detected patterns
./scripts/send-telemetry.ps1 -IncludePatterns
```

### Aggregate Metrics (Automatic)

The `aggregate-telemetry.yml` workflow runs every Monday at 10:00 UTC:
1. Fetches all Discussions from the "Telemetry" category
2. Filters by the target week
3. Aggregates metrics from all submissions
4. Creates a single Evolution Report issue

Manual trigger:
```bash
gh workflow run aggregate-telemetry.yml
```

## Data Collected

| Category | Metrics | Purpose |
|----------|---------|---------|
| **Order 1** | Issues opened/closed, PRs merged | Workflow health |
| **Order 2** | Agent-state usage %, Atomic commit ratio | Protocol adoption |
| **Order 3** | Friction reports, Evolution proposals | Pain points |

## Privacy

- **Anonymous by default:** Project names are SHA256 hashed (`anon-a1b2c3d4`)
- **No code is sent:** Only aggregate numbers
- **No personal data:** No usernames, emails, or identifiable info
- **Opt-in only:** You choose when to send
- **Revocable:** Delete your Discussion to remove your data

## Schema v2.0

```json
{
  "schema_version": "2.0",
  "submission_method": "discussion",
  "project_id": "anon-a1b2c3d4",
  "anonymous": true,
  "timestamp": "2025-12-06T00:00:00Z",
  "week": 49,
  "year": 2025,
  "protocol_version": "2.1",
  "order1": {
    "issues_open": 5,
    "issues_closed_total": 42,
    "prs_merged_total": 28
  },
  "order2": {
    "agent_state_usage_pct": 75.0,
    "atomic_commit_ratio": 82.0,
    "sample_size": 10
  },
  "order3": {
    "friction_reports": 2,
    "evolution_proposals": 1
  },
  "patterns": ["low_atomic_commit_ratio"]
}
```

## Ecosystem Patterns Detected

The aggregation workflow detects these patterns:

| Pattern | Trigger | Action |
|---------|---------|--------|
| Low adoption | `avg_agent_state_usage < 50%` | Improve documentation |
| Excellent adoption | `avg_agent_state_usage >= 80%` | Celebrate! 🎉 |
| Low atomicity | `avg_atomic_commit_ratio < 70%` | Strengthen CI validation |
| High friction | `total_friction_reports > 10` | Prioritize usability fixes |

## Opt-Out

To stop sending telemetry:
1. **Simply don't run the script** - it's manually triggered
2. **Remove the workflow** from your project if you forked it
3. **Delete your Discussions** to remove historical data

## Files

| File | Purpose |
|------|---------|
| `README.md` | This documentation |
| `../scripts/send-telemetry.ps1` | Script to send metrics |
| `../.github/workflows/aggregate-telemetry.yml` | Weekly aggregation |

## Changelog

### v2.0 (2025-12-06)
- 🔄 **Breaking:** Switched from PRs to Discussions
- ✨ New: `schema_version: 2.0` with `submission_method` field
- ✨ New: Ecosystem pattern detection
- ✨ New: Single aggregated report per week
- 🗑️ Deprecated: `telemetry/submissions/` directory (no longer used)

### v1.0 (2025-12-05)
- Initial PR-based implementation
