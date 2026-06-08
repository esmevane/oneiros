# Deterministic Token Collision

**Date:** 2026-06-08
**Status:** Diagnosed, not fixed
**Affected test:** `tests::workflows::remotes::ticket_rotation` (ignored)

## Summary

`Token::issue` is a pure function of claims: `(project_id, tenant_id, actor_id) → base32(postcard(claims))`. Any two tickets issued for the same project by the same actor produce **identical token strings**.

The tickets table enforces `token TEXT NOT NULL UNIQUE`, and the store uses `INSERT OR REPLACE`. When a second ticket is issued with the same token (e.g., `remote share` after `project create`), it **replaces** the first ticket's row entirely. The original ticket is gone.

When the replacement ticket is revoked, all auth using that token breaks — including the project's CLI token (written to disk by `project create`).

## Impact

Any flow that creates multiple tickets for the same (project, actor, tenant):
- `project create` → `remote share` → `ticket revoke`: kills CLI access
- Two consecutive `remote share` calls: second silently replaces first
- `bookmark share` + `remote share` for same project: collision

The `push_with_revoked_ticket_is_denied` test passes only because the token collision means the revoked ticket IS the only ticket — the project ticket was replaced, so revocation works "correctly" by accident.

## Exploration

### Attempted: Remove UNIQUE + ORDER BY unrevoked-first

Removing the UNIQUE constraint and using `ORDER BY revoked_at IS NULL DESC` in `get_by_token` lets multiple tickets share a token and prefers valid ones. This fixes `ticket_rotation` but **breaks `push_with_revoked_ticket_is_denied`** — the push can't distinguish "revoked share ticket" from "valid project ticket" when they share a token. Revocation becomes meaningless.

### Attempted: ORDER BY alone

Doesn't help because UNIQUE constraint already limits results to one row.

## Possible remediations

### A. Make tokens unique per-ticket (preferred)

Add a nonce or include the ticket's target/permissions in the token claims. Example:

```rust
let claims = TokenClaims::builder()
    .project_id(project.id)
    .tenant_id(actor.tenant_id)
    .actor_id(actor_id)
    .nonce(Nonce::new())  // makes each token unique
    .build();
```

**Pros:** Cleanest solution. Tokens become proper ticket identifiers.
**Cons:** Breaking change — invalidates all existing tokens on disk and in the DB. Needs migration, token versioning, backward compatibility window.

### B. Reuse existing tickets

When `TicketService::issue` would create a ticket with claims matching an existing valid ticket, return the existing one instead. `INSERT OR IGNORE` instead of `INSERT OR REPLACE`.

**Pros:** Non-breaking. Simple code change.
**Cons:** Loses the ability to issue multiple tickets with different scopes for the same project. `remote share` can't create a read-only ticket if a full-access ticket already exists.

### C. Auth by link, not just token

Change auth lookup to match on `(token, target)` instead of just `token`. The HTTP auth header would still carry only the token, but the lookup could filter by target context.

**Pros:** Doesn't require token format change.
**Cons:** Ambiguous which target to use for auth. Complex. Doesn't match the current "token is authorization" model.

## Related

- `notes/serverstate-from-parts-secret.md` — the other bug found in the same session
- `crates/oneiros-engine/src/values/token.rs` — Token::issue
- `crates/oneiros-engine/src/domains/ticket/store.rs` — schema + INSERT OR REPLACE
- `crates/oneiros-engine/src/tests/workflows/remotes.rs` — affected tests
