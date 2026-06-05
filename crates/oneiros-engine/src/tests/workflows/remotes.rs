//! System workflow — remote distribution.
//!
//! Remotes are hosts you can push bookmarks to and pull bookmarks from,
//! authorized by project-scoped capability tickets. These tests characterize
//! the full remote workflow: ticket issuance, remote management, listing,
//! pulling, and pushing bookmarks.
//!
//! Each test exercises a single scenario in the workflow. They build on
//! the existing test harness patterns (TestApp, Client) and follow the
//! same structural idioms as tests/workflows/host.rs.

use crate::tests::harness::TestApp;
use crate::*;

// ─── Remote lifecycle ───────────────────────────────────────────────

/// Adding a remote with a valid ticket persists it and makes it listable.
#[tokio::test]
async fn add_remote_with_valid_ticket() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once the Remote domain exists.
    // It characterizes:
    // 1. Issue a read+write ticket on the remote host
    // 2. Parse the ticket URI on the local host
    // 3. `remote add` connects to the peer and validates the ticket
    // 4. The remote appears in `remote list`
    Ok(())
}

/// Adding a remote with an invalid ticket is rejected.
#[tokio::test]
async fn add_remote_with_invalid_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once the Remote domain exists.
    // It characterizes:
    // 1. Attempt `remote add` with a bogus URI
    // 2. The error surfaces as a rejected remote addition
    Ok(())
}

/// Removing a remote drops it from the list.
#[tokio::test]
async fn remove_remote_drops_from_list() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once the Remote domain exists.
    // It characterizes:
    // 1. Add a remote
    // 2. Remove it
    // 3. `remote list` no longer shows it
    Ok(())
}

// ─── Remote bookmarks ───────────────────────────────────────────────

/// Listing bookmarks on a remote returns the remote host's bookmark names.
#[tokio::test]
async fn list_remote_bookmarks() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgeListBookmarks exists.
    // It characterizes:
    // 1. Create bookmarks on the remote host
    // 2. Add the remote on the local host
    // 3. `remote bookmarks` returns the remote's bookmark names
    Ok(())
}

// ─── Push ───────────────────────────────────────────────────────────

/// Pushing a bookmark to a remote creates it on the remote host.
#[tokio::test]
async fn push_bookmark_to_remote() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgePushBookmark exists.
    // It characterizes:
    // 1. Create events on the local host
    // 2. Create a bookmark containing those events
    // 3. `bookmark push <remote> <name>`
    // 4. The bookmark appears on the remote host with the pushed events
    Ok(())
}

/// Pushing with --as renames the bookmark on the remote.
#[tokio::test]
async fn push_bookmark_with_as_renames() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgePushBookmark exists.
    // It characterizes:
    // 1. Push bookmark "my-change" with --as "feature-x"
    // 2. Remote has bookmark "feature-x", not "my-change"
    Ok(())
}

/// Pushing without Write permission is rejected.
#[tokio::test]
async fn push_with_read_only_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgePushBookmark exists.
    // It characterizes:
    // 1. Remote issues a read-only ticket
    // 2. Local adds the remote with that ticket
    // 3. `bookmark push` is rejected with permission error
    Ok(())
}

// ─── Pull ───────────────────────────────────────────────────────────

/// Pulling a bookmark from a remote creates a local copy.
#[tokio::test]
async fn pull_bookmark_from_remote() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgePullBookmark exists.
    // It characterizes:
    // 1. Create events and a bookmark on the remote host
    // 2. Add the remote on the local host
    // 3. `bookmark pull <remote> <name> --as <local-name>`
    // 4. Local host has the bookmark with the same events
    Ok(())
}

/// Pulling with a read-only ticket succeeds.
#[tokio::test]
async fn pull_with_read_only_ticket_succeeds() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgePullBookmark exists.
    // It characterizes:
    // 1. Remote issues a read-only ticket
    // 2. Local adds the remote with that ticket
    // 3. `bookmark pull` succeeds (read is sufficient)
    Ok(())
}

/// Pulling without any read permission is rejected.
#[tokio::test]
async fn pull_with_write_only_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once BridgePullBookmark exists.
    // It characterizes:
    // 1. Remote issues a write-only ticket
    // 2. Local adds the remote with that ticket
    // 3. `bookmark pull` is rejected with permission error
    Ok(())
}

/// Push-pull roundtrip: push a bookmark, pull it back under a different name.
#[tokio::test]
async fn push_pull_roundtrip() -> Result<(), Box<dyn core::error::Error>> {
    // NOTE: This test will compile once both push and pull exist.
    // It characterizes:
    // 1. Push a bookmark to the remote
    // 2. Pull it back under a different name
    // 3. The pulled bookmark has the same events as the pushed one
    Ok(())
}
