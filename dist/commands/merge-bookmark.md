---
description: Merge a bookmark into the active bookmark
argument-hint: <source>
---

Run `oneiros bookmark merge $ARGUMENTS` to merge the source bookmark's changes into the active bookmark. CRDT resolution handles conflicts automatically.

After merging, the active bookmark contains the combined state of both timelines. The source bookmark remains unchanged.
