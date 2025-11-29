# scripts/init_project.ps1
# 🧠 Git-Core Protocol - Project Initializer (PowerShell)
# 
# Options:
#   -Organize    Organize existing files before setup
#   -Auto        Non-interactive mode (auto-accept defaults)
#   -Private     Create private repository (default: public)
#
# Usage:
#   .\init_project.ps1
#   .\init_project.ps1 -Organize
#   .\init_project.ps1 -Auto -Organize -Private

param(
    [switch]$Organize,
    [switch]$Auto,
    [switch]$Private
)

$ErrorActionPreference = "Stop"

# Function to organize existing files
function Invoke-OrganizeFiles {
    Write-Host "`n📂 Organizing existing files..." -ForegroundColor Yellow
    
    # Create directories
    $dirs = @("docs/archive", "scripts", "tests", "src")
    foreach ($dir in $dirs) {
        New-Item -ItemType Directory -Force -Path $dir -ErrorAction SilentlyContinue | Out-Null
    }
    
    # Files to keep in root
    $keepInRoot = @("README.md", "AGENTS.md", "CHANGELOG.md", "CONTRIBUTING.md", "LICENSE.md", "LICENSE")
    
    # Move markdown files to docs/archive
    Get-ChildItem -Filter "*.md" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.Name -notin $keepInRoot) {
            Move-Item $_.FullName -Destination "docs/archive/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) moved to docs/archive/" -ForegroundColor Cyan
        } else {
            Write-Host "  ✓ Keeping $($_.Name) in root" -ForegroundColor Green
        }
    }
    
    # Move test files
    $testPatterns = @("test_*.py", "*_test.py", "*.test.js", "*.test.ts", "*.spec.js", "*.spec.ts")
    foreach ($pattern in $testPatterns) {
        Get-ChildItem -Filter $pattern -File -ErrorAction SilentlyContinue | ForEach-Object {
            Move-Item $_.FullName -Destination "tests/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) moved to tests/" -ForegroundColor Cyan
        }
    }
    
    # Move loose scripts (except init scripts)
    $scriptKeep = @("install.sh")
    Get-ChildItem -Filter "*.sh" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.Name -notin $scriptKeep -and $_.DirectoryName -eq (Get-Location).Path) {
            Move-Item $_.FullName -Destination "scripts/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) moved to scripts/" -ForegroundColor Cyan
        }
    }
    Get-ChildItem -Filter "*.bat" -File -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_.DirectoryName -eq (Get-Location).Path) {
            Move-Item $_.FullName -Destination "scripts/" -Force -ErrorAction SilentlyContinue
            Write-Host "  → $($_.Name) moved to scripts/" -ForegroundColor Cyan
        }
    }
    
    Write-Host "✅ Files organized" -ForegroundColor Green
}

Write-Host "🧠 Initializing Git-Core Protocol..." -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Run organize if requested
if ($Organize) {
    Invoke-OrganizeFiles
}

# 1. Validate environment
Write-Host "`n📋 Validating environment..." -ForegroundColor Yellow

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: Git is not installed." -ForegroundColor Red
    exit 1
}
Write-Host "✓ Git installed" -ForegroundColor Green

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: GitHub CLI (gh) is not installed." -ForegroundColor Red
    Write-Host "  Install from: https://cli.github.com/" -ForegroundColor Yellow
    exit 1
}
Write-Host "✓ GitHub CLI installed" -ForegroundColor Green

# Check if gh is authenticated
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error: Not authenticated with GitHub CLI." -ForegroundColor Red
    Write-Host "  Run: gh auth login" -ForegroundColor Yellow
    exit 1
}
Write-Host "✓ GitHub CLI authenticated" -ForegroundColor Green

# 2. Get project name
$PROJECT_NAME = Split-Path -Leaf (Get-Location)
Write-Host "`n📁 Project: $PROJECT_NAME" -ForegroundColor Yellow

# 3. Check if this is an existing Git repository
$EXISTING_REPO = $false
$SKIP_REPO_CREATE = $false

if (Test-Path ".git") {
    $EXISTING_REPO = $true
    Write-Host "ℹ️  Existing Git repository detected" -ForegroundColor Cyan
    
    # Check if remote already exists
    $remoteUrl = git remote get-url origin 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Remote 'origin' already configured" -ForegroundColor Green
        Write-Host "  $remoteUrl" -ForegroundColor Cyan
        $SKIP_REPO_CREATE = $true
    }
} else {
    Write-Host "`n🔧 Initializing Git repository..." -ForegroundColor Yellow
    git init
    git add .
    git commit -m "feat: 🚀 Initial commit with Git-Core Protocol"
}

# 4. Create GitHub repository (if needed)
if (-not $SKIP_REPO_CREATE) {
    Write-Host "`n☁️  Creating GitHub repository..." -ForegroundColor Yellow

    if ($Auto) {
        if ($Private) {
            $VISIBILITY = "--private"
            Write-Host "  (Auto mode: creating private repository)" -ForegroundColor Cyan
        } else {
            $VISIBILITY = "--public"
            Write-Host "  (Auto mode: creating public repository)" -ForegroundColor Cyan
        }
    } else {
        $PRIVATE_CHOICE = Read-Host "Private repository? (y/N)"
        if ($PRIVATE_CHOICE -match "^[Yy]$") {
            $VISIBILITY = "--private"
        } else {
            $VISIBILITY = "--public"
        }
    }

    Invoke-Expression "gh repo create $PROJECT_NAME $VISIBILITY --source=. --remote=origin --push"
} else {
    Write-Host "`nℹ️  Skipping repository creation (already exists)" -ForegroundColor Cyan
    
    # Check for uncommitted changes
    $status = git status --porcelain
    if ($status) {
        Write-Host "⚠️  Uncommitted changes detected, committing..." -ForegroundColor Yellow
        git add .
        git commit -m "chore: 🧠 Add Git-Core Protocol configuration"
        git push origin HEAD
    }
}

# 5. Setup Architecture file if empty
$archFile = ".ai/ARCHITECTURE.md"
if (-not (Test-Path $archFile) -or (Get-Item $archFile).Length -eq 0) {
    Write-Host "`n📐 Setting up ARCHITECTURE.md..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Force -Path ".ai" | Out-Null
    @"
# 🏗️ Architecture

## Stack
- **Language:** TBD
- **Framework:** TBD
- **Database:** TBD

## Key Decisions
_Document architectural decisions here_

## Project Structure
``````
TBD
``````
"@ | Set-Content $archFile -Encoding UTF8
}

# 6. Create Semantic Labels for AI
Write-Host "`n🏷️  Creating semantic labels..." -ForegroundColor Yellow

function Create-Label {
    param($name, $description, $color)
    
    $existingLabels = gh label list --json name | ConvertFrom-Json
    if ($existingLabels.name -notcontains $name) {
        gh label create $name --description $description --color $color 2>$null
        Write-Host "  ✓ $name" -ForegroundColor Green
    } else {
        Write-Host "  ~ $name (already exists)" -ForegroundColor Yellow
    }
}

Create-Label "ai-plan" "High-level planning tasks" "0E8A16"
Create-Label "ai-context" "Critical context information" "FBCA04"
Create-Label "ai-blocked" "Blocked - requires human intervention" "D93F0B"
Create-Label "in-progress" "Task in progress" "1D76DB"
Create-Label "needs-review" "Requires review" "5319E7"

# 7. Create Initial Issues
Write-Host "`n📝 Creating initial issues..." -ForegroundColor Yellow

gh issue create `
    --title "🏗️ SETUP: Define Architecture and Tech Stack" `
    --body @"
## Objective
Define and document the architectural decisions for the project.

## Tasks
- [ ] Define main language/framework
- [ ] Define database (if applicable)
- [ ] Define folder structure
- [ ] Document in ``.ai/ARCHITECTURE.md``

## Notes for AI Agent
Read project requirements and propose an appropriate stack.
"@ `
    --label "ai-plan"

gh issue create `
    --title "⚙️ INFRA: Initial development environment setup" `
    --body @"
## Objective
Set up development tools.

## Tasks
- [ ] Configure linter
- [ ] Configure formatter
- [ ] Configure pre-commit hooks (optional)
- [ ] Create base folder structure
- [ ] Add initial dependencies

## Notes for AI Agent
Use best practices for the chosen stack.
"@ `
    --label "ai-plan"

gh issue create `
    --title "📚 DOCS: Initial project documentation" `
    --body @"
## Objective
Create basic documentation.

## Tasks
- [ ] Update README.md with project description
- [ ] Document how to run the project
- [ ] Document how to contribute

## Notes for AI Agent
Keep documentation concise and practical.
"@ `
    --label "ai-plan"

# 8. Final message
Write-Host "`n==========================================" -ForegroundColor Cyan
Write-Host "✅ Project initialized successfully!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""
$username = (gh api user --jq .login)
Write-Host "📍 Repository: https://github.com/$username/$PROJECT_NAME" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Next steps:" -ForegroundColor Yellow
Write-Host "   1. Open the project in your AI editor (Cursor/Windsurf/VS Code)"
Write-Host "   2. Type: 'Start with the first assigned issue'"
Write-Host "   3. The agent will read the rules and begin working"
Write-Host ""
Write-Host "📋 Issues created:" -ForegroundColor Yellow
gh issue list --limit 5
