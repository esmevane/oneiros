//! Typed database primitives.
//!
//! Each DB the engine opens has its own type that owns a connection,
//! handles its own pragmas, and reports its own errors. Types deref to
//! `rusqlite::Connection` so existing query code keeps working while
//! the seams migrate.
//!
//! `open` methods are `async fn` even though their bodies are sync —
//! the signature is the migration seam for sqlx.

use std::ops::Deref;

use crate::*;

// ─────────────────────────────────────────────────────────────────────
// Errors
// ─────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum HostDbError {
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

#[derive(Debug, thiserror::Error)]
pub enum EventsDbError {
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

#[derive(Debug, thiserror::Error)]
pub enum BookmarkDbError {
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

// ─────────────────────────────────────────────────────────────────────
// HostDb — system.db (host-wide projections: brains, bookmarks,
// chronicle, tickets, tenants, actors, peers, follows).
// ─────────────────────────────────────────────────────────────────────

pub struct HostDb {
    connection: rusqlite::Connection,
}

impl HostDb {
    pub async fn open(platform: &Platform) -> Result<Self, HostDbError> {
        platform.ensure_data_dir()?;
        let connection = rusqlite::Connection::open(platform.system_db_path())?;
        connection.pragma_update(None, "journal_mode", "wal")?;
        Ok(Self { connection })
    }
}

impl Deref for HostDb {
    type Target = rusqlite::Connection;
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

// ─────────────────────────────────────────────────────────────────────
// EventsDb — append-only event log per brain.
// ─────────────────────────────────────────────────────────────────────

pub struct EventsDb {
    connection: rusqlite::Connection,
}

impl EventsDb {
    pub async fn open(platform: &Platform, brain: &BrainName) -> Result<Self, EventsDbError> {
        platform.ensure_brain_dir(brain)?;
        let connection = rusqlite::Connection::open(platform.events_db_path(brain))?;
        connection.pragma_update(None, "journal_mode", "wal")?;
        Ok(Self { connection })
    }
}

impl Deref for EventsDb {
    type Target = rusqlite::Connection;
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

// ─────────────────────────────────────────────────────────────────────
// BookmarkDb — per-bookmark projection database with the brain's
// events DB ATTACHed at `events`. Unqualified table names resolve to
// the bookmark DB; event-log queries use the `events` schema.
// ─────────────────────────────────────────────────────────────────────

pub struct BookmarkDb {
    connection: rusqlite::Connection,
}

impl BookmarkDb {
    pub async fn open(
        platform: &Platform,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<Self, BookmarkDbError> {
        platform.ensure_bookmarks_dir(brain)?;
        let connection = rusqlite::Connection::open(platform.bookmark_db_path(brain, bookmark))?;
        connection.pragma_update(None, "journal_mode", "wal")?;
        connection.pragma_update(None, "limit_attached", "125")?;
        connection.execute_batch(&format!(
            "ATTACH DATABASE '{}' AS events",
            platform.events_db_path(brain).display(),
        ))?;
        Ok(Self { connection })
    }
}

impl Deref for BookmarkDb {
    type Target = rusqlite::Connection;
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_platform() -> (tempfile::TempDir, Platform) {
        let dir = tempfile::tempdir().unwrap();
        let platform = Platform::new(dir.path());
        (dir, platform)
    }

    #[tokio::test]
    async fn host_db_opens_and_creates_system_db() {
        let (_dir, platform) = test_platform();
        let _db = HostDb::open(&platform).await.unwrap();
        assert!(platform.system_db_path().exists());
    }

    #[tokio::test]
    async fn events_db_opens_and_creates_brain_dir() {
        let (_dir, platform) = test_platform();
        let brain = BrainName::new("alpha");
        let _db = EventsDb::open(&platform, &brain).await.unwrap();
        assert!(platform.brain_dir(&brain).is_dir());
        assert!(platform.events_db_path(&brain).exists());
    }

    #[tokio::test]
    async fn bookmark_db_opens_with_events_attached() {
        let (_dir, platform) = test_platform();
        let brain = BrainName::new("alpha");
        let bookmark = BookmarkName::main();

        // Pre-create events db so ATTACH points at a real file.
        let _events = EventsDb::open(&platform, &brain).await.unwrap();
        let db = BookmarkDb::open(&platform, &brain, &bookmark)
            .await
            .unwrap();

        // Attached schema is queryable.
        let count: i64 = db
            .query_row("select count(*) from events.sqlite_master", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert!(count >= 0);
    }
}
