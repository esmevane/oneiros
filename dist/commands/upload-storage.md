---
description: Archive a file for later
argument-hint: "<key> <file> [--description <text>]"
---

Run `oneiros storage set $ARGUMENTS` to store a file's contents under a named key. The content is deduplicated — storing the same content under different keys shares the underlying blob.

Use `--description` to annotate what the stored content represents.
