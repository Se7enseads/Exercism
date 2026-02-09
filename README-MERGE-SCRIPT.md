# PR Merge Automation Script

This repository contains a script to automatically rebase and merge all open Exercism solution PRs.

## Overview

The repository has 29 open pull requests from the Exercism solutions syncer bot that add exercise solutions. This script automates the process of merging all these PRs with rebase strategy.

## Prerequisites

1. **GitHub CLI (`gh`)**: Install from [cli.github.com](https://cli.github.com/)
   ```bash
   # On macOS
   brew install gh
   
   # On Linux
   sudo apt install gh  # Debian/Ubuntu
   sudo dnf install gh  # Fedora
   
   # On Windows
   winget install --id GitHub.cli
   ```

2. **Authentication**: Login to GitHub CLI
   ```bash
   gh auth login
   ```

3. **Permissions**: Ensure you have write access to the repository

## Usage

### Option 1: Using the Automated Script

```bash
# Make the script executable
chmod +x merge-all-prs.sh

# Run the script
./merge-all-prs.sh
```

The script will:
1. Fetch all open PRs (excluding PR #43 which is the current automation PR)
2. Display the list of PRs to be merged
3. Ask for confirmation
4. Attempt to rebase and merge each PR automatically
5. Provide a summary of results

### Option 2: Manual Merge with GitHub CLI

To merge PRs manually one at a time:

```bash
# List all open PRs
gh pr list --repo Se7enseads/Exercism --state open

# Merge a specific PR with rebase
gh pr merge <PR_NUMBER> --repo Se7enseads/Exercism --rebase --auto
```

### Option 3: Using GitHub Web Interface

1. Go to [Pull Requests](https://github.com/Se7enseads/Exercism/pulls)
2. For each PR:
   - Click on the PR
   - Click "Rebase and merge" button
   - Confirm the merge

## Pull Requests to Merge

The following PRs are currently open and will be merged:

| PR # | Title | Type |
|------|-------|------|
| 42 | [Sync Solution] rust/collatz-conjecture | Solution |
| 41 | [Sync Iteration] rust/collatz-conjecture/3 | Iteration |
| 40 | [Sync Iteration] rust/collatz-conjecture/2 | Iteration |
| 39 | [Sync Iteration] rust/collatz-conjecture/1 | Iteration |
| 38 | [Sync Iteration] rust/high-scores/1 | Iteration |
| 37 | [Sync Iteration] rust/anagram/1 | Iteration |
| 36 | [Sync Iteration] rust/clock/2 | Iteration |
| 35 | [Sync Iteration] rust/clock/1 | Iteration |
| 34 | [Sync Solution] rust/space-age | Solution |
| 33 | [Sync Iteration] rust/space-age/1 | Iteration |
| 32 | [Sync Iteration] x86-64-assembly/hello-world/1 | Iteration |
| 31 | [Sync Solution] rust/bob | Solution |
| 30 | [Sync Iteration] rust/bob/1 | Iteration |
| 29 | [Sync Iteration] rust/sum-of-multiples/1 | Iteration |
| 28 | [Sync Iteration] rust/raindrops/1 | Iteration |
| 27 | [Sync Iteration] rust/proverb/1 | Iteration |
| 26 | [Sync Iteration] rust/leap/3 | Iteration |
| 25 | [Sync Iteration] rust/leap/2 | Iteration |
| 24 | [Sync Iteration] rust/leap/1 | Iteration |
| 23 | [Sync Iteration] rust/reverse-string/1 | Iteration |
| 22 | [Sync Iteration] rust/gigasecond/1 | Iteration |
| 21 | [Sync Iteration] go/hello-world/1 | Iteration |
| 20 | [Sync Iteration] rust/armstrong-numbers/1 | Iteration |
| 19 | [Sync Iteration] c/hello-world/2 | Iteration |
| 16 | [Sync Iteration] go/lasagna/1 | Iteration |
| 15 | [Sync Iteration] go/weather-forecast/1 | Iteration |
| 14 | [Sync Solution] go/weather-forecast | Solution |
| 13 | [Sync Solution] c/hello-world | Solution |
| 12 | [Sync Iteration] c/hello-world/1 | Iteration |

**Total: 29 PRs**

## Merge Strategy

The script uses the **rebase and merge** strategy which:
- Rebases the PR branch onto the base branch (main)
- Creates a clean, linear commit history
- Maintains individual commit messages from each PR
- Avoids merge commits

## Troubleshooting

### Merge Conflicts

If a PR has merge conflicts:
1. The script will skip it automatically
2. You can resolve conflicts manually:
   ```bash
   # Fetch the PR branch
   gh pr checkout <PR_NUMBER>
   
   # Rebase onto main
   git rebase main
   
   # Resolve conflicts and continue
   git rebase --continue
   
   # Push the rebased branch
   git push --force-with-lease
   
   # Merge via CLI
   gh pr merge <PR_NUMBER> --rebase
   ```

### Rate Limiting

The script includes a 1-second delay between merges to avoid GitHub API rate limits. If you hit rate limits:
- Wait a few minutes
- Resume by running the script again (it will only process remaining open PRs)

### Failed Merges

If a merge fails:
1. Check the PR status: `gh pr view <PR_NUMBER>`
2. Verify the PR is mergeable
3. Check for branch protection rules or required status checks
4. Try merging manually through the web interface

## After Merging

Once all PRs are merged:
1. Verify the main branch has all changes
2. Review the commit history: `git log --oneline -30`
3. Ensure all Exercism solutions are in the correct directories
4. Close this PR (#43) as completed

## Support

For issues with:
- **The script**: Review the error messages and check prerequisites
- **GitHub CLI**: See [GitHub CLI documentation](https://cli.github.com/manual/)
- **Exercism integration**: Visit [Exercism documentation](https://exercism.org/docs)
