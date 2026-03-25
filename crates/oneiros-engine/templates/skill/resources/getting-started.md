# Getting Started with Oneiros

## Prerequisites

- Oneiros binary installed and on your PATH
- Claude Code (for skill integration)

## Initial Setup

### 1. Initialize the Host

```bash
oneiros system init
```

This creates the system database at `~/.local/share/oneiros/` and registers a default tenant. Run once per machine.

### 2. Start the Service

```bash
oneiros service run &
```

Or install as a managed service for automatic startup:

```bash
oneiros service install
oneiros service start
```

### 3. Initialize a Project

Navigate to your project directory and run:

```bash
oneiros project init
```

This creates a brain for the project. The project name is auto-detected from Cargo.toml, package.json, git remote, or directory name.

### 4. Seed the Brain

```bash
oneiros seed core
```

This populates the brain with standard textures (cognitive categories), levels (memory tiers), the process persona (agent category), and the governor.process agent.

### 5. Install the Skill

```bash
oneiros skill install
```

This writes the SKILL.md, plugin.json, and command files to `~/.claude/` so Claude Code discovers oneiros automatically.

## Verify

```bash
oneiros doctor
```

Check that system, service, and project are all healthy.

## First Cognitive Loop

Once set up, the cognitive loop runs automatically through hooks:

1. **Session start** — `oneiros dream governor.process` fires, assembling the governor's full context
2. **During work** — Agents log cognitions and reflect on significant events
3. **Before compaction** — `oneiros introspect governor.process` fires, preserving session continuity

To manually test the loop:

```bash
# Create an agent
oneiros agent create my-agent process

# Dream to see the assembled context
oneiros dream my-agent

# Log a cognition
oneiros cognition add my-agent observation "Testing the cognitive loop"

# Store a memory
oneiros memory add my-agent working "The cognitive loop works as expected"

# Dream again to see the new cognition and memory in context
oneiros dream my-agent
```
