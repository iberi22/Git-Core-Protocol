<#
.SYNOPSIS
    Sends anonymized protocol metrics to the official Git-Core Protocol repository.

.DESCRIPTION
    This script collects evolution metrics from the current project and submits
    them as a PR to the official Git-Core Protocol repository for centralized
    analysis and protocol improvement.

.PARAMETER DryRun
    If set, shows what would be sent without actually creating the PR.

.PARAMETER Anonymous
    If set, removes project identifiers (default: true).

.PARAMETER IncludePatterns
    If set, includes detected patterns in the telemetry.

.EXAMPLE
    ./send-telemetry.ps1
    # Sends anonymized metrics to official repo

.EXAMPLE
    ./send-telemetry.ps1 -DryRun
    # Preview what would be sent
#>

param(
    [switch]$DryRun,
    [bool]$Anonymous = $true,
    [switch]$IncludePatterns
)

$ErrorActionPreference = "Stop"

$OFFICIAL_REPO = "iberi22/Git-Core-Protocol"
$TELEMETRY_BRANCH_PREFIX = "telemetry"
$TELEMETRY_DIR = "telemetry/submissions"

Write-Host "📡 Git-Core Protocol - Federated Telemetry System" -ForegroundColor Cyan
Write-Host "   Destination: github.com/$OFFICIAL_REPO" -ForegroundColor Gray

# ============================================
# 1. COLLECT LOCAL METRICS
# ============================================
Write-Host "`n📊 Collecting local metrics..." -ForegroundColor Yellow

$timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ"
$weekNumber = [System.Globalization.ISOWeek]::GetWeekOfYear((Get-Date))
$year = (Get-Date).Year

# Get current repo info
$repoUrl = git config --get remote.origin.url 2>$null
$repoName = if ($repoUrl) {
    $repoUrl -replace ".*[:/]([^/]+/[^/]+?)(.git)?$", '$1'
} else {
    "unknown"
}

# Anonymize if requested
$projectId = if ($Anonymous) {
    # Generate hash from repo name for anonymous but consistent ID
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($repoName)
    $hash = [System.Security.Cryptography.SHA256]::Create().ComputeHash($bytes)
    "anon-" + [BitConverter]::ToString($hash[0..3]).Replace("-", "").ToLower()
} else {
    $repoName
}

Write-Host "   Project ID: $projectId" -ForegroundColor Gray

# Collect metrics
$metrics = @{
    schema_version = "1.0"
    project_id = $projectId
    anonymous = $Anonymous
    timestamp = $timestamp
    week = $weekNumber
    year = $year
    protocol_version = "2.1"
    order1 = @{}
    order2 = @{}
    order3 = @{}
}

# Order 1: Operational
try {
    $issuesOpen = (gh issue list --state open --json number 2>$null | ConvertFrom-Json).Count
    $issuesClosed = (gh issue list --state closed --limit 100 --json number 2>$null | ConvertFrom-Json).Count
    $prsOpen = (gh pr list --state open --json number 2>$null | ConvertFrom-Json).Count
    $prsMerged = (gh pr list --state merged --limit 100 --json number 2>$null | ConvertFrom-Json).Count

    $metrics.order1 = @{
        issues_open = $issuesOpen
        issues_closed_total = $issuesClosed
        prs_open = $prsOpen
        prs_merged_total = $prsMerged
    }
    Write-Host "   ✓ Order 1 metrics collected" -ForegroundColor Green
} catch {
    Write-Warning "   Could not collect Order 1 metrics: $_"
}

# Order 2: Quality
try {
    # Check for agent-state usage in recent issues
    $recentIssues = gh issue list --limit 10 --json number 2>$null | ConvertFrom-Json
    $agentStateCount = 0

    foreach ($issue in $recentIssues) {
        $issueBody = gh issue view $issue.number --json body 2>$null | ConvertFrom-Json
        if ($issueBody.body -match "<agent-state>") {
            $agentStateCount++
        }
    }

    $usagePct = if ($recentIssues.Count -gt 0) {
        [math]::Round(($agentStateCount / $recentIssues.Count) * 100, 1)
    } else { 0 }

    # Check atomic commits
    $commits = git log --oneline -50 2>$null
    $totalCommits = ($commits | Measure-Object -Line).Lines
    $atomicCommits = ($commits | Where-Object { $_ -match "^[a-f0-9]+ (feat|fix|docs|style|refactor|test|chore)\(" }).Count
    $atomicRatio = if ($totalCommits -gt 0) {
        [math]::Round(($atomicCommits / $totalCommits) * 100, 1)
    } else { 0 }

    $metrics.order2 = @{
        agent_state_usage_pct = $usagePct
        atomic_commit_ratio = $atomicRatio
        sample_size = $recentIssues.Count
    }
    Write-Host "   ✓ Order 2 metrics collected" -ForegroundColor Green
} catch {
    Write-Warning "   Could not collect Order 2 metrics: $_"
}

# Order 3: Evolution
try {
    $frictionCount = (gh issue list --label "friction" --state all --json number 2>$null | ConvertFrom-Json).Count
    $evolutionCount = (gh issue list --label "evolution" --state all --json number 2>$null | ConvertFrom-Json).Count

    $metrics.order3 = @{
        friction_reports = $frictionCount
        evolution_proposals = $evolutionCount
    }
    Write-Host "   ✓ Order 3 metrics collected" -ForegroundColor Green
} catch {
    Write-Warning "   Could not collect Order 3 metrics: $_"
}

# Detect patterns (optional)
if ($IncludePatterns) {
    $patterns = @()

    if ($metrics.order2.agent_state_usage_pct -lt 50) {
        $patterns += "low_agent_state_adoption"
    }
    if ($metrics.order2.atomic_commit_ratio -lt 70) {
        $patterns += "low_atomic_commit_ratio"
    }
    if ($metrics.order3.friction_reports -gt 5) {
        $patterns += "high_friction"
    }

    $metrics.patterns = $patterns
}

# ============================================
# 2. GENERATE TELEMETRY FILE
# ============================================
$telemetryJson = $metrics | ConvertTo-Json -Depth 10
$fileName = "$($projectId)_week$($weekNumber)_$($year).json"

Write-Host "`n📄 Generated telemetry file: $fileName" -ForegroundColor Yellow
Write-Host $telemetryJson

if ($DryRun) {
    Write-Host "`n🔍 DRY RUN - No PR will be created" -ForegroundColor Magenta
    return
}

# ============================================
# 3. FORK & CREATE PR TO OFFICIAL REPO
# ============================================
Write-Host "`n🚀 Creating PR to $OFFICIAL_REPO..." -ForegroundColor Yellow

# Create temp directory
$tempDir = Join-Path $env:TEMP "git-core-telemetry-$(Get-Random)"
New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

try {
    Push-Location $tempDir

    # Clone official repo (sparse checkout for speed)
    Write-Host "   Cloning official repo..." -ForegroundColor Gray
    git clone --depth 1 --filter=blob:none --sparse "https://github.com/$OFFICIAL_REPO.git" repo 2>$null
    Set-Location repo
    git sparse-checkout set $TELEMETRY_DIR 2>$null

    # Create telemetry directory if not exists
    $telemetryPath = Join-Path (Get-Location) $TELEMETRY_DIR
    if (-not (Test-Path $telemetryPath)) {
        New-Item -ItemType Directory -Path $telemetryPath -Force | Out-Null
    }

    # Create branch
    $branchName = "$TELEMETRY_BRANCH_PREFIX/$projectId-week$weekNumber-$year"
    git checkout -b $branchName 2>$null

    # Write telemetry file
    $filePath = Join-Path $telemetryPath $fileName
    $telemetryJson | Out-File -FilePath $filePath -Encoding utf8

    # Commit
    git add .
    git commit -m "telemetry: add metrics from $projectId (week $weekNumber, $year)" 2>$null

    # Push (requires user has fork or push access)
    Write-Host "   Pushing to fork..." -ForegroundColor Gray
    git push origin $branchName 2>$null

    # Create PR
    Write-Host "   Creating Pull Request..." -ForegroundColor Gray
    $prBody = @"
## 📡 Federated Telemetry Submission

**Project ID:** $projectId
**Week:** $weekNumber ($year)
**Protocol Version:** 2.1

### Metrics Summary

| Category | Key Metrics |
|----------|-------------|
| **Order 1** | Issues: $($metrics.order1.issues_open) open, $($metrics.order1.issues_closed_total) closed |
| **Order 2** | Agent-State: $($metrics.order2.agent_state_usage_pct)%, Atomic Commits: $($metrics.order2.atomic_commit_ratio)% |
| **Order 3** | Friction: $($metrics.order3.friction_reports), Evolution: $($metrics.order3.evolution_proposals) |

---
*Auto-generated by Git-Core Protocol Telemetry System*
"@

    gh pr create --repo $OFFICIAL_REPO --title "telemetry: $projectId week $weekNumber ($year)" --body $prBody --label "telemetry" 2>$null

    Write-Host "`n✅ Telemetry PR created successfully!" -ForegroundColor Green

} catch {
    Write-Error "Failed to create telemetry PR: $_"
} finally {
    Pop-Location
    # Cleanup
    if (Test-Path $tempDir) {
        Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
    }
}
