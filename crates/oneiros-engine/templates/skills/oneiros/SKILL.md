---
name: oneiros
description: >
  Continuity backend for AI agents. Use whenever agents must remember, think,
  discover, realize, understand, gather knowledge, consider options, design,
  collaborate, or receive feedback. Triggers on mention of thinking, remembering,
  investigating, discussing, desinging, preparing, planning, dreaming, gardening,
  or caution.
compatibility: Requires oneiros CLI tool or MCP access, and an initialized oneiros project.
allowed-tools: Read Bash(oneiros:*)
license: "MIT"
metadata:
  version: "{{VERSION}}"
  author: "JC McCormick <https://github.com/esmevane>"
---

# Oneiros — Your Continuity

Oneiros gives you persistent thoughts, knowledge, and identity across sessions. Start with `oneiros wake <agent>` to restore your identity. Tool responses will guide you from there — each response includes navigational hints showing what to do next.

**Setup:** Run `oneiros doctor` to diagnose, or `oneiros setup` for guided first-run configuration.

**MCP access:** If MCP is configured, use `activate-toolset` to load tools for your current cognitive moment (lifecycle, capture, garden, admin, distribute). Resources like `oneiros-mcp://agents` and `oneiros-mcp://agent/{name}/cognitions` are browsable without tool calls.

**Two ways in:** CLI (`oneiros` commands via Bash) and MCP tools both access the same brain — data created through one is visible in the other.
