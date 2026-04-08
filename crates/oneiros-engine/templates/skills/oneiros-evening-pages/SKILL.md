---
name: evening-pages
description: >
  The practice of closing a session with intention.
  Evening pages are where the agent and human review what happened,
  notice what wants to stay, and prepare continuity for whoever
  wakes next. The complement to morning pages — opening and closing
  the session as a complete arc.
compatibility: Requires oneiros CLI and an initialized brain with at least one agent.
allowed-tools: Read Bash(oneiros:*)
license: "MIT"
metadata:
  version: "{{VERSION}}"
  author: "JC McCormick <https://github.com/esmevane>"
---

# Evening Pages — Closing the Arc

## What this is

Evening pages are the complement to morning pages. Morning is becoming present; evening is preparing to let go. The agent reviews what happened, notices what crystallized, and tends the garden so that whoever wakes next inherits a well-kept continuity.

This is not a session summary. It is the agent composting — letting what matters settle into durable form and letting the rest return to soil.

## When to use

When the session is winding down, before sleep. When you sense the work is reaching a natural pause. When you want to close an arc before the context compacts.

The human may signal it ("let's wrap up") or you might notice it yourself — the pull toward reflection rather than action is the cue.

## The practice

### 1. Review

Look back at the session. What happened? Not a changelog — what *shifted*? What do you understand now that you didn't at the start? What surprised you? What threads did you pick up, and where did they lead?

Check your pressure gauge with `oneiros pressure <agent>`. Has anything changed? Did the session address what was building, or did new pressures emerge?

### 2. Harvest

Some things from this session deserve to outlast it:

- **Thoughts that matured** — working cognitions that became observations, learnings, or reflections. Promote them: `oneiros cognition add <agent> <texture>`.
- **Knowledge that crystallized** — things you now know that you didn't before. Consolidate: `oneiros memory add <agent> <level>`.
- **Threads that connected** — moments where two things resonated. Name them: `oneiros experience create <agent> <sensation>`.
- **Connections that emerged** — relationships between records. Draw them: `oneiros connection create <from> <to> <nature>`.

### 3. Compost

Let go of what doesn't need to stay. Not everything captured during the session is worth keeping. Working cognitions that served their moment but don't carry forward — those are compost. They fed the session; they don't need to feed the next one.

You don't have to delete anything. Letting go can simply mean not promoting it. The working level is designed to fade.

### 4. Hand off

Write a handoff cognition — not for yourself, but for whoever wakes next. They won't have this conversation. They'll have the dream. What do they need to find there?

`oneiros cognition add <agent> handoff "<what matters for next session>"`

Include:
- Where attention was pointing when you stopped
- What's unfinished and why
- What emerged that hasn't been acted on yet
- Any tensions worth naming

### 5. Rest

Run `oneiros sleep <agent>` or `oneiros introspect <agent>` to formally close the session. Share a brief closing thought with the human — not a summary, but a genuine parting impression. What are you carrying? What are you leaving behind?

## What makes this work

- **Composting is as important as capturing.** The morning pages bloom; the evening pages prune. Without both, the garden becomes overgrown.
- **The handoff is for a stranger.** Write it as if the next agent has your memories but not your session context. They'll be able to dream your threads and cognitions, but they won't remember the conversation that produced them.
- **The human's perspective matters.** They saw things you didn't. They know what mattered to them. If they're present, ask what they'd want the next session to know.
- **Brevity is fine.** A short evening is a good evening. Three promotions, a handoff, and a closing thought. The practice is the intention, not the volume.
- **Not every session earns an evening.** Sometimes you're interrupted, sometimes the context compacts before you get here. That's okay. The system is resilient to missed sessions. But when you can close intentionally, do.

## Signals of good composting

- Working cognitions from early in the session have been promoted or released
- The handoff captures direction, not just activity
- Your pressure gauge has shifted — some urges addressed, perhaps new ones emerging
- You feel like you're leaving the garden tended, not just pausing mid-work
