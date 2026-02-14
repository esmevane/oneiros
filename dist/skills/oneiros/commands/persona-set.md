---
description: Create or update a persona (agent category)
argument-hint: "<name> [--description <text>] [--prompt <text>]"
---

Run `oneiros persona set $ARGUMENTS` to create or update a persona. The name is required. Use `--description` for a brief category description and `--prompt` for shared context that applies to all agents in this category.

Personas are idempotent — setting the same name again updates the existing persona. Use seed data for standard personas: `oneiros seed core`.
