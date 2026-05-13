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
pub(crate) enum HostDbError {
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum EventsDbError {
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BookmarkDbError {
    #[error(transparent)]
    Connection(#[from] rusqlite::Error),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

// ─────────────────────────────────────────────────────────────────────
// HostDb — host.db (host-wide projections: projects, bookmarks,
// chronicle, tickets, tenants, actors, peers, follows).
// ─────────────────────────────────────────────────────────────────────

pub(crate) struct HostDb {
    connection: rusqlite::Connection,
}

impl HostDb {
    /// Open from any scope tier that carries host info. The primary
    /// public entry — services and actors at any tier reach the
    /// host db this way.
    pub(crate) async fn open<S: HasHost>(scope: &S) -> Result<Self, HostDbError> {
        Self::open_with(&scope.config().platform()).await
    }

    /// Open directly from a `Platform`. Underlying primitive — used by
    /// the scope-form above and by tests / hydration paths that don't
    /// have a scope handy.
    pub(crate) async fn open_with(platform: &Platform) -> Result<Self, HostDbError> {
        platform.ensure_data_dir()?;
        let connection = rusqlite::Connection::open(platform.host_db_path())?;
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
// EventsDb — append-only event log per project.
// ─────────────────────────────────────────────────────────────────────

pub(crate) struct EventsDb {
    connection: rusqlite::Connection,
}

impl EventsDb {
    /// Open from any scope tier that carries project info.
    pub(crate) async fn open<S: HasProject>(scope: &S) -> Result<Self, EventsDbError> {
        Self::open_with(&scope.config().platform(), &scope.project().name).await
    }

    /// Open directly from a `Platform` + project. Underlying primitive.
    pub(crate) async fn open_with(
        platform: &Platform,
        project: &ProjectName,
    ) -> Result<Self, EventsDbError> {
        platform.ensure_project_dir(project)?;
        let connection = rusqlite::Connection::open(platform.events_db_path(project))?;
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
// BookmarkDb — per-bookmark projection database with the project's
// events DB ATTACHed at `events`. Unqualified table names resolve to
// the bookmark DB; event-log queries use the `events` schema.
// ─────────────────────────────────────────────────────────────────────

pub(crate) struct BookmarkDb {
    connection: rusqlite::Connection,
}

impl BookmarkDb {
    /// Open from a bookmark-tier scope.
    pub(crate) async fn open<S: HasBookmark>(scope: &S) -> Result<Self, BookmarkDbError> {
        let config = scope.config();
        Self::open_with(
            &config.platform(),
            &scope.project().name,
            &scope.bookmark().name,
            config.database.limit_attached,
        )
        .await
    }

    /// Open directly from a `Platform` + project + bookmark. Underlying
    /// primitive — used by the scope-form above and by tests.
    pub(crate) async fn open_with(
        platform: &Platform,
        project: &ProjectName,
        bookmark: &BookmarkName,
        limit_attached: u32,
    ) -> Result<Self, BookmarkDbError> {
        platform.ensure_bookmarks_dir(project)?;
        let connection = rusqlite::Connection::open(platform.bookmark_db_path(project, bookmark))?;
        connection.pragma_update(None, "journal_mode", "wal")?;
        connection.pragma_update(None, "limit_attached", limit_attached.to_string())?;
        connection.execute_batch(&format!(
            "ATTACH DATABASE '{}' AS events",
            platform.events_db_path(project).display(),
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
    async fn host_db_opens_and_creates_host_db() {
        let (_dir, platform) = test_platform();
        let _db = HostDb::open_with(&platform).await.unwrap();
        assert!(platform.host_db_path().exists());
    }

    #[tokio::test]
    async fn events_db_opens_and_creates_project_dir() {
        let (_dir, platform) = test_platform();
        let project = ProjectName::new("alpha");
        let _db = EventsDb::open_with(&platform, &project).await.unwrap();
        assert!(platform.project_dir(&project).is_dir());
        assert!(platform.events_db_path(&project).exists());
    }

    #[tokio::test]
    async fn bookmark_db_opens_with_events_attached() {
        let (_dir, platform) = test_platform();
        let project = ProjectName::new("alpha");
        let bookmark = BookmarkName::main();

        // Pre-create events db so ATTACH points at a real file.
        let _events = EventsDb::open_with(&platform, &project).await.unwrap();
        let db = BookmarkDb::open_with(&platform, &project, &bookmark, 125)
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
