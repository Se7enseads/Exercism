# How to Merge All Open Pull Requests

This guide explains how to merge all 29 open Exercism solution PRs into the main branch.

## Quick Start

### Option 1: Using the Python Script (Recommended)
```bash
python3 merge-all-prs.py
```

### Option 2: Using the Bash Script
```bash
./merge-all-prs.sh
```

### Option 3: Using GitHub CLI Manually
```bash
# List all PRs
gh pr list --repo Se7enseads/Exercism --state open

# Merge each PR one at a time
gh pr merge <PR_NUMBER> --repo Se7enseads/Exercism --rebase --auto
```

## What These Scripts Do

1. Fetch all open PRs (29 Exercism solution PRs)
2. Display them for your review
3. Ask for confirmation
4. Automatically rebase and merge each PR
5. Provide a summary of results

## Requirements

- **GitHub CLI**: Install with `brew install gh` (macOS) or see https://cli.github.com/
- **Authentication**: Run `gh auth login` if not already authenticated
- **Python 3**: Only needed if using the Python script

## After Running

Once complete:
1. All Exercism solutions will be merged into main branch
2. Check that everything looks good
3. Close PR #43 (this automation PR)

## Full Documentation

See `README-MERGE-SCRIPT.md` for detailed documentation, troubleshooting, and manual merge instructions.

## Support

- For script issues: Check prerequisites and error messages
- For merge conflicts: See the troubleshooting section in README-MERGE-SCRIPT.md
- For GitHub CLI help: https://cli.github.com/manual/
