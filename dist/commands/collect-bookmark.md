---
description: Collect events into a bookmark from a follow source or remote
argument-hint: "<name> [--from <remote-name>] [--as <local-name>]"
---

Run `oneiros bookmark collect $ARGUMENTS` to pull events from a follow source or a peer host into a local bookmark. Collection uses the chronicle Merkle diff — only events your project doesn't already have are transferred.

Collect from a followed source:
```bash
oneiros bookmark collect my-bookmark
```

Collect directly from a peer host (no follow required):
```bash
oneiros bookmark collect their-feature --from dreamforge
oneiros bookmark collect their-feature --from dreamforge --as my-copy
```
