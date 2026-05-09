---
description: Follow a remote bookmark
argument-hint: <uri> --name <name>
---

Run `oneiros bookmark follow $ARGUMENTS` to subscribe to a remote bookmark, identified by its URI, under the local `--name`.

Following creates a follow record but does not move events on its own — use `bookmark collect` to reconcile.
