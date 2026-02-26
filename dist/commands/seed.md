---
description: Apply predefined seed data (textures, levels, personas, agents)
argument-hint: "core"
---

Run `oneiros seed core` to apply core seed data: 8 textures, 5 levels, the process persona, and the governor.process agent.

Seeding is idempotent — running it multiple times is safe. Each seed entity is applied through the normal event pipeline via `set_texture`, `set_level`, `set_persona`, and `agent create`.
