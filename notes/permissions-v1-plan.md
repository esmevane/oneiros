# Permissions V1 — Design & Implementation Plan

## Motivation

Tickets in oneiros are designed as **object capabilities** (ocaps) — generalized permission tokens that apply broadly across the system, not just to bookmark distribution. The current implicit behavior (all tickets grant read access to their target) needs to become explicit so we can support push/submit (write access) and future capability types.

The CCS Arc 2 analysis originally proposed adding a `TicketScope {Pull, Submit}` field to `Link`. That design was rejected because:

1. **`Link` encodes resource identity + transport token** — it should not encode permissions
2. **Permissions belong on `Ticket`** — following the ocap model (matching emblem's prior art)
3. **`Link` is a postcard-serialized wire format** — changing it breaks URIs and serialization compatibility

## Design

### New types

**`PermissionOp`** (`src/values/permission_op.rs`):
```rust
pub(crate) enum PermissionOp {
    Read,   // pull, diff, fetch (existing behavior)
    Write,  // submit, push (new behavior)
}
```

**`Permission`** (`src/values/permission.rs`) — versioned wrapper:
```rust
versioned! {
    pub(crate) enum Permission {
        V1 => {
            pub(crate) operation: PermissionOp,
        }
        V0 => {}  // empty — implicit read
    }
}
```

### Serde behavior

The `versioned!` macro uses `#[serde(untagged)]` on the wrapper enum and `#[serde(deny_unknown_fields)]` on each variant struct. This means:

| JSON input | Deserializes as |
|---|---|
| `{}` | `Permission::V0(PermissionV0{})` |
| `{"operation": "read"}` | `Permission::V1(PermissionV1{operation: Read})` |
| `{"operation": "write"}` | `Permission::V1(PermissionV1{operation: Write})` |

On `Ticket`, the `permissions` field has:
- `#[serde(default, skip_serializing_if = "Vec::is_empty")]`
- `#[builder(default)]`

So existing tickets (no `permissions` key in JSON) deserialize to `vec![]` — implicit read access. New tickets serialize without the field when empty.

### Ticket model change

```rust
// Only new field added:
#[builder(default)]
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub(crate) permissions: Vec<Permission>,
```

No migration needed — `#[serde(default)]` handles absent keys. The `TicketStore` (SQLite projection) will need a column addition when we want to persist permissions in the projection, but the event log already carries them via `TicketIssued`.

## Implementation layers

### Layer 1: Types + wiring ✅

- [x] Create `src/values/permission_op.rs`
- [x] Create `src/values/permission.rs`
- [x] Register in `src/values/mod.rs`
- [x] Add `permissions` field to `Ticket`
- [x] Fix `TicketRepo` struct literal (adds `permissions: vec![]`)
- [x] Update `TicketService::issue` to accept optional permissions
- [x] Add `--permission` flag to `ticket issue` CLI
- [x] Tests: roundtrip serde (V0 → {}, V1 read/write, absent permissions)
- [x] Tests: `Ticket::can()` (empty=read, explicit read/write, read+write, V0 in vec)
- [x] All 804 tests pass

### Layer 2: Permission checks ✅

- [x] Add `Ticket::can(&self, op: PermissionOp) -> bool` method
- [x] Wire into Bridge handler (`validate_ticket` checks `can(Read)`)
- [x] Add `DenyReason::InsufficientPermissions` to bridge errors
- [x] Tests: `can()` covers all permission matrices

### Layer 3: Submit ticket creation (new operations)

- [ ] `TicketService::issue` accepts `permissions: Vec<Permission>`
- [ ] `ticket issue --permission write --actor alice --project my-project`
- [ ] Submit-scoped tickets target `Ref::project(project_id)` (not a bookmark)
- [ ] Tests: submit ticket carries `[V1 { operation: Write }]`

### Layer 4: BridgeSubmit handler (CCS Arc 2)

- [ ] `BridgeSubmit` protocol message
- [ ] Handler validates: `ticket.can(Write)`, ticket not revoked/expired/exhausted
- [ ] On accept: creates bookmark, calls ad-hoc collect
- [ ] Tests: write-scoped ticket passes, read-only ticket rejected

## Future growth

When resource-qualified permissions are needed (V2+):

```rust
versioned! {
    pub(crate) enum Permission {
        V2 => {
            pub(crate) resource: Resource,  // e.g. Resource::Bookmark(id)
            pub(crate) operation: PermissionOp,
        }
        V1 => {
            pub(crate) operation: PermissionOp,
        }
        V0 => {}
    }
}
```

Existing V1 tickets continue to work — their permission applies to the ticket's `link.target`. V2 tickets can narrow permissions to specific resources.

## Open questions

- **TicketStore column:** Do we add a `permissions` column to the SQLite projection now, or defer until we need to query by permission? The event log carries the data already.
- **`ticket issue` default:** Should `ticket issue` (no `--permission`) default to `[Read]` explicitly or an empty vec? Empty vec is backward compatible but less explicit.
- **`bookmark share` permissions:** Currently creates a ticket via `TicketService::issue` targeting a bookmark. Should it explicitly set `[Read]` or leave empty?
