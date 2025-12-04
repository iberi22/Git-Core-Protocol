#!/bin/bash
# install.sh - Remote installer for Git-Core Protocol
# Usage: curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash
#
# 🎯 This script can be executed by AI agents to bootstrap any project with Git-Core Protocol
# Options:
#   --organize, -o    Organize existing files before installing
#   --auto, -y        Non-interactive mode (auto-accept)
#   --upgrade, -u     Upgrade existing installation (PRESERVES ARCHITECTURE.md)
#   --force, -f       Force upgrade (overwrites EVERYTHING including ARCHITECTURE.md)

set -e

REPO_URL="https://github.com/iberi22/Git-Core-Protocol"
RAW_URL="https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main"
TEMP_DIR=".git-core-temp"
BACKUP_DIR=".git-core-backup"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${CYAN}🧠 Git-Core Protocol - Remote Installer v1.4.0${NC}"
echo "=============================================="
echo ""

# Parse arguments
ORGANIZE_FILES=false
AUTO_MODE=false
UPGRADE_MODE=false
FORCE_MODE=false
for arg in "$@"; do
    case $arg in
        --organize|-o)
            ORGANIZE_FILES=true
            ;;
        --auto|-y)
            AUTO_MODE=true
            ;;
        --upgrade|-u)
            UPGRADE_MODE=true
            AUTO_MODE=true
            ;;
        --force|-f)
            FORCE_MODE=true
            UPGRADE_MODE=true
            AUTO_MODE=true
            ;;
        --help|-h)
            echo "Usage: install.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --organize, -o    Organize existing files before installing"
            echo "  --auto, -y        Non-interactive mode"
            echo "  --upgrade, -u     Upgrade protocol files (PRESERVES your ARCHITECTURE.md)"
            echo "  --force, -f       Force full upgrade (overwrites everything)"
            echo "  --help, -h        Show this help"
            echo ""
            echo "Examples:"
            echo "  curl -fsSL .../install.sh | bash                    # New install"
            echo "  curl -fsSL .../install.sh | bash -s -- --upgrade    # Safe upgrade"
            echo "  curl -fsSL .../install.sh | bash -s -- --force      # Full reset"
            exit 0
            ;;
    esac
done

# Show mode
if [ "$FORCE_MODE" = true ]; then
    echo -e "${RED}⚠️  FORCE MODE: ALL files will be overwritten (including ARCHITECTURE.md)${NC}"
elif [ "$UPGRADE_MODE" = true ]; then
    echo -e "${YELLOW}🔄 UPGRADE MODE: Protocol files updated, your ARCHITECTURE.md preserved${NC}"
fi
echo ""

# Function to get current version
get_current_version() {
    if [ -f ".git-core-protocol-version" ]; then
        cat ".git-core-protocol-version" | tr -d '[:space:]'
    else
        echo "0.0.0"
    fi
}

# Function to get remote version
get_remote_version() {
    curl -fsSL "$RAW_URL/.git-core-protocol-version" 2>/dev/null | tr -d '[:space:]' || echo "unknown"
}

# Show version info
CURRENT_VERSION=$(get_current_version)
if [ "$CURRENT_VERSION" != "0.0.0" ]; then
    REMOTE_VERSION=$(get_remote_version)
    echo -e "${BLUE}📊 Version Info:${NC}"
    echo -e "   Current: ${YELLOW}$CURRENT_VERSION${NC}"
    echo -e "   Latest:  ${GREEN}$REMOTE_VERSION${NC}"
    echo ""
fi

# Function to migrate from .ai/ to .✨/
migrate_ai_directory() {
    if [ -d ".ai" ]; then
        echo -e "${YELLOW}🔄 Detected legacy .ai/ directory...${NC}"

        mkdir -p ".✨"

        # Copy all files from .ai/ to .✨/
        cp -r .ai/* .✨/ 2>/dev/null || true

        echo -e "  ${GREEN}✓ Migrated .ai/ → .✨/${NC}"
        echo -e "  ${CYAN}ℹ️  You can safely delete .ai/ after verifying${NC}"
        return 0
    fi
    return 1
}

# Function to backup user files
backup_user_files() {
    echo -e "${CYAN}💾 Backing up user files...${NC}"
    mkdir -p "$BACKUP_DIR"

    # Check both .✨/ and .ai/ for backwards compatibility
    AI_DIR=""
    if [ -d ".✨" ]; then
        AI_DIR=".✨"
    elif [ -d ".ai" ]; then
        AI_DIR=".ai"
    fi

    # Backup ARCHITECTURE.md if it exists
    if [ -n "$AI_DIR" ] && [ -f "$AI_DIR/ARCHITECTURE.md" ]; then
        cp "$AI_DIR/ARCHITECTURE.md" "$BACKUP_DIR/ARCHITECTURE.md"
        echo -e "  ${GREEN}✓ $AI_DIR/ARCHITECTURE.md backed up${NC}"
    fi

    # Backup CONTEXT_LOG.md if it exists
    if [ -n "$AI_DIR" ] && [ -f "$AI_DIR/CONTEXT_LOG.md" ]; then
        cp "$AI_DIR/CONTEXT_LOG.md" "$BACKUP_DIR/CONTEXT_LOG.md"
        echo -e "  ${GREEN}✓ $AI_DIR/CONTEXT_LOG.md backed up${NC}"
    fi

    # Backup custom workflows
    if [ -d ".github/workflows" ]; then
        mkdir -p "$BACKUP_DIR/workflows"
        for file in .github/workflows/*.yml; do
            if [ -f "$file" ]; then
                filename=$(basename "$file")
                # Only backup non-protocol workflows
                case "$filename" in
                    update-protocol.yml|structure-validator.yml|codex-review.yml|agent-dispatcher.yml)
                        # Protocol workflows - don't backup
                        ;;
                    *)
                        cp "$file" "$BACKUP_DIR/workflows/"
                        echo -e "  ${GREEN}✓ Custom workflow: $filename${NC}"
                        ;;
                esac
            fi
        done
    fi
}

# Function to restore user files
restore_user_files() {
    echo -e "${CYAN}📥 Restoring user files...${NC}"

    # Ensure .✨ directory exists for restoration
    mkdir -p ".✨"

    # Restore ARCHITECTURE.md (unless force mode)
    if [ "$FORCE_MODE" != true ] && [ -f "$BACKUP_DIR/ARCHITECTURE.md" ]; then
        cp "$BACKUP_DIR/ARCHITECTURE.md" ".✨/ARCHITECTURE.md"
        echo -e "  ${GREEN}✓ .✨/ARCHITECTURE.md restored${NC}"
    fi

    # Always restore CONTEXT_LOG.md
    if [ -f "$BACKUP_DIR/CONTEXT_LOG.md" ]; then
        cp "$BACKUP_DIR/CONTEXT_LOG.md" ".✨/CONTEXT_LOG.md"
        echo -e "  ${GREEN}✓ .✨/CONTEXT_LOG.md restored${NC}"
    fi

    # Restore custom workflows
    if [ -d "$BACKUP_DIR/workflows" ]; then
        for file in "$BACKUP_DIR/workflows"/*.yml; do
            if [ -f "$file" ]; then
                cp "$file" ".github/workflows/"
                echo -e "  ${GREEN}✓ Custom workflow restored: $(basename $file)${NC}"
            fi
        done
    fi

    # Cleanup backup
    rm -rf "$BACKUP_DIR"
}

# Function to organize existing files
organize_existing_files() {
    echo -e "${YELLOW}📂 Organizing existing files...${NC}"

    mkdir -p docs/archive scripts tests src

    for file in *.md; do
        if [ -f "$file" ]; then
            case "$file" in
                README.md|AGENTS.md|CHANGELOG.md|CONTRIBUTING.md|LICENSE.md)
                    echo -e "  ${GREEN}✓ Keeping $file in root${NC}"
                    ;;
                *)
                    mv "$file" "docs/archive/" 2>/dev/null && \
                    echo -e "  ${CYAN}→ $file moved to docs/archive/${NC}" || true
                    ;;
            esac
        fi
    done

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

# Check if should organize
if [ "$ORGANIZE_FILES" = true ]; then
    organize_existing_files
fi

# Check if current directory has files
if [ "$(ls -A 2>/dev/null | grep -v '^\.' | head -1)" ] && [ "$AUTO_MODE" = false ]; then
    echo -e "${YELLOW}⚠️  Current directory is not empty.${NC}"
    echo ""
    echo "Options:"
    echo "  1) Continue and merge files"
    echo "  2) Organize existing files first"
    echo "  3) Cancel"
    echo ""
    read -p "Select (1/2/3): " CHOICE

    case $CHOICE in
        1) echo "Continuing..." ;;
        2) organize_existing_files ;;
        3) echo "Cancelled."; exit 0 ;;
        *) echo "Invalid option."; exit 1 ;;
    esac
fi

# Backup user files before upgrade
if [ "$UPGRADE_MODE" = true ]; then
    backup_user_files
fi

# Download template
echo -e "\n${CYAN}📥 Downloading Git-Core Protocol...${NC}"
git clone --depth 1 "$REPO_URL" "$TEMP_DIR" 2>/dev/null || {
    echo -e "${RED}❌ Error cloning repository${NC}"
    exit 1
}

rm -rf "$TEMP_DIR/.git"

# Install files
echo -e "${CYAN}📦 Installing protocol files...${NC}"

# Run migration from .ai/ to .✨/ if needed
migrate_ai_directory

# Handle .✨ directory (protocol uses .✨, template may have .ai)
TEMPLATE_AI_DIR=""
if [ -d "$TEMP_DIR/.✨" ]; then
    TEMPLATE_AI_DIR="$TEMP_DIR/.✨"
elif [ -d "$TEMP_DIR/.ai" ]; then
    TEMPLATE_AI_DIR="$TEMP_DIR/.ai"
fi

if [ -n "$TEMPLATE_AI_DIR" ]; then
    if [ "$UPGRADE_MODE" = true ]; then
        # Remove old directories
        rm -rf .✨ .ai 2>/dev/null || true

        # Copy to .✨
        mkdir -p ".✨"
        cp -r "$TEMPLATE_AI_DIR"/* .✨/
        echo -e "  ${GREEN}✓ .✨/ (upgraded)${NC}"
    elif [ ! -d ".✨" ] && [ ! -d ".ai" ]; then
        mkdir -p ".✨"
        cp -r "$TEMPLATE_AI_DIR"/* .✨/
        echo -e "  ${GREEN}✓ .✨/${NC}"
    else
        # Ensure .✨ exists
        mkdir -p ".✨"
        echo -e "  ${YELLOW}~ .✨/ (exists, merging new files only)${NC}"
        for file in "$TEMPLATE_AI_DIR"/*; do
            filename=$(basename "$file")
            if [ ! -f ".✨/$filename" ]; then
                cp "$file" ".✨/"
                echo -e "    ${GREEN}+ $filename${NC}"
            fi
        done
    fi
fi

# Copy other directories
for dir in .github scripts docs bin; do
    if [ -d "$TEMP_DIR/$dir" ]; then
        if [ "$UPGRADE_MODE" = true ]; then
            rm -rf "$dir"
            cp -r "$TEMP_DIR/$dir" .
            
            # Cleanup internal files
            if [ "$dir" = ".github" ]; then
                rm -f ".github/workflows/build-tools.yml"
                rm -f ".github/workflows/release.yml"
            fi
            if [ "$dir" = "scripts" ]; then
                rm -f "scripts/bump-version.ps1"
                rm -f "scripts/bump-version.sh"
            fi

            echo -e "  ${GREEN}✓ $dir/ (upgraded)${NC}"
        elif [ ! -d "$dir" ]; then
            cp -r "$TEMP_DIR/$dir" .

            # Cleanup internal files
            if [ "$dir" = ".github" ]; then
                rm -f ".github/workflows/build-tools.yml"
                rm -f ".github/workflows/release.yml"
            fi
            if [ "$dir" = "scripts" ]; then
                rm -f "scripts/bump-version.ps1"
                rm -f "scripts/bump-version.sh"
            fi

            echo -e "  ${GREEN}✓ $dir/${NC}"
        else
            cp -rn "$TEMP_DIR/$dir"/* "$dir/" 2>/dev/null || true
            
            # Cleanup internal files
            if [ "$dir" = ".github" ]; then
                rm -f ".github/workflows/build-tools.yml"
                rm -f ".github/workflows/release.yml"
            fi
            if [ "$dir" = "scripts" ]; then
                rm -f "scripts/bump-version.ps1"
                rm -f "scripts/bump-version.sh"
            fi

            echo -e "  ${GREEN}✓ $dir/ (merged)${NC}"
        fi
    fi
done

# Protocol files
PROTOCOL_FILES=".cursorrules .windsurfrules AGENTS.md .git-core-protocol-version"
for file in $PROTOCOL_FILES; do
    if [ -f "$TEMP_DIR/$file" ]; then
        if [ "$UPGRADE_MODE" = true ]; then
            cp "$TEMP_DIR/$file" .
            echo -e "  ${GREEN}✓ $file (upgraded)${NC}"
        elif [ ! -f "$file" ]; then
            cp "$TEMP_DIR/$file" .
            echo -e "  ${GREEN}✓ $file${NC}"
        else
            echo -e "  ${YELLOW}~ $file (exists)${NC}"
        fi
    fi
done

# Files that should never be overwritten
PRESERVE_FILES=".gitignore README.md"
for file in $PRESERVE_FILES; do
    if [ -f "$TEMP_DIR/$file" ] && [ ! -f "$file" ]; then
        cp "$TEMP_DIR/$file" .
        echo -e "  ${GREEN}✓ $file${NC}"
    elif [ -f "$file" ]; then
        echo -e "  ${YELLOW}~ $file (preserved)${NC}"
    fi
done

# Cleanup temp
rm -rf "$TEMP_DIR"

# Restore user files after upgrade
if [ "$UPGRADE_MODE" = true ]; then
    restore_user_files
fi

# Make scripts executable
chmod +x scripts/*.sh 2>/dev/null || true

# Show final version
NEW_VERSION=$(get_current_version)

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Git-Core Protocol v$NEW_VERSION installed${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

if [ "$UPGRADE_MODE" = true ]; then
    echo -e "${CYAN}📋 Upgraded from v$CURRENT_VERSION → v$NEW_VERSION${NC}"
    if [ "$FORCE_MODE" != true ]; then
        echo -e "${GREEN}✓ Your ARCHITECTURE.md was preserved${NC}"
    fi
else
    echo -e "📋 Files installed:"
    echo "   .✨/ARCHITECTURE.md    - Document your architecture here"
    echo "   .github/               - Copilot rules + workflows"
    echo "   scripts/               - Init and update scripts"
    echo "   AGENTS.md              - Rules for all AI agents"
fi

echo ""
echo -e "${YELLOW}🚀 Next step:${NC}"
echo "   ./scripts/init_project.sh"
echo ""
echo -e "${CYAN}💡 Commands:${NC}"
echo "   Safe upgrade:  curl -fsSL .../install.sh | bash -s -- --upgrade"
echo "   Full reset:    curl -fsSL .../install.sh | bash -s -- --force"
echo "   Check updates: ./scripts/check-protocol-update.sh"
