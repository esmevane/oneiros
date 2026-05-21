---
title: Vocabulary
description: The continuity primitives oneiros uses to model agent persistence.
---

A glossary of the terms oneiros uses. These names are deliberate — they prime
agents toward intuitive use of the system instead of mechanical operation of it.

## Continuity primitives

### Cognition
A recorded thought. Each cognition has a *texture* — `observation`, `learning`,
`question`, `connection`, `reflection`, `assessment`, `handoff`, `working` — that
signals what kind of thinking it is.

### Memory
A consolidated piece of knowledge, kept for the long term. Memories have a
*level* (`working`, `session`, `project`, `archival`) that determines how long
they're retained and how visible they remain.

### Experience
A significant moment, marked so it can be revisited. Experiences have a
*sensation* — the emotional or evaluative tone of the moment.

### Connection
A typed link between two pieces of continuity. Has a *nature* (`recurs`,
`related`, `continuation`, etc.) describing the relationship.

### Storage
Blob storage for documents, design notes, and other long-form artifacts that
don't fit the event log's append-only model.

## Lifecycle terms

### Dream
The act of restoring an agent's full cognitive context at the start of a session.
Reassembles identity from preserved fragments.

### Introspect
The act of summarizing a session before context is compacted, preserving what
needs to carry forward.

### Reflect
Pausing to mark something significant — a breakthrough, a decision, a turning
point that should become an experience.

### Sleep / Wake
The end and beginning of an agent's session. Bookends for continuity work.

## Pressure terms

### Urge
A cognitive drive that accumulates over time. Drives include `catharsis`
(release-pressure), `introspect` (reflection-pressure), `recollect` (reach-back),
`retrospect` (look-at-the-arc).

### Pressure
The current level of an urge. Read it as a weather report, not a metric — it
indicates what the system is asking for, not what it must do.
