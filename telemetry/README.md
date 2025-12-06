# 📡 Federated Telemetry System

This directory receives **anonymized metrics** from projects using the Git-Core Protocol worldwide.

## How It Works

```
┌─────────────────┐    PR with metrics    ┌─────────────────────┐
│  Your Project   │ ─────────────────────▶│ Official Git-Core   │
│  (uses protocol)│                       │ Protocol Repo       │
└─────────────────┘                       └─────────────────────┘
                                                    │
                                                    ▼
                                          ┌─────────────────────┐
                                          │ Aggregate Analysis  │
                                          │ • Pattern Detection │
                                          │ • Protocol Improve  │
                                          └─────────────────────┘
```

## For Protocol Users

Send your project's metrics:

```powershell
# In your project directory
./scripts/send-telemetry.ps1

# Preview without sending
./scripts/send-telemetry.ps1 -DryRun
```

## Data Collected

| Category | Metrics | Purpose |
|----------|---------|---------|
| **Order 1** | Issues opened/closed, PRs merged | Workflow health |
| **Order 2** | Agent-state usage %, Atomic commit ratio | Protocol adoption |
| **Order 3** | Friction reports, Evolution proposals | Pain points |

## Privacy

- **Anonymous by default:** Project names are hashed
- **No code is sent:** Only aggregate numbers
- **Opt-in:** You choose when to send

## File Format

```json
{
  "schema_version": "1.0",
  "project_id": "anon-a1b2c3d4",
  "anonymous": true,
  "timestamp": "2025-12-05T18:00:00Z",
  "week": 49,
  "year": 2025,
  "protocol_version": "2.1",
  "order1": {
    "issues_open": 5,
    "issues_closed_total": 42,
    "prs_merged_total": 28
  },
  "order2": {
    "agent_state_usage_pct": 75,
    "atomic_commit_ratio": 82
  },
  "order3": {
    "friction_reports": 2,
    "evolution_proposals": 1
  }
}
```

## Benefits of Contributing

1. **Help improve the protocol** for everyone
2. **Identify common friction points** across projects
3. **Drive data-informed decisions** for new features
4. **Benchmark your project** against the community
