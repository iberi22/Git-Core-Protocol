#!/bin/bash
# scripts/init_project.sh
# 🧠 Git-Core Protocol - Project Initializer
#
# Options:
#   --auto, -a       Non-interactive mode (auto-accept defaults)
#   --organize, -o   Organize existing files before setup
#   --private, -p    Create private repository (default: public)
#
# Usage:
#   ./init_project.sh
#   ./init_project.sh --auto --organize
#   ./init_project.sh -a -o -p

set -e

# Parse arguments
AUTO_MODE=false
ORGANIZE_FILES=false
PRIVATE_REPO=false

for arg in "$@"; do
    case $arg in
        --auto|-a)
            AUTO_MODE=true
            ;;
        --organize|-o)
            ORGANIZE_FILES=true
            ;;
        --private|-p)
            PRIVATE_REPO=true
            ;;
    esac
done

echo "🧠 Initializing Git-Core Protocol..."
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to organize existing files
organize_files() {
    echo -e "\n${CYAN}📂 Organizing existing files...${NC}"
    
    # Create directories
    mkdir -p docs/archive scripts tests src
    
    # Files to keep in root
    local keep_in_root="README.md AGENTS.md CHANGELOG.md CONTRIBUTING.md LICENSE.md LICENSE"
    
    # Move markdown files to docs/archive
    for file in *.md; do
        if [ -f "$file" ]; then
            if echo "$keep_in_root" | grep -qw "$file"; then
                echo -e "  ${GREEN}✓ Keeping $file in root${NC}"
            else
                mv "$file" "docs/archive/" 2>/dev/null && \
                echo -e "  ${CYAN}→ $file moved to docs/archive/${NC}" || true
            fi
        fi
    done
    
    # Move test files
    for pattern in test_*.py *_test.py *.test.js *.test.ts *.spec.js *.spec.ts; do
        for file in $pattern; do
            if [ -f "$file" ] && [ "$file" != "$pattern" ]; then
                mv "$file" "tests/" 2>/dev/null && \
                echo -e "  ${CYAN}→ $file moved to tests/${NC}" || true
            fi
        done
    done
    
    echo -e "${GREEN}✅ Files organized${NC}"
}

# Run organize if requested
if [ "$ORGANIZE_FILES" = true ]; then
    organize_files
fi

# 1. Validate environment
echo -e "\n📋 Validating environment..."

if ! command -v git &> /dev/null; then
    echo -e "${RED}❌ Error: Git is not installed.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Git installed${NC}"

if ! command -v gh &> /dev/null; then
    echo -e "${RED}❌ Error: GitHub CLI (gh) is not installed.${NC}"
    echo "  Install from: https://cli.github.com/"
    exit 1
fi
echo -e "${GREEN}✓ GitHub CLI installed${NC}"

# Check if gh is authenticated
if ! gh auth status &> /dev/null; then
    echo -e "${RED}❌ Error: Not authenticated with GitHub CLI.${NC}"
    echo "  Run: gh auth login"
    exit 1
fi
echo -e "${GREEN}✓ GitHub CLI authenticated${NC}"

# 2. Get project name
PROJECT_NAME=$(basename "$PWD")
echo -e "\n📁 Project: ${YELLOW}${PROJECT_NAME}${NC}"

# 3. Check if this is an existing Git repository
EXISTING_REPO=false
if [ -d ".git" ]; then
    EXISTING_REPO=true
    echo -e "${CYAN}ℹ️  Existing Git repository detected${NC}"
    
    # Check if remote already exists
    if git remote get-url origin &> /dev/null; then
        echo -e "${GREEN}✓ Remote 'origin' already configured${NC}"
        REMOTE_URL=$(git remote get-url origin)
        echo -e "  ${CYAN}$REMOTE_URL${NC}"
        SKIP_REPO_CREATE=true
    else
        SKIP_REPO_CREATE=false
    fi
else
    echo -e "\n🔧 Initializing Git repository..."
    git init
    git add .
    git commit -m "feat: 🚀 Initial commit with Git-Core Protocol"
    SKIP_REPO_CREATE=false
fi

# 4. Create GitHub repository (if needed)
if [ "$SKIP_REPO_CREATE" != true ]; then
    echo -e "\n☁️  Creating GitHub repository..."
    
    if [ "$AUTO_MODE" = true ]; then
        if [ "$PRIVATE_REPO" = true ]; then
            VISIBILITY="--private"
            echo -e "  ${CYAN}(Auto mode: creating private repository)${NC}"
        else
            VISIBILITY="--public"
            echo -e "  ${CYAN}(Auto mode: creating public repository)${NC}"
        fi
    else
        read -p "Private repository? (y/N): " PRIVATE_CHOICE
        if [[ $PRIVATE_CHOICE =~ ^[Yy]$ ]]; then
            VISIBILITY="--private"
        else
            VISIBILITY="--public"
        fi
    fi
    
    gh repo create "$PROJECT_NAME" $VISIBILITY --source=. --remote=origin --push
else
    echo -e "\n${CYAN}ℹ️  Skipping repository creation (already exists)${NC}"
    # Make sure we have latest changes committed
    if [ -n "$(git status --porcelain)" ]; then
        echo -e "${YELLOW}⚠️  Uncommitted changes detected, committing...${NC}"
        git add .
        git commit -m "chore: 🧠 Add Git-Core Protocol configuration"
        git push origin HEAD
    fi
fi

# 5. Setup Architecture file if empty
if [ ! -s .ai/ARCHITECTURE.md ] || [ ! -f .ai/ARCHITECTURE.md ]; then
    echo -e "\n📐 Setting up ARCHITECTURE.md..."
    mkdir -p .ai
    cat > .ai/ARCHITECTURE.md << 'EOF'
# 🏗️ Architecture

## Stack
- **Language:** TBD
- **Framework:** TBD
- **Database:** TBD

## Key Decisions
_Document architectural decisions here_

## Project Structure
```
TBD
```
EOF
fi

# 6. Create Semantic Labels for AI
echo -e "\n🏷️  Creating semantic labels..."

# Function to create label if it doesn't exist
create_label() {
    local name=$1
    local description=$2
    local color=$3
    
    if ! gh label list | grep -q "$name"; then
        gh label create "$name" --description "$description" --color "$color" 2>/dev/null || true
        echo -e "  ${GREEN}✓ $name${NC}"
    else
        echo -e "  ${YELLOW}~ $name (already exists)${NC}"
    fi
}

create_label "ai-plan" "High-level planning tasks" "0E8A16"
create_label "ai-context" "Critical context information" "FBCA04"
create_label "ai-blocked" "Blocked - requires human intervention" "D93F0B"
create_label "in-progress" "Task in progress" "1D76DB"
create_label "needs-review" "Requires review" "5319E7"

# 7. Create Initial Issues
echo -e "\n📝 Creating initial issues..."

gh issue create \
    --title "🏗️ SETUP: Define Architecture and Tech Stack" \
    --body "## Objective
Define and document the architectural decisions for the project.

## Tasks
- [ ] Define main language/framework
- [ ] Define database (if applicable)
- [ ] Define folder structure
- [ ] Document in \`.ai/ARCHITECTURE.md\`

## Notes for AI Agent
Read project requirements and propose an appropriate stack." \
    --label "ai-plan"

gh issue create \
    --title "⚙️ INFRA: Initial development environment setup" \
    --body "## Objective
Set up development tools.

## Tasks
- [ ] Configure linter
- [ ] Configure formatter
- [ ] Configure pre-commit hooks (optional)
- [ ] Create base folder structure
- [ ] Add initial dependencies

## Notes for AI Agent
Use best practices for the chosen stack." \
    --label "ai-plan"

gh issue create \
    --title "📚 DOCS: Initial project documentation" \
    --body "## Objective
Create basic documentation.

## Tasks
- [ ] Update README.md with project description
- [ ] Document how to run the project
- [ ] Document how to contribute

## Notes for AI Agent
Keep documentation concise and practical." \
    --label "ai-plan"

# 8. Final message
echo -e "\n=========================================="
echo -e "${GREEN}✅ Project initialized successfully!${NC}"
echo -e "=========================================="
echo ""
echo "📍 Repository: https://github.com/$(gh api user --jq .login)/$PROJECT_NAME"
echo ""
echo "🚀 Next steps:"
echo "   1. Open the project in your AI editor (Cursor/Windsurf/VS Code)"
echo "   2. Type: 'Start with the first assigned issue'"
echo "   3. The agent will read the rules and begin working"
echo ""
echo "📋 Issues created:"
gh issue list --limit 5
