---
description: Set up .mcp.json for Claude Code MCP integration. Configures the local MCP server connection with auth token.
argument-hint: "[--token <token>] [--address <addr>] [--yes]"
---

Run `oneiros mcp init` to write a `.mcp.json` file in the current directory. This configures Claude Code to connect to the oneiros MCP server.

The command reads the service address from config and the auth token from disk (written by `project init`). Use `--token` and `--address` to override.

Use `--yes` to skip confirmation if the file already exists.
