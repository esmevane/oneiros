---
description: Create a new agent with a persona
argument-hint: "<name> <persona> [--description <text>] [--prompt <text>]"
---

Run `oneiros agent create $ARGUMENTS` to create a named agent assigned to an existing persona. The persona must exist first.

Use `--description` for a brief agent description and `--prompt` for the agent's behavioral identity. The persona provides shared category context; the agent prompt defines what makes this agent unique within that category.
