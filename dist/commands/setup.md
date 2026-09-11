---
description: Guided first-run setup — initializes host, project, vocabulary, agents, MCP config, and service in one flow.
argument-hint: "[--name <name>] [--accept-all] [--install-host] [--init-mcp]"
---

Run `oneiros setup` for a guided first-run experience. It walks through each step of setting up oneiros for a project:

1. Host initialization
2. Project initialization (project creation)
3. Vocabulary seeding (textures, levels, sensations, natures, personas, urges)
4. Agent seeding (governor, oneiroi, activity scribe)
5. MCP config (Claude Code integration)
6. Service installation and start

Use `--accept-all` (`-y`) to accept all optional steps without prompting. Use `--install-host` or `--init-mcp` to opt into specific optional steps. Use `--name` to set the host name.

Each step is idempotent — running setup again skips what's already done.
