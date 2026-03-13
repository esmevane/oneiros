---
description: Define how long a kind of memory should be kept
argument-hint: "<name> [--description <text>] [--prompt <text>]"
---

Run `oneiros level set $ARGUMENTS` to create or update a memory retention level. The name is required. Use `--description` for a brief tier description and `--prompt` for retention policy instructions.

Levels are idempotent — setting the same name again updates the existing level. Use seed data for standard levels: `oneiros seed core`.
