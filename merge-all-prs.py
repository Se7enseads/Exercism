#!/usr/bin/env python3
"""
Script to rebase and merge all open Exercism solution PRs.
This script provides a more robust alternative to the bash script with better error handling.
"""

import subprocess
import json
import sys
import time
from typing import List, Dict, Tuple

# ANSI color codes
GREEN = '\033[0;32m'
YELLOW = '\033[1;33m'
RED = '\033[0;31m'
BLUE = '\033[0;34m'
NC = '\033[0m'  # No Color

OWNER = "Se7enseads"
REPO = "Exercism"
EXCLUDE_PR = 43  # The current PR that creates this script


def check_gh_cli() -> bool:
    """Check if GitHub CLI is installed and authenticated."""
    try:
        subprocess.run(['gh', '--version'], capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        print(f"{RED}Error: GitHub CLI (gh) is not installed.{NC}")
        print("Please install it from: https://cli.github.com/")
        return False
    
    try:
        subprocess.run(['gh', 'auth', 'status'], capture_output=True, check=True)
    except subprocess.CalledProcessError:
        print(f"{RED}Error: Not authenticated with GitHub CLI.{NC}")
        print("Please run: gh auth login")
        return False
    
    return True


def get_open_prs() -> List[Dict]:
    """Fetch all open PRs from the repository."""
    try:
        result = subprocess.run(
            ['gh', 'pr', 'list', '--repo', f'{OWNER}/{REPO}', 
             '--state', 'open', '--json', 'number,title,mergeable,mergeStateStatus', 
             '--limit', '100'],
            capture_output=True,
            text=True,
            check=True
        )
        prs = json.loads(result.stdout)
        # Exclude the current PR
        return [pr for pr in prs if pr['number'] != EXCLUDE_PR]
    except subprocess.CalledProcessError as e:
        print(f"{RED}Error fetching PRs: {e}{NC}")
        return []
    except json.JSONDecodeError as e:
        print(f"{RED}Error parsing PR data: {e}{NC}")
        return []


def merge_pr(pr_number: int) -> Tuple[bool, str]:
    """
    Attempt to merge a PR with rebase strategy.
    Returns (success: bool, message: str)
    """
    try:
        result = subprocess.run(
            ['gh', 'pr', 'merge', str(pr_number), '--repo', f'{OWNER}/{REPO}',
             '--rebase', '--auto'],
            capture_output=True,
            text=True,
            check=True
        )
        return True, "Successfully merged or queued for auto-merge"
    except subprocess.CalledProcessError as e:
        error_msg = e.stderr.strip() if e.stderr else str(e)
        return False, f"Failed to merge: {error_msg}"


def main():
    """Main function to orchestrate PR merging."""
    print(f"{GREEN}=== Exercism PR Merge Script ==={NC}\n")
    
    # Check prerequisites
    if not check_gh_cli():
        sys.exit(1)
    
    # Fetch PRs
    print(f"{YELLOW}Fetching open pull requests...{NC}")
    prs = get_open_prs()
    
    if not prs:
        print(f"{YELLOW}No pull requests to merge.{NC}")
        sys.exit(0)
    
    print(f"{GREEN}Found {len(prs)} pull requests to merge{NC}\n")
    
    # Display PRs
    print("Pull requests to be merged:")
    for i, pr in enumerate(prs, 1):
        mergeable_status = pr.get('mergeable', 'UNKNOWN')
        status_color = GREEN if mergeable_status == 'MERGEABLE' else YELLOW
        print(f"  {i:2d}. PR #{pr['number']:3d}: {pr['title'][:70]} {status_color}[{mergeable_status}]{NC}")
    
    print()
    
    # Ask for confirmation
    try:
        response = input(f"Do you want to proceed with rebasing and merging these PRs? (y/N): ")
        if response.lower() not in ['y', 'yes']:
            print(f"{YELLOW}Aborted by user.{NC}")
            sys.exit(0)
    except KeyboardInterrupt:
        print(f"\n{YELLOW}Aborted by user.{NC}")
        sys.exit(0)
    
    # Process PRs
    print(f"\n{GREEN}Starting merge process...{NC}\n")
    
    merged = 0
    failed = 0
    skipped = 0
    
    for pr in prs:
        pr_number = pr['number']
        pr_title = pr['title']
        mergeable = pr.get('mergeable', 'UNKNOWN')
        
        print(f"{YELLOW}Processing PR #{pr_number}...{NC}")
        print(f"  Title: {pr_title}")
        
        # Check if PR has conflicts
        if mergeable == 'CONFLICTING':
            print(f"  {RED}✗ Skipping - has merge conflicts{NC}\n")
            skipped += 1
            continue
        
        # Attempt to merge
        success, message = merge_pr(pr_number)
        
        if success:
            print(f"  {GREEN}✓ {message}{NC}\n")
            merged += 1
        else:
            print(f"  {RED}✗ {message}{NC}\n")
            failed += 1
        
        # Small delay to avoid rate limiting
        time.sleep(1)
    
    # Summary
    print(f"\n{GREEN}=== Merge Summary ==={NC}")
    print(f"Total PRs processed: {len(prs)}")
    print(f"{GREEN}Merged: {merged}{NC}")
    print(f"{RED}Failed: {failed}{NC}")
    print(f"{YELLOW}Skipped: {skipped}{NC}\n")
    
    if failed > 0:
        print(f"{YELLOW}Some PRs failed to merge. Please check them manually:{NC}")
        print(f"  gh pr list --repo {OWNER}/{REPO} --state open\n")
    
    if skipped > 0:
        print(f"{YELLOW}Some PRs were skipped due to conflicts. To resolve:{NC}")
        print(f"  1. Check out the PR: gh pr checkout <PR_NUMBER>")
        print(f"  2. Rebase: git rebase main")
        print(f"  3. Resolve conflicts and push: git push --force-with-lease\n")
    
    print(f"{GREEN}Done!{NC}")
    
    # Return appropriate exit code
    sys.exit(0 if failed == 0 else 1)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        print(f"\n{YELLOW}Interrupted by user.{NC}")
        sys.exit(130)
    except Exception as e:
        print(f"{RED}Unexpected error: {e}{NC}")
        sys.exit(1)
