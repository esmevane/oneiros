---
description: Parse, validate, and plan a lens expression with substrate and type annotations
---

Run `oneiros lens explain "<source>"` to take a lens query expression
through the full parse → validate → plan pipeline. The response carries
the source, its canonical display form, and a tree-rendered plan that
annotates each predicate with the substrate it would route to
(`search-index:agent`, `connections`, `chronicle-walk`, etc.) and the
resolved result type (`entities` or `events`).

Returns a parse error on malformed input, a validation error on unknown
predicates / arity mismatches / arg type mismatches / set operator type
mismatches.
