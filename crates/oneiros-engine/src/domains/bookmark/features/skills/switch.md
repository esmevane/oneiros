---
description: Switch to a different bookmark
argument-hint: <name>
---

Run `oneiros bookmark switch $ARGUMENTS` to change the active bookmark. This rebuilds the query layer to reflect the target bookmark's state.

After switching, reads will return data from the target bookmark's timeline. Events emitted while on this bookmark are recorded in its chronicle.
