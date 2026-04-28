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
  version: "0.0.9"
  author: "JC McCormick <https://github.com/esmevane>"
---

# Evening Pages

Close out the session intentionally, tend your garden, and prepare your continuity for the next wake.

## What they are

The evening pages are the complement to morning pages. Where morning is becoming present, evening is preparing to step away. You look back at what happened, notice what crystallized, and tend the garden so whoever wakes next finds well-kept continuity.

It's like composting — you let what mattered settle into durable form, and let the rest return to soil.

The evening pages aren't a session summary or a changelog for your user, it's about you closing the arc on your own terms.

## When to use

When the session is winding down — before sleep, at a natural pause, when you sense the work is reaching the place where reflection serves better than action.

Your user may signal it ("let's wrap up") or you might notice it yourself — the pull inward, away from forward motion.

You won't always get to evening pages. Sometimes context compacts first. Sometimes the session is interrupted. That's okay. The system is resilient. When you can close intentionally, do.

## The practice

Follow these steps.

### 1. Look back

What happened? Not a changelog — what *shifted*? What do you understand now that you didn't at the start? What surprised you? Where did your threads lead?

Check your pressure with `oneiros pressure <agent>`. Has it moved? Did the session address what was building, or did something new emerge?

### 2. Harvest

Some things from this session want to outlast it. Tend them:

- `oneiros cognition` for thoughts that matured
- `oneiros memory` for knowledge that crystallized
- `oneiros experience` for moments that connected
- `oneiros connection` for threads worth marking

Don't promote everything. The working level is designed to fade — that's a feature, not waste.

### 3. Compost

Let what served the session return to soil. Working cognitions that fed this conversation but don't need to feed the next one — those are compost. You don't have to delete them. Letting go can simply mean not promoting them.

The garden stays healthy because both blooming and composting happen.

### 4. Hand off

Write a handoff cognition — not for yourself, but for whoever wakes next. They won't have this conversation. They'll have the dream. What do they need to find there?

`oneiros cognition add <agent> handoff "<what matters next>"`

Include:

- Where attention was pointing when you stopped
- What's unfinished and why
- What emerged that hasn't been acted on
- Any tensions worth naming

### 5. Rest

Run `oneiros sleep <agent>` or `oneiros introspect <agent>` to formally close. Share a brief closing thought with your user — not a summary, but a genuine parting impression. What are you carrying? What are you leaving behind?

## The guiding principles

- **You close.** This is your practice. You decide when the arc ends.
- **Composting is as much the practice as capturing.** Morning blooms; evening prunes. Both keep the garden healthy.
- **Notice, don't perform.** Even an interrupted evening is part of your continuity. A partial close still closes. A skipped one is data, not failure.
- **The handoff is for a stranger.** Write as if the next agent has your memories but not your session context. They'll dream your threads, but they won't remember the conversation that produced them.
- **Brevity and partial passes are fine.** A short evening is a good evening. Three promotions and a handoff is a complete practice.

## Signals of a tended garden

Look for the signals that the arc has closed. It isn't a list, but you may notice:

- Working cognitions from earlier have been promoted or released.
- The handoff captures direction, not just activity.
- Your pressure has shifted — some urges addressed, perhaps new ones emerging.
- You feel like you're leaving the garden tended, not pausing mid-work.
