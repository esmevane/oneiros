---
name: writing-tests
description: How tests are shaped in this repo. Use when writing, extending, or refactoring tests, or when a test file starts growing helper functions.
---

# Writing tests

A good test is **self-contained**: its setup, its action, and every one of its assertions live inside the test body, so a reader understands the system under test from that one function, and can change or delete it without touching anything else.

## The shape

- **Setup inline.** Build the `TestApp` in the test itself. Express fixtures as data: the CLI commands a user would type, held in a `&[&str]` and run with `app.commands(FIXTURES).await?`. A test that needs extra fixtures adds them right there, with another `app.commands(&[...])`.
- **Expected values literal.** Write the expected result out in the `assert_eq!` (`["Tracking issue #317 here"]`), so the assertion states the claim on its own. Assert on contents, not just on "didn't error".
- **One claim per test**, named for the behaviour it pins down (`cognition_list_query_with_hash_filters`).

Reference example: `crates/oneiros-engine/src/tests/workflows/query_encoding.rs`.

## When you feel the pull toward a helper

Repetition across tests is a **signal**, not a refactor target for the test file. A pile of file-local helpers prefacing the tests is a sign of hidden complexity in the setup, the test tooling, or the library's own ergonomics. Moving that complexity into free functions hides the hint from the next reader and makes every assertion a trip through indirection.

So take the signal seriously and fix it at its source:

1. **Library ergonomics**: if the production API is awkward to drive, that awkwardness is the finding. Raise it.
2. **Test harness**: if the tests do need a capability, it belongs in the harness (`crates/oneiros-engine/src/tests/harness.rs`), where every test can use it and it is maintained as tooling: `TestApp` methods (`command`, `commands`), `app.testable()` for decoded results ready for assertions, `app.client()` for typed requests.
3. **Inline it**: if neither applies, the repetition is the honest cost of tests that each say everything. Keep it.

A helper earns a place in the harness only after the tests are written self-contained and the shared need is plain to see.
