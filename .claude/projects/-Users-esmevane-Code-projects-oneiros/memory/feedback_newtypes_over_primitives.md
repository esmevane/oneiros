---
name: Newtypes over primitives
description: Esmevane prefers newtypes over primitive types in all cases — Source not String, domain IDs not raw strings
type: feedback
---

Always use newtypes and value types instead of primitives. If a newtype exists, use it. If it doesn't, consider creating one.

**Why:** The newtype IS the documentation. `Source` in a type signature communicates intent differently than `String`. The engine has value types (Source, domain IDs, etc.) that aren't being leveraged — this is a design smell. It's the same principle as naming-as-design: the type name shapes how the code reader understands the system.

**How to apply:** When touching code that uses primitives where newtypes exist, prefer the newtype. When designing new types, default to newtypes over String/i64/etc. Don't do a sweeping refactor — correct as you go, especially when the change is in the neighborhood of current work.
