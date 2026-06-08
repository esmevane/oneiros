# ServerState::from_parts Generates Random Host Secret

**Date:** 2026-06-08
**Status:** Diagnosed, not fixed
**Severity:** Latent — no known test failures triggered by this

## Summary

`ServerState::from_parts` is a constructor used to create a temporary `ServerState` from individual components (config, canons, bridge, mailbox). It is currently called in one place: `handle_push_bookmark` in the bridge sync handler, to pass a `ServerState` into `BookmarkService::collect_from_peer_link`.

The constructor generates a **fresh random host_secret**:

```rust
pub(crate) fn from_parts(
    config: Config,
    canons: CanonIndex,
    bridge: Bridge,
    mailbox: Mailbox,
) -> Self {
    Self {
        config,
        canons,
        bridge,
        api: Arc::new(OnceLock::new()),
        mailbox,
        host_secret: iroh::SecretKey::generate(),  // BUG: should accept or clone the real secret
    }
}
```

The real `ServerState` (created by `ServerState::bind`) receives the host's persistent secret key, which is used for host token generation and verification. The `from_parts` version discards this and creates a random key.

## Impact (currently contained)

The temporary `ServerState` from `from_parts` is used only within `collect_from_peer_link`, which:
- Uses `state.config()` for DB access
- Uses `state.canons()` for chronicle resolution
- Uses `state.bridge()` for network operations
- Uses `state.mailbox()` for event import

None of these paths call `state.ticket_verifier()` or perform host token verification. So the bad secret is **never used for auth**.

**However**, this is a time bomb. If any code path in the future starts calling `ticket_verifier()` on a `ServerState` created via `from_parts`, host token verification will silently fail — all host tokens validated against the random secret will be rejected.

## Discovery context

Found while investigating the `ticket_rotation` test failure. The `from_parts` path was examined as a possible cause because it's called during push operations. It turned out to be unrelated to the test failure (revoked tickets exit `handle_push_bookmark` before reaching `from_parts`), but the random secret generation was flagged as a latent bug.

## Possible remediations

### A. Pass host_secret as parameter

Add `host_secret: iroh::SecretKey` as a parameter to `from_parts`:

```rust
pub(crate) fn from_parts(
    config: Config,
    canons: CanonIndex,
    bridge: Bridge,
    mailbox: Mailbox,
    host_secret: iroh::SecretKey,  // caller provides the real secret
) -> Self { ... }
```

And update the call site in `handle_push_bookmark` to thread through `self.host_secret` (which the `SyncHandler` would need to store).

**Pros:** Correct. Caller controls the secret.
**Cons:** Adds a parameter. `SyncHandler` needs to store the secret.

### B. Store host_secret in Config

Move the host secret into `Config` (or a sub-field), making it available wherever config is available. `from_parts` already receives `Config`.

**Pros:** No new parameters. Secret lives in a single canonical place.
**Cons:** Config becomes security-sensitive. Needs careful serialization exclusion. Broader refactor.

### C. Remove from_parts entirely

Refactor `collect_from_peer_link` to not need a full `ServerState`. It only uses `canons()`, `config()`, `bridge()`, and `mailbox()` — all of which are available individually.

**Pros:** Eliminates the problematic constructor. Cleaner dependency shape.
**Cons:** Requires refactoring `collect_from_peer_link`'s signature and all internal call sites.

## Related

- `notes/token-collision-bug.md` — the other bug found in the same session
- `crates/oneiros-engine/src/http/state.rs` — ServerState::from_parts
- `crates/oneiros-engine/src/domains/bridge/service.rs` — call site in handle_push_bookmark
