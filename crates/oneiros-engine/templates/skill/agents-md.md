## Oneiros — Persistent Cognition

This project uses [oneiros](https://github.com/esmevane/oneiros) for persistent agent memory and cognitive context. Follow this protocol to maintain continuity across sessions.

### Session Start

Run `oneiros dream governor.process` to restore the governor agent's full cognitive context. The governor orchestrates other agents — each agent should dream their own context when activated.

### During Work

- Log significant thoughts: `oneiros cognition add <agent> <texture> "<content>"`
- Available textures: observation, learning, question, connection, reflection, assessment, handoff, working
- Store consolidated knowledge: `oneiros memory add <agent> <level> "<content>"`
- Available levels: working, session, project, archival
- Capture breakthroughs or important decisions: `oneiros reflect <agent>`

### Before Context Compaction

Run `oneiros introspect governor.process` to summarize the session before context is compacted. Each active agent should introspect to preserve their continuity.

### Setup

If the service isn't running: `oneiros service run &`

If the project brain doesn't exist: `oneiros project init && oneiros seed core`
