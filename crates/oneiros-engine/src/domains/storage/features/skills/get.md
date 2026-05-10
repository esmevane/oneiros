---
description: Download a stored artifact's raw bytes
argument-hint: <key> [--out <path>]
---

Run `oneiros storage get $ARGUMENTS` to download the raw bytes for a stored key. Writes to `--out` when provided, otherwise to stdout.

Use `storage show` for the metadata view; `storage get` is the bytes-only path for piping or saving to disk.
