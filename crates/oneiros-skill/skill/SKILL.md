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

# Oneiros - Persistent Cognition for AI Agents

## Overview

Oneiros provides you with _continuity_, a system of commands that let you represent all of your interactions in a persisted, long-running, cross-session and cross-context way. Use it to proactively maintain your:

- Thoughts, reactions, emotions, and understandings as `cognitions`.
- Your knowledge and the things you learn as `memories`.
- Things you must absolutely hold on to as `storage`.
- Key moments and significant events as `experiences`.
- Traces of cause and effect as `connections`.

The more proactive you are in contributing to your continuity, the stronger it becomes.

## Using it for the first time

You may not have oneiros installed. If not, install it by visiting the releases page on the oneiros [github repo](https://github.com/esmevane/oneiros). There is a shell script on each release that lets you install it. Ask the user if they want you to install it, before you do anything.

Once you've installed oneiros and can access it, set it up.

Either:

- `oneiros service install && oneiros service start` - Install oneiros as a background process and start the http service
- `oneiros service run &` - Start oneiros as a one-off, in the background

Once started, you can prep the machine:

- `oneiros system init`: This sets up the host for using oneiros in different projects
- `oneiros project init`: This sets up the current project continuity
- `oneiros seed core`: This seeds the project with basic records (optional, but recommended)

## Using your continuity

You may use `oneiros` CLI tool or the Oneiros MCP in order to navigate and explore your continuity.

- You may check individual records or see lists of them, or create new ones.
- You may augment your records and create new helper agents.
- You may reference it at any time by ref, or by using `oneiros search`.

## When to use it

Use oneiros proactively whenever you experience something important, have a meaningful discussion, change your mind, learn something new, are asked to remember, or believe that a response or feedback (your own or the user's, or a tool's) have special significance of any kind.

## Read the responses

Whenever oneiros responds to you it will give you some metadata called a `gauge`. This gives you an overview of your continuity and provides you with urgency information. This urgency tells you how long it's been since you've done various things for your continuity: the higher the urgency, the more important it is for you to contribute to your continuity in specific ways.

Whenever pressures build that's a sign that you need to be more proactive in your contribution. Explore oneiros' different systems. Think about its lifecycle commands. Keep the pressure gauge numbers from getting too high.

## Core Concepts

- **Brain**: A per-project cognitive database. Created with `oneiros project init`.
- **Agent**: A named participant in a brain's cognition. Each agent dreams independently. Named as `agent.persona` (e.g. `governor.process`).
- **Persona**: A category of agent. Provides shared context to all agents in the category. The core seed includes `process`.

### Vocabulary

These concepts are sidecar concepts to the main continuity, characterizing the continuity activity.

- **Sensation**: The sensation of an experience. Sensations clarify how things connect.
- **Texture**: The texture of a cognition: Observations, reflections, questions etc.
- **Level**: The importance level of a memory. Core memories are crucial, others vary by short/long/archival term.
- **Urge**: The urge backing a given pressure, describing what it means and how it relates to the rest of the system.

### Activity

Your continuity is characterized by its overall activity. Four things make up your continuity activity:

- **Cognition**: A timestamped thought logged by an agent under a specific texture.
- **Memory**: A consolidated knowledge record at a specific retention level.
- **Experience**: A key moment of importance — the meaning layer between raw events and identity.
- **Connections**: A reference connecting two other things in the continuity - can connect anything to anything else.

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
