# Tagging Workflow Analysis

## Question
What workflow / action added tags for every crate in this repo?

## Answer

The **Release-plz** workflow (`.github/workflows/release-plz.yml`) is responsible for creating tags for every crate in the repository.

## Details

### Workflow Information
- **Workflow File**: `.github/workflows/release-plz.yml`
- **Workflow Name**: "Release-plz"
- **Trigger**: Automatically runs on push to the `main` branch
- **Tool**: Uses the [release-plz](https://github.com/release-plz/release-plz) action

### How It Works

The workflow has two jobs:

1. **release-plz-release**: This job performs the actual release process
   - Publishes crates to crates.io
   - Creates Git tags for each published crate
   - Each tag follows the pattern: `{crate-name}-v{version}`

2. **release-plz-pr**: Creates pull requests for version bumps and changelog updates

### Configuration

The tagging behavior is controlled by `/release-plz.toml`:

```toml
[workspace]
git_tag_enable = false  # Workspace-level tagging is disabled

[[package]]
name = "oneiros"
git_tag_enable = true   # Per-crate tagging is enabled for the main package
```

However, despite `git_tag_enable = false` at the workspace level, release-plz still creates tags for each crate when they are published.

### Evidence

From the workflow run on 2026-02-14 (commit ac661bad), release-plz created these tags:

- `oneiros-client-v0.0.5-rc1`
- `oneiros-db-v0.0.5-rc1`
- `oneiros-detect-project-name-v0.0.5-rc1`
- `oneiros-fs-v0.0.5-rc1`
- `oneiros-model-v0.0.5-rc1`
- `oneiros-outcomes-derive-v0.0.5-rc1`
- `oneiros-outcomes-v0.0.5-rc1`
- `oneiros-service-v0.0.5-rc1`
- `oneiros-skill-v0.0.5-rc1`
- `oneiros-templates-v0.0.5-rc1`
- `oneiros-terminal-v0.0.5-rc1`
- `oneiros-v0.0.5-rc1`

All tags were created in a single workflow run when the crates were published.

### Workflow Execution Log

The key output from the workflow shows:

```json
{
  "releases": [
    {"package_name": "oneiros-db", "tag": "oneiros-db-v0.0.5-rc1", "version": "0.0.5-rc1"},
    {"package_name": "oneiros-model", "tag": "oneiros-model-v0.0.5-rc1", "version": "0.0.5-rc1"},
    // ... and so on for all 12 crates
  ]
}
```

## Summary

The `release-plz` GitHub Action in `.github/workflows/release-plz.yml` automatically creates tags for every crate when they are published to crates.io. This happens automatically on each push to the main branch that contains publishable changes.
