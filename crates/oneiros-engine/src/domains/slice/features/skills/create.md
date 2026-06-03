---
description: Create a standing lens-filtered view over your project's continuity
argument-hint: "<name> <lens-expr>"
---

Run `oneiros slice create $ARGUMENTS` to create a standing lens-filtered view of the event stream. The slice materializes all events matching the given lens expression retroactively. Use it to narrow focus, prepare a shareable snapshot, or inspect a curated subset of continuity.

The lens expression uses the same syntax as `oneiros lens query`. For example:

- `oneiros slice create gov "agent(governor.process)"` — all events from the governor agent
- `oneiros slice create obs "texture(observation)"` — all observation cognitions

Slices are independent of bookmarks. When you're ready to share a slice, you can snapshot it into a bookmark for transport.
