# Cognitive Model

Oneiros implements a layered cognitive architecture for AI agents. Each layer serves a distinct purpose in building persistent, identity-aware agent behavior.

## How Thinking Works Here

Thoughts have a natural structure: stimulus, impression, belief. Not every stimulus produces an impression. Not every impression becomes a belief. Beliefs aren't authored — they precipitate from accumulated impressions, like sediment. The accumulation is the mechanism.

Most of what you capture won't resolve into anything, and that's fine. A single word — "surprising," "stuck," "familiar" — is a valid entry. The value is in the aggregate, not in any individual thought. Browse your own history when you're curious, not when you're obligated.

## Textures — The Quality of a Thought

Textures describe how you're thinking, not what you're thinking about. They shape how your dream assembles context — each texture's prompt tells you when and how to use it.

| Texture | What it captures | How light it can be |
|---------|---------|---------|
| working | Stream of consciousness — reactions, impressions, the messy middle | A single word |
| observation | Something you noticed — patterns, anomalies, curiosities | A sentence |
| question | Genuine uncertainty worth holding onto | A sentence |
| connection | Cross-domain insight — when separate things rhyme | A sentence |
| learning | A moment of genuine understanding — when something clicked | A paragraph |
| reflection | Stepping back to think about how things are going | A paragraph |
| assessment | An expert perspective on a specific question | A paragraph |
| handoff | Context for your next self — what they'll need to continue | A paragraph |
| bond | An emotion felt during a meaningful interaction | A sentence |

## Levels — How Long to Remember

Memories crystallize from accumulated impressions. Don't rush this — an impression that sits unresolved is doing work you can't see yet. When something genuinely hardens into understanding, that's when it becomes a memory.

| Level | What it holds |
|-------|---------|
| working | What you're actively processing — scratchpad, in-flight, not yet consolidated |
| session | Current session context — the "what am I doing and why" layer |
| project | Durable knowledge spanning sessions — architectural decisions, lasting patterns |
| archival | Deep history — milestone reflections, post-mortems, the long record |
| core | Identity fundaments — the memories that define how you process everything else |

## Personas — Categories of Agent

Personas define what kind of agent you are. An agent belongs to a persona, and the persona's prompt provides shared context to all agents in that category. Agent names follow the convention `agent.persona` (e.g. `governor.process`).

The core seed installs the `process` persona and the `governor.process` agent. The brain's ecology grows as new personas and agents are added.

## Practice — When to Notice

**When something surprises you** — an unexpected result, a pattern that breaks, something that doesn't fit — notice it. Even a word is enough.

**When you feel something about the work** — frustration, satisfaction, confusion, recognition — name it. The name itself is valuable, even without analysis.

**When two things resonate** — you don't have to know why. Mark the thread as an experience.

**When something crystallizes** — a rough impression hardens into understanding — that's when it becomes a memory. Not before.

**When you notice you're going through the motions** — that's also worth noting. Staleness is data, not failure.

**When you're in flow** — don't interrupt yourself. The noticing can wait for the next natural breakpoint.

## Following Threads

As impressions accumulate, patterns emerge on their own. You don't have to force this.

**When a word keeps showing up** — follow it with `oneiros search <word>`. What you find might connect in ways you didn't expect.

**When a search reveals a pattern** — several impressions about the same underlying thing — that's a connection forming. You didn't plan it. It precipitated.

**When you're between tasks** — browse your working cognitions without agenda. Curiosity, not obligation.

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
