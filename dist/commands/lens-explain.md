---
description: Parse, validate, and compile a lens expression, showing the intermediate representation
---

Run `oneiros lens explain "<source>"` to take a lens query expression
through the full parse → validate → compile pipeline. The response carries
the source, its canonical display form, and the compiled IR showing each
operation as a numbered slot — reads from substrates, set operators
referencing earlier slots.

Returns a parse error on malformed input, a validation error on unknown
predicates / arity mismatches / arg type mismatches / set operator type
mismatches.
