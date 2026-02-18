---
description: Create a new experience connecting cognitive records
argument-hint: "<agent> <sensation> <description> [--ref <id:kind[:role]>]..."
---

Run `oneiros experience create $ARGUMENTS` to create a new experience. Requires the agent name, sensation name, and a description of the connection.

Use `--ref` to link cognitive records (cognitions, memories, experiences, or storage entries). Format: `--ref <id>:<kind>` or `--ref <id>:<kind>:<role>`. Repeat `--ref` for multiple references.

Experiences are the edges in the cognitive graph — they describe how thoughts relate to each other.
