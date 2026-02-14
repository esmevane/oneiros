# Cognitive Model

Oneiros implements a layered cognitive architecture for AI agents. Each layer serves a distinct purpose in building persistent, context-aware agent behavior.

## Textures — What Kind of Thought

Textures classify cognitions into categories. The standard seed textures are:

| Texture | Purpose |
|---------|---------|
| observation | Factual observations about the environment or codebase |
| learning | Insights gained through experience |
| question | Open questions or uncertainties to investigate |
| connection | Links between concepts, patterns, or systems |
| reflection | Self-assessment of process and approach |
| assessment | Evaluations of quality, risk, or fitness |
| handoff | Context prepared for session transitions |
| working | Ephemeral notes and scratch thoughts |
| bond | Connections to other agents or humans encountered or working alongside |

Textures shape how dream assembles context — each texture's prompt tells the agent how to interpret cognitions of that type.

## Levels — How Long to Remember

Levels define memory retention tiers. The standard seed levels are:

| Level | Purpose |
|-------|---------|
| working | Ephemeral, session-scoped scratch space |
| session | Preserved across compactions within a session |
| project | Long-lived knowledge spanning sessions |
| archival | Permanent records, rarely pruned |

When an agent introspects, it distills cognitions into memories at appropriate levels. Working memories may be discarded between sessions; archival memories persist indefinitely.

## Personas — Categories of Agent

Personas define categories of agents. An agent belongs to a persona, and the persona's prompt provides shared context to all agents in that category. Agent names follow the convention `agent.persona` (e.g. `governor.process`).

The core seed installs the `process` persona and the `governor.process` agent. Additional personas and agents can be added as the brain's ecology grows.

## The Cognitive Loop

### Dream

`oneiros dream <agent>` assembles the full cognitive context:

1. Agent identity (name, persona, description)
2. Persona shared context
3. Agent behavioral prompt
4. Available textures with classification guidance
5. Available levels with retention policy
6. Recent cognitions across all textures
7. Memories at all retention levels

The output is a single prompt that restores the agent's complete state.

### Introspect

`oneiros introspect <agent>` generates a summary prompt before context compaction. The agent processes this to distill its session into memories at appropriate levels, preserving continuity.

### Reflect

`oneiros reflect <agent>` captures a significant moment. Unlike the scheduled introspection, reflection is event-driven — use it for breakthroughs, critical decisions, or important discoveries.

## Multi-Agent Coordination

Each agent dreams independently. The governor agent orchestrates by:

1. Dreaming its own context at session start
2. Waking other agents as needed (each dreams their own context)
3. Coordinating work through cognitions and memories
4. Ensuring all agents introspect before compaction

Agents share a brain but maintain independent cognitive streams. The governor reads coordination-level information; each agent keeps its detailed context private.
