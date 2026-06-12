---
description: Parse a lens expression and echo its round-trip display form
---

Run `oneiros lens parse "<source>"` to parse a lens query expression. The
response carries the original source plus the parser's canonical
[`Lens::Display`] form — useful for confirming precedence, whitespace
handling, and quoting before you commit to a query.

Returns a parse error with byte spans on malformed input. Does not
validate predicates against the registry; use `oneiros lens explain` for
that.
