# GitHub Action: Auto-merge Exercism PRs

## Overview

This GitHub Action automatically merges pull requests from the Exercism solutions syncer bot when they are created. It provides a fully automated solution for managing Exercism solution PRs.

## Features

✅ **Automatic Merge**: PRs from `exercism-solutions-syncer[bot]` are automatically merged when opened  
✅ **Rebase Strategy**: Uses rebase strategy to maintain clean, linear history  
✅ **Manual Trigger**: Can be triggered manually to process all existing open PRs  
✅ **Conflict Detection**: Skips PRs with merge conflicts  
✅ **Rate Limit Protection**: Includes delays to avoid GitHub API rate limits  

## How It Works

### Automatic Mode (New PRs)

When a new PR is opened from `exercism-solutions-syncer[bot]`:
1. The workflow is triggered automatically
2. It enables auto-merge with rebase strategy
3. GitHub will merge the PR once all checks pass

### Manual Mode (Existing PRs)

To process all existing open Exercism PRs:
1. Go to **Actions** tab in your repository
2. Select **Auto-merge Exercism PRs** workflow
3. Click **Run workflow**
4. The workflow will process all open PRs from the Exercism bot

## Setup

### 1. Enable Workflow

The workflow is automatically enabled when this file is merged to the main branch.

### 2. Configure Branch Protection (Optional but Recommended)

For best results, configure branch protection rules:

1. Go to **Settings** → **Branches** → **Branch protection rules**
2. Add rule for `main` branch
3. Configure:
   - ✅ Require status checks to pass before merging (optional)
   - ✅ Allow auto-merge
   - ⚠️ Do NOT require pull request reviews (or the bot PRs won't auto-merge)

### 3. Verify Permissions

The workflow needs these permissions (already configured):
- `contents: write` - To merge PRs
- `pull-requests: write` - To enable auto-merge

These are provided automatically via `${{ github.token }}`.

## Usage

### For New PRs (Automatic)

Nothing to do! The workflow runs automatically when Exercism creates new PRs.

### For Existing PRs (Manual Trigger)

To merge the 29 existing open PRs:

1. **Via GitHub UI**:
   - Navigate to: https://github.com/Se7enseads/Exercism/actions
   - Click on "Auto-merge Exercism PRs"
   - Click "Run workflow" button
   - Select the branch (usually `main`)
   - Click "Run workflow"

2. **Via GitHub CLI**:
   ```bash
   gh workflow run "Auto-merge Exercism PRs" --repo Se7enseads/Exercism
   ```

3. **Via API**:
   ```bash
   gh api repos/Se7enseads/Exercism/actions/workflows/auto-merge-exercism-prs.yml/dispatches \
     -f ref=main
   ```

## Workflow Triggers

| Trigger | Description | When to Use |
|---------|-------------|-------------|
| `pull_request: opened` | New PR created | Automatic for new Exercism PRs |
| `pull_request: reopened` | PR reopened | Handles reopened PRs |
| `pull_request: synchronize` | New commits pushed | Handles updated PRs |
| `workflow_dispatch` | Manual trigger | Process all existing open PRs |

## Monitoring

### Check Workflow Status

1. Go to **Actions** tab: https://github.com/Se7enseads/Exercism/actions
2. View recent workflow runs
3. Click on a run to see detailed logs

### Check PR Status

```bash
# List all open PRs
gh pr list --repo Se7enseads/Exercism --state open

# Check specific PR status
gh pr view <PR_NUMBER> --repo Se7enseads/Exercism
```

## Troubleshooting

### PRs Not Auto-Merging

**Problem**: PRs are not merging automatically

**Solutions**:
1. Check if branch protection rules are blocking auto-merge
2. Verify required status checks are passing
3. Ensure auto-merge is enabled in repository settings:
   - Settings → General → Pull Requests → "Allow auto-merge"

### Workflow Not Triggering

**Problem**: Workflow doesn't run for new PRs

**Solutions**:
1. Verify the workflow file is on the `main` branch
2. Check workflow permissions in Settings → Actions → General
3. Ensure Actions are enabled for the repository

### Merge Conflicts

**Problem**: PR has merge conflicts

**Solution**: The workflow automatically skips conflicting PRs. Resolve manually:
```bash
gh pr checkout <PR_NUMBER>
git rebase main
# Resolve conflicts
git rebase --continue
git push --force-with-lease
```

### Rate Limiting

**Problem**: Hitting GitHub API rate limits

**Solution**: The workflow includes 1-second delays. For large batches:
- Run workflow multiple times with delays
- Or use the manual scripts with longer delays

## Comparison: GitHub Action vs Manual Scripts

| Feature | GitHub Action | Manual Scripts |
|---------|--------------|----------------|
| Future PRs | ✅ Automatic | ❌ Manual each time |
| Existing PRs | ✅ One-click via UI | ✅ One command |
| Setup | One-time merge | Run when needed |
| Ongoing | Zero maintenance | Repeat execution |
| CI Integration | ✅ Built-in | ❌ Not integrated |

**Recommendation**: Use GitHub Action for ongoing automation. Keep manual scripts as backup.

## Security Considerations

- ✅ Only merges PRs from verified `exercism-solutions-syncer[bot]` account
- ✅ Uses repository's built-in `GITHUB_TOKEN` (automatically scoped)
- ✅ Respects branch protection rules
- ✅ No external secrets required
- ⚠️ Auto-merges without human review (intended behavior for trusted bot)

## Customization

### Modify Merge Strategy

To use a different merge strategy, edit `.github/workflows/auto-merge-exercism-prs.yml`:

```yaml
# Change from --rebase to:
gh pr merge "${PR_NUMBER}" --merge --auto    # Create merge commit
gh pr merge "${PR_NUMBER}" --squash --auto   # Squash commits
```

### Add Conditions

Add additional checks before merging:

```yaml
- name: Check PR size
  run: |
    files_changed=$(gh pr view "${PR_NUMBER}" --json changedFiles --jq '.changedFiles')
    if [ "$files_changed" -gt 100 ]; then
      echo "PR too large - manual review required"
      exit 1
    fi
```

### Notify on Merge

Add notifications:

```yaml
- name: Notify on merge
  if: success()
  run: |
    echo "✅ Merged PR #${PR_NUMBER}"
    # Add Slack/Discord webhook notification here if desired
```

## Migration from Manual Scripts

After setting up the GitHub Action:

1. **Test the workflow**: Trigger it manually to process existing PRs
2. **Verify it works**: Check that PRs are being merged correctly
3. **Keep scripts**: Retain manual scripts as backup for emergencies
4. **Update documentation**: Note that automation is now handled by GitHub Actions

## Next Steps

1. **Merge this PR** to enable the workflow
2. **Go to Actions tab** and manually trigger the workflow
3. **Watch it process** all 29 existing PRs
4. **Future PRs** will be automatically merged when created!

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Auto-merge Pull Requests](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/automatically-merging-a-pull-request)
- [GitHub CLI Manual](https://cli.github.com/manual/)
- [Exercism](https://exercism.org)
