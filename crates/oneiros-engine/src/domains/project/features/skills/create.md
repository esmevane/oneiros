---
description: Create a project on this host
argument-hint: [--name <name>] [--yes]
---

Run `oneiros project create $ARGUMENTS` to provision a project on this host. Without `--name`, the name is auto-detected from the workspace (Cargo.toml, package.json, git repo name, or directory name).

This is idempotent — running it on an already-created project is safe.
