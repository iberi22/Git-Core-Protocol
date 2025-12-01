# install.ps1 - Remote installer for Git-Core Protocol (Windows)
# Usage: irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
#
# 🎯 This script can be executed by AI agents to bootstrap any project with Git-Core Protocol
#
# Environment variables for options:
#   $env:GIT_CORE_ORGANIZE = "1"  - Organize existing files
#   $env:GIT_CORE_AUTO = "1"      - Non-interactive mode
#   $env:GIT_CORE_UPGRADE = "1"   - Upgrade (preserves ARCHITECTURE.md)
#   $env:GIT_CORE_FORCE = "1"     - Force upgrade (overwrites everything)

$ErrorActionPreference = "Stop"

$REPO_URL = "https://github.com/iberi22/Git-Core-Protocol"
$RAW_URL = "https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main"
$TEMP_DIR = ".git-core-temp"
$BACKUP_DIR = ".git-core-backup"

Write-Host "🧠 Git-Core Protocol - Remote Installer v2.0" -ForegroundColor Cyan
Write-Host "==============================================" -ForegroundColor Cyan
Write-Host ""

# Check for environment variable flags
$OrganizeFiles = $env:GIT_CORE_ORGANIZE -eq "1"
$AutoMode = $env:GIT_CORE_AUTO -eq "1"
$UpgradeMode = $env:GIT_CORE_UPGRADE -eq "1"
$ForceMode = $env:GIT_CORE_FORCE -eq "1"

# Force implies upgrade and auto
if ($ForceMode) {
    $UpgradeMode = $true
    $AutoMode = $true
    Write-Host "⚠️  FORCE MODE: ALL files will be overwritten (including ARCHITECTURE.md)" -ForegroundColor Red
    Write-Host ""
} elseif ($UpgradeMode) {
    $AutoMode = $true
    Write-Host "🔄 UPGRADE MODE: Protocol files updated, your ARCHITECTURE.md preserved" -ForegroundColor Yellow
    Write-Host ""
}

# Function to get current version
function Get-CurrentVersion {
    if (Test-Path ".git-core-protocol-version") {
        return (Get-Content ".git-core-protocol-version" -Raw).Trim()
    }
    return "0.0.0"
}

# Function to get remote version
function Get-RemoteVersion {
    try {
        $response = Invoke-WebRequest -Uri "$RAW_URL/.git-core-protocol-version" -UseBasicParsing -ErrorAction SilentlyContinue
        return $response.Content.Trim()
    } catch {
        return "unknown"
    }
}

# Show version info
$CurrentVersion = Get-CurrentVersion
if ($CurrentVersion -ne "0.0.0") {
    $RemoteVersion = Get-RemoteVersion
    Write-Host "📊 Version Info:" -ForegroundColor Blue
    Write-Host "   Current: $CurrentVersion" -ForegroundColor Yellow
    Write-Host "   Latest:  $RemoteVersion" -ForegroundColor Green
    Write-Host ""
}

# Function to backup user files
function Backup-UserFiles {
    Write-Host "💾 Backing up user files..." -ForegroundColor Cyan
    New-Item -ItemType Directory -Force -Path $BACKUP_DIR | Out-Null
    
    # Backup ARCHITECTURE.md
    if (Test-Path ".✨/ARCHITECTURE.md") {
        Copy-Item ".✨/ARCHITECTURE.md" "$BACKUP_DIR/ARCHITECTURE.md"
        Write-Host "  ✓ .✨/ARCHITECTURE.md backed up" -ForegroundColor Green
    }
    
    # Backup CONTEXT_LOG.md
    if (Test-Path ".✨/CONTEXT_LOG.md") {
        Copy-Item ".✨/CONTEXT_LOG.md" "$BACKUP_DIR/CONTEXT_LOG.md"
        Write-Host "  ✓ .✨/CONTEXT_LOG.md backed up" -ForegroundColor Green
    }
    
    # Backup custom workflows
    if (Test-Path ".github/workflows") {
        New-Item -ItemType Directory -Force -Path "$BACKUP_DIR/workflows" | Out-Null
        $protocolWorkflows = @("update-protocol.yml", "structure-validator.yml", "codex-review.yml", "agent-dispatcher.yml")
        
        Get-ChildItem ".github/workflows/*.yml" -ErrorAction SilentlyContinue | ForEach-Object {
            if ($_.Name -notin $protocolWorkflows) {
                Copy-Item $_.FullName "$BACKUP_DIR/workflows/"
                Write-Host "  ✓ Custom workflow: $($_.Name)" -ForegroundColor Green
            }
        }
    }
}

# Function to restore user files
function Restore-UserFiles {
    Write-Host "📥 Restoring user files..." -ForegroundColor Cyan
    
    # Restore ARCHITECTURE.md (unless force mode)
    if (-not $ForceMode -and (Test-Path "$BACKUP_DIR/ARCHITECTURE.md")) {
        Copy-Item "$BACKUP_DIR/ARCHITECTURE.md" ".✨/ARCHITECTURE.md" -Force
        Write-Host "  ✓ .✨/ARCHITECTURE.md restored" -ForegroundColor Green
    }
    
    # Always restore CONTEXT_LOG.md
    if (Test-Path "$BACKUP_DIR/CONTEXT_LOG.md") {
        Copy-Item "$BACKUP_DIR/CONTEXT_LOG.md" ".✨/CONTEXT_LOG.md" -Force
        Write-Host "  ✓ .✨/CONTEXT_LOG.md restored" -ForegroundColor Green
    }
    
    # Restore custom workflows
    if (Test-Path "$BACKUP_DIR/workflows") {
        Get-ChildItem "$BACKUP_DIR/workflows/*.yml" -ErrorAction SilentlyContinue | ForEach-Object {
            Copy-Item $_.FullName ".github/workflows/" -Force
            Write-Host "  ✓ Custom workflow restored: $($_.Name)" -ForegroundColor Green
        }
    }
    
    # Cleanup backup
    Remove-Item -Recurse -Force $BACKUP_DIR -ErrorAction SilentlyContinue
}

# Function to organize existing files
function Invoke-OrganizeFiles {
    Write-Host "📂 Organizing existing files..." -ForegroundColor Yellow

    $dirs = @("docs/archive", "scripts", "tests", "src")
    foreach ($dir in $dirs) {
        New-Item -ItemType Directory -Force -Path $dir -ErrorAction SilentlyContinue | Out-Null
    }

    $keepInRoot = @("README.md", "AGENTS.md", "CHANGELOG.md", "CONTRIBUTING.md", "LICENSE.md", "LICENSE")

    Get-ChildItem -Filter "*.md" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.Name -notin $keepInRoot) {
            Move-Item $_.FullName -Destination "docs/archive/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) moved to docs/archive/" -ForegroundColor Cyan
        } else {
            Write-Host "  ✓ Keeping $($_.Name) in root" -ForegroundColor Green
        }
    }

    $testPatterns = @("test_*.py", "*_test.py", "*.test.js", "*.test.ts", "*.spec.js", "*.spec.ts")
    foreach ($pattern in $testPatterns) {
        Get-ChildItem -Filter $pattern -File -ErrorAction SilentlyContinue | ForEach-Object {
            Move-Item $_.FullName -Destination "tests/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) moved to tests/" -ForegroundColor Cyan
        }
    }

    Write-Host "✅ Files organized" -ForegroundColor Green
}

# Check if should organize
if ($OrganizeFiles) {
    Invoke-OrganizeFiles
}

# Check if directory has files
$hasFiles = (Get-ChildItem -File -ErrorAction SilentlyContinue | Where-Object { $_.Name -notlike ".*" } | Measure-Object).Count -gt 0

if ($hasFiles -and -not $AutoMode) {
    Write-Host "⚠️  Current directory is not empty." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  1) Continue and merge files"
    Write-Host "  2) Organize existing files first"
    Write-Host "  3) Cancel"
    Write-Host ""
    $choice = Read-Host "Select (1/2/3)"

    switch ($choice) {
        "1" { Write-Host "Continuing..." }
        "2" { Invoke-OrganizeFiles }
        "3" { Write-Host "Cancelled."; exit 0 }
        default { Write-Host "Invalid option."; exit 1 }
    }
}

# Backup user files before upgrade
if ($UpgradeMode) {
    Backup-UserFiles
}

# Download template
Write-Host "`n📥 Downloading Git-Core Protocol..." -ForegroundColor Cyan

try {
    git clone --depth 1 $REPO_URL $TEMP_DIR 2>$null
} catch {
    Write-Host "❌ Error cloning repository" -ForegroundColor Red
    exit 1
}

Remove-Item -Recurse -Force "$TEMP_DIR/.git" -ErrorAction SilentlyContinue

# Install files
Write-Host "📦 Installing protocol files..." -ForegroundColor Cyan

# Handle .ai directory specially
if (Test-Path "$TEMP_DIR/.ai") {
    if ($UpgradeMode) {
        if (Test-Path ".ai") {
            Remove-Item -Recurse -Force ".ai"
        }
        Copy-Item -Recurse "$TEMP_DIR/.ai" .
        Write-Host "  ✓ .✨/ (upgraded)" -ForegroundColor Green
    } elseif (-not (Test-Path ".ai")) {
        Copy-Item -Recurse "$TEMP_DIR/.ai" .
        Write-Host "  ✓ .✨/" -ForegroundColor Green
    } else {
        Write-Host "  ~ .✨/ (exists, merging new files)" -ForegroundColor Yellow
        Get-ChildItem "$TEMP_DIR/.ai" | ForEach-Object {
            if (-not (Test-Path ".✨/$($_.Name)")) {
                Copy-Item $_.FullName ".✨/"
                Write-Host "    + $($_.Name)" -ForegroundColor Green
            }
        }
    }
}

# Copy other directories
$dirs = @(".github", "scripts", "docs")
foreach ($dir in $dirs) {
    if (Test-Path "$TEMP_DIR/$dir") {
        if ($UpgradeMode) {
            if (Test-Path $dir) {
                Remove-Item -Recurse -Force $dir
            }
            Copy-Item -Recurse "$TEMP_DIR/$dir" .
            Write-Host "  ✓ $dir/ (upgraded)" -ForegroundColor Green
        } elseif (-not (Test-Path $dir)) {
            Copy-Item -Recurse "$TEMP_DIR/$dir" .
            Write-Host "  ✓ $dir/" -ForegroundColor Green
        } else {
            Copy-Item -Recurse -Force "$TEMP_DIR/$dir/*" $dir -ErrorAction SilentlyContinue
            Write-Host "  ✓ $dir/ (merged)" -ForegroundColor Green
        }
    }
}

# Protocol files
$protocolFiles = @(".cursorrules", ".windsurfrules", "AGENTS.md", ".git-core-protocol-version")
foreach ($file in $protocolFiles) {
    if (Test-Path "$TEMP_DIR/$file") {
        if ($UpgradeMode) {
            Copy-Item -Force "$TEMP_DIR/$file" .
            Write-Host "  ✓ $file (upgraded)" -ForegroundColor Green
        } elseif (-not (Test-Path $file)) {
            Copy-Item "$TEMP_DIR/$file" .
            Write-Host "  ✓ $file" -ForegroundColor Green
        } else {
            Write-Host "  ~ $file (exists)" -ForegroundColor Yellow
        }
    }
}

# Files that should never be overwritten
$preserveFiles = @(".gitignore", "README.md")
foreach ($file in $preserveFiles) {
    if ((Test-Path "$TEMP_DIR/$file") -and -not (Test-Path $file)) {
        Copy-Item "$TEMP_DIR/$file" .
        Write-Host "  ✓ $file" -ForegroundColor Green
    } elseif (Test-Path $file) {
        Write-Host "  ~ $file (preserved)" -ForegroundColor Yellow
    }
}

# Cleanup
Remove-Item -Recurse -Force $TEMP_DIR -ErrorAction SilentlyContinue

# Restore user files after upgrade
if ($UpgradeMode) {
    Restore-UserFiles
}

# Show final version
$NewVersion = Get-CurrentVersion

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "✅ Git-Core Protocol v$NewVersion installed" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

if ($UpgradeMode) {
    Write-Host "📋 Upgraded from v$CurrentVersion → v$NewVersion" -ForegroundColor Cyan
    if (-not $ForceMode) {
        Write-Host "✓ Your ARCHITECTURE.md was preserved" -ForegroundColor Green
    }
} else {
    Write-Host "📋 Files installed:"
    Write-Host "   .✨/ARCHITECTURE.md    - Document your architecture here"
    Write-Host "   .github/               - Copilot rules + workflows"
    Write-Host "   scripts/               - Init and update scripts"
    Write-Host "   AGENTS.md              - Rules for all AI agents"
}

Write-Host ""
Write-Host "🚀 Next step:" -ForegroundColor Yellow
Write-Host "   .\scripts\init_project.ps1"
Write-Host ""
Write-Host "💡 Commands:" -ForegroundColor Cyan
Write-Host '   Safe upgrade:  $env:GIT_CORE_UPGRADE = "1"; irm .../install.ps1 | iex'
Write-Host '   Full reset:    $env:GIT_CORE_FORCE = "1"; irm .../install.ps1 | iex'
Write-Host "   Check updates: .\scripts\check-protocol-update.ps1"

