# Remote Distribution — Design & Implementation Plan

## Motivation

The CCS Arc 2 charter and subsequent design discussions have converged on a **remote** model for bookmark distribution. A remote is a host you can push bookmarks to and pull bookmarks from, authorized by a single project-scoped capability ticket. This replaces the earlier bookmark-scoped `BridgeSubmit` design with a more general, git-remote-inspired model.

## Design

### The ticket model

A single ticket grants access to a **project** on a remote host:

```
dreamforge$ oneiros ticket issue --permission read,write --project my-project
→ oneiros://dreamforge/link:<project-capability>
```

| Permission | Operations |
|---|---|
| `Read` | List bookmarks, pull any bookmark's data |
| `Write` | Push bookmarks, update/overwrite |

The ticket is a bearer capability — anyone who holds it can use it. Validation checks the ticket's validity (not revoked/expired/exhausted) and permissions at connection time.

### Remote domain

```rust
Remote {
    name: RemoteName,           // "dreamforge"
    address: PeerAddress,       // where to connect
    ticket: Link,               // project-scoped capability
    project: ProjectName,       // which project on the remote
}
```

Stored in the host DB with event-sourced lifecycle (`RemoteAdded`, `RemoteRemoved`).

### Bridge protocol extensions

| Operation | Direction | Permission | What it does |
|---|---|---|---|
| `BridgeListBookmarks` | client → server | `Read` | Returns all bookmark names for the project |
| `BridgePullBookmark` | client → server | `Read` | Full fetch of a named bookmark's events (no Merkle diff needed for first pull) |
| `BridgePushBookmark` | client → server | `Write` | Push a bookmark to the remote (renamed from `BridgeSubmit`) |

All three carry the project-scoped ticket for authorization.

### CLI surface

```
# Remote management
oneiros remote add <name> <ticket-uri>
oneiros remote list
oneiros remote remove <name>

# Listing remote bookmarks
oneiros remote bookmarks <name>

# Pulling from a remote
oneiros bookmark pull <remote> <remote-bookmark-name> --as <local-name>

# Pushing to a remote
oneiros bookmark push <remote> <local-bookmark-name> --as <remote-name>
```

## Workflows

### 1. Remote addition + validation

```
dreamforge$ oneiros ticket issue --permission read,write --project my-project
→ oneiros://dreamforge/link:<capability>

alice$ oneiros remote add dreamforge oneiros://dreamforge/link:<capability>
```

On `remote add`:
1. Parse URI → PeerAddress + Link
2. Bridge connect to peer
3. Send `BridgeListBookmarks` with the ticket to validate it
4. On success, persist remote record

### 2. List remote bookmarks

```
alice$ oneiros remote bookmarks dreamforge
→ main
→ feature-x
→ bugfix-y
```

Sends `BridgeListBookmarks` with the stored ticket. Returns bookmark names.

### 3. Pull from remote

```
alice$ oneiros bookmark pull dreamforge feature-x --as my-copy
```

1. Look up remote by name → get address + ticket
2. Send `BridgePullBookmark { ticket, bookmark_name }`
3. Remote handler validates ticket (Read), fetches all events for the bookmark, returns them
4. Create local bookmark `my-copy`, import events via existing `collect_from_peer_link`-style flow

### 4. Push to remote

```
alice$ oneiros bookmark push dreamforge my-change --as feature-x
```

1. Look up remote by name → get address + ticket
2. Share the local bookmark to get a peer link
3. Send `BridgePushBookmark { ticket, bookmark, bookmark_name }`
4. Remote handler validates ticket (Write), creates/updates bookmark, pulls data
5. Return acceptance/rejection

### 5. Remote removal

```
alice$ oneiros remote remove dreamforge
```

Deletes the stored remote record. No bridge communication needed.

## Implementation layers

### Layer 1: Bridge protocol (types only)

- [ ] Add `BridgeListBookmarks { ticket: Link, project: ProjectName }` request
- [ ] Add `BridgeListBookmarksResponse { bookmarks: Vec<BookmarkName> }` response
- [ ] Add `BridgePullBookmark { ticket: Link, bookmark_name: BookmarkName }` request
- [ ] Add `BridgePullBookmarkResponse { events: Vec<StoredEvent> }` response
- [ ] Rename `BridgeSubmit` → `BridgePushBookmark` (keep fields: ticket, bookmark, bookmark_name)
- [ ] Add variants to `BridgeRequest` and `BridgeResponse` enums

### Layer 2: Remote domain

- [ ] `Remote` model (`name, address, ticket, project`)
- [ ] `RemoteStore` (SQLite migration + CRUD)
- [ ] `RemoteEvents` (`RemoteAdded`, `RemoteRemoved`)
- [ ] `RemoteRepo` (query methods)
- [ ] `RemoteService` (`add`, `remove`, `list`)
- [ ] `RemoteCommands` CLI (`add`, `list`, `remove`)
- [ ] `RemoteRouter` HTTP routes
- [ ] Skill docs

### Layer 3: Bridge handlers

- [ ] `SyncHandler::handle_list_bookmarks` — validate Read, query canon for bookmark names, return them
- [ ] `SyncHandler::handle_pull_bookmark` — validate Read, fetch all events from event log, return them
- [ ] Rename `handle_submit` → `handle_push_bookmark`
- [ ] Wire new variants in `accept` dispatch

### Layer 4: Bookmark push/pull

- [ ] `bookmark pull <remote> <name> --as <local-name>` CLI + service
- [ ] `bookmark push <remote> <name> --as <remote-name>` CLI + service (adapt existing submit)
- [ ] `remote bookmarks <name>` CLI + service

### Layer 5: Workflow tests

- [ ] `src/tests/workflows/remotes.rs` — characterization tests for all workflows
