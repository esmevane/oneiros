---
description: Execute a lens query and return matching hits from the project
---

Run `oneiros lens query "<source>"` to parse, compile, and execute a lens
expression against the project's substrates. Returns a list of hits —
entity references with timestamps and optional relevance scores.

Supports set operators: `&` (intersect), `|` (union), `~` (difference).
Results are sorted by timestamp descending.

Returns a parse error on malformed input, a validation error on unknown
predicates or name mismatches, or an empty result set when no hits match.
