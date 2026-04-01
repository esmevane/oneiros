---
description: Guided first-run setup — initializes system, project, vocabulary, agents, MCP config, and service in one flow.
argument-hint: "[--name <name>] [--yes]"
---

Run `oneiros setup` for a guided first-run experience. It walks through each step of setting up oneiros for a project:

1. System initialization (host setup)
2. Project initialization (brain creation)
3. Vocabulary seeding (textures, levels, sensations, natures, personas, urges)
4. Agent seeding (governor, oneiroi, activity scribe)
5. MCP config (Claude Code integration)
6. Service installation and start

Use `--yes` to accept all defaults without prompting. Use `--name` to set the system/host name.

Each step is idempotent — running setup again skips what's already done.
