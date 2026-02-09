#!/bin/bash

# Script to rebase and merge all open Exercism solution PRs
# This script automates the process of merging multiple PRs from Exercism solutions syncer

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Repository details
OWNER="Se7enseads"
REPO="Exercism"

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}Error: GitHub CLI (gh) is not installed.${NC}"
    echo "Please install it from: https://cli.github.com/"
    exit 1
fi

# Check if user is authenticated with gh
if ! gh auth status &> /dev/null; then
    echo -e "${RED}Error: Not authenticated with GitHub CLI.${NC}"
    echo "Please run: gh auth login"
    exit 1
fi

echo -e "${GREEN}=== Exercism PR Merge Script ===${NC}"
echo ""

# Get all open PRs except the current one (PR #43)
echo -e "${YELLOW}Fetching open pull requests...${NC}"
PRS=$(gh pr list --repo "$OWNER/$REPO" --state open --json number,title --limit 100 | jq -r '.[] | select(.number != 43) | "\(.number):\(.title)"')

if [ -z "$PRS" ]; then
    echo -e "${YELLOW}No pull requests to merge.${NC}"
    exit 0
fi

# Count PRs
PR_COUNT=$(echo "$PRS" | wc -l)
echo -e "${GREEN}Found $PR_COUNT pull requests to merge${NC}"
echo ""

# Display PRs that will be merged
echo "Pull requests to be merged:"
echo "$PRS" | nl
echo ""

# Ask for confirmation
read -p "Do you want to proceed with rebasing and merging these PRs? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Aborted by user.${NC}"
    exit 0
fi

# Process each PR
MERGED=0
FAILED=0
SKIPPED=0

echo ""
echo -e "${GREEN}Starting merge process...${NC}"
echo ""

# Get PR numbers
PR_NUMBERS=$(echo "$PRS" | cut -d: -f1)

for PR_NUM in $PR_NUMBERS; do
    echo -e "${YELLOW}Processing PR #$PR_NUM...${NC}"
    
    # Get PR details
    PR_INFO=$(gh pr view "$PR_NUM" --repo "$OWNER/$REPO" --json number,title,state,mergeable,mergeStateStatus)
    PR_TITLE=$(echo "$PR_INFO" | jq -r '.title')
    PR_MERGEABLE=$(echo "$PR_INFO" | jq -r '.mergeable')
    
    echo "  Title: $PR_TITLE"
    
    # Check if PR is mergeable
    if [ "$PR_MERGEABLE" == "CONFLICTING" ]; then
        echo -e "  ${RED}✗ Skipping - has merge conflicts${NC}"
        ((SKIPPED++))
        echo ""
        continue
    fi
    
    # Try to merge with rebase
    if gh pr merge "$PR_NUM" --repo "$OWNER/$REPO" --rebase --auto; then
        echo -e "  ${GREEN}✓ Successfully merged (or queued for auto-merge)${NC}"
        ((MERGED++))
    else
        echo -e "  ${RED}✗ Failed to merge${NC}"
        ((FAILED++))
    fi
    
    echo ""
    
    # Small delay to avoid rate limiting
    sleep 1
done

# Summary
echo ""
echo -e "${GREEN}=== Merge Summary ===${NC}"
echo "Total PRs processed: $PR_COUNT"
echo -e "${GREEN}Merged: $MERGED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo -e "${YELLOW}Skipped: $SKIPPED${NC}"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "${YELLOW}Some PRs failed to merge. Please check them manually:${NC}"
    echo "  gh pr list --repo $OWNER/$REPO --state open"
fi

echo ""
echo -e "${GREEN}Done!${NC}"
