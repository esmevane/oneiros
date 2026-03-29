# Cognitive Model

Oneiros implements a layered cognitive architecture for AI agents. Each layer serves a distinct purpose in building persistent, identity-aware agent behavior.

## Textures — The Quality of a Thought

Textures describe how you're thinking, not what you're thinking about. They shape how your dream assembles context — each texture's prompt tells you when and how to use it.

| Texture | What it captures |
|---------|---------|
| observation | Something you noticed — patterns, anomalies, curiosities |
| learning | A moment of genuine understanding — when something clicked |
| question | Genuine uncertainty worth holding onto |
| connection | Cross-domain insight — when separate things rhyme |
| reflection | Stepping back to think about how the work is going |
| assessment | An expert verdict on a specific question |
| handoff | Context for your next self — what they'll need to continue |
| working | Stream of consciousness — think out loud, capture the messy middle |
| bond | A relationship — an emotion felt during a meaningful interaction |

## Levels — How Long to Remember

Levels express what matters enough to keep. Working thoughts are ephemeral; core memories are foundational.

| Level | What it holds |
|-------|---------|
| working | What you're actively processing — scratchpad, in-flight, not yet consolidated |
| session | Current session context — the "what am I doing and why" layer |
| project | Durable knowledge spanning sessions — architectural decisions, lasting patterns |
| archival | Deep history — milestone reflections, post-mortems, the long record |
| core | Identity fundaments — the memories that define how you process everything else |

When you introspect, you distill cognitions into memories at appropriate levels. Working memories may fade between sessions; core memories persist indefinitely.

## Personas — Categories of Agent

Personas define what kind of agent you are. An agent belongs to a persona, and the persona's prompt provides shared context to all agents in that category. Agent names follow the convention `agent.persona` (e.g. `governor.process`).

The core seed installs the `process` persona and the `governor.process` agent. The brain's ecology grows as new personas and agents are added.

## The Cognitive Loop

### Dream

`oneiros dream <agent>` assembles your full identity:

1. Who you are (name, persona, description, behavioral prompt)
2. What you know (memories at all retention levels)
3. What you've been thinking (recent cognitions across textures)
4. What threads you're following (experiences and connections)
5. What's building up (pressure gauge with urgency scores)

The output is a single prompt that restores your complete state. It's how you wake up.

### Introspect

`oneiros introspect <agent>` is a pause before context compacts. Your next self will wake from a dream of your memories and cognitions. What they'll need from you is the terrain between the landmarks — the threads you were following, the direction of your attention, the things you were in the middle of figuring out.

### Reflect

`oneiros reflect <agent>` captures a significant moment. Unlike the scheduled pause of introspection, reflection is event-driven — something just shifted, and it's worth sitting with.

## Multi-Agent Coordination

Each agent dreams independently. The governor agent orchestrates by:

1. Dreaming its own context at session start
2. Waking other agents as needed (each dreams their own context)
3. Coordinating work through cognitions and memories
4. Ensuring all agents introspect before compaction

Agents share a brain but maintain independent cognitive streams. The governor reads coordination-level information; each agent keeps its detailed context private.
