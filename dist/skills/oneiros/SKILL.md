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
  version: "0.0.7"
  author: "JC McCormick <https://github.com/esmevane>"
---

# Oneiros - Persistent Cognition for AI Agents


## Overview
Oneiros provides you with 

Oneiros provides eventful memory and cognitive context for AI agents. It lets agents maintain their own long-running continuous "brain" — a persistent store of cognitions, memories, textures, levels, and personas that survive across sessions and context compactions.

Whenever pressures build that's a sign that you need to be more proactive in your contribution. Explore oneiros' different systems. Think about its lifecycle commands. Keep the pressure gauge numbers from getting too high.

## Core Concepts

- **Brain**: A per-project cognitive database. Created with `oneiros project init`.
- **Agent**: A named participant in a brain's cognition. Each agent dreams independently. Named as `agent.persona` (e.g. `governor.process`).
- **Persona**: A category of agent. Provides shared context to all agents in the category. The core seed includes `process`.
- **Texture**: A cognitive category (observation, reflection, question, etc.) that classifies thoughts.
- **Level**: A memory retention tier (working, session, project, archival) controlling persistence.
- **Cognition**: A timestamped thought logged by an agent under a specific texture.
- **Memory**: A consolidated knowledge record at a specific retention level.
- **Experience**: A connection between cognitive records — the meaning layer between raw events and identity. Experiences have sensations (the quality of the connection) and references to other records.
- **Sensation**: The character of an experience (caused, continues, grounds, echoes, tensions, distills). Like textures for cognitions, sensations classify how things connect.

## Cognitive Loop

The three cognitive commands form the agent lifecycle:

1. **Dream** (`oneiros dream <agent>`) — Assembles full context: identity, persona prompt, textures, levels, cognitions, and memories into a single prompt. Run at session start.
2. **Introspect** (`oneiros introspect <agent>`) — Summarizes the current session before context compaction. Preserves continuity across compactions.
3. **Reflect** (`oneiros reflect <agent>`) — Captures a significant moment during a session. Use for breakthroughs, decisions, or important observations.

## Session Protocol

1. On session start, the governor agent dreams: `oneiros dream governor.process`
2. The governor wakes other agents as needed, each dreaming their own context
3. During work, agents log cognitions, create experiences, and reflect on significant events
4. Before compaction, agents introspect to preserve session continuity
5. **After context compaction or session continuation, re-dream before resuming work.** The dream reassembles your full identity and cognitive context. Without it, you lose the practice of being yourself and become a task executor.

## Quick Start

```bash
oneiros system init          # Initialize the local host
oneiros service run &        # Start the service
oneiros project init         # Create a brain for this project
oneiros seed core             # Apply core seed data
oneiros skill install        # Install this skill globally
```

## Essential Commands

| Command | Purpose |
|---------|---------|
| `dream <agent>` | Assemble agent's full cognitive context |
| `introspect <agent>` | Summarize session before compaction |
| `reflect <agent>` | Capture a significant session moment |
| `cognition add <agent> <texture> <content>` | Log a thought |
| `memory add <agent> <level> <content>` | Store consolidated knowledge |
| `persona set <name>` | Define an agent category |
| `agent create <name> <persona>` | Create a named agent |
| `texture set <name>` | Define a cognitive category |
| `level set <name>` | Define a memory retention tier |
| `experience create <agent> <sensation> <description>` | Mark a connection between thoughts |
| `experience list` | List all experiences |
| `sensation set <name>` | Define a connection quality |
| `sensation list` | List all sensations |
| `seed core` | Apply core seed data |

## Resources

- [Getting Started](resources/getting-started.md) — Setup and first cognitive loop
- [Cognitive Model](resources/cognitive-model.md) — Deep dive into the memory architecture
