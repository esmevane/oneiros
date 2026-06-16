//! Typed database primitives.
//!
//! Each DB the engine opens has its own type that owns a connection and
//! derefs to `rusqlite::Connection` so existing query code keeps working.
//! All connection opens route through [`Config`] methods so that every
//! pragma is applied from a single source — no more scattered
//! `rusqlite::Connection::open` calls with ad-hoc pragma strings.
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
        Self::open_with(scope.config()).await
    }

    /// Open directly from a `Config`. Underlying primitive — used by
    /// the scope-form above and by paths that have a `Config` but no
    /// scope.
    pub(crate) async fn open_with(config: &Config) -> Result<Self, HostDbError> {
        config.platform().ensure_data_dir()?;
        let connection = config.host_db()?;
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
        Self::open_with(scope.config(), &scope.project().name).await
    }

    /// Open directly from a `Config` + project. Underlying primitive.
    pub(crate) async fn open_with(
        config: &Config,
        project: &ProjectName,
    ) -> Result<Self, EventsDbError> {
        config.platform().ensure_project_dir(project)?;
        // Clone the config with the project name so events_db_path resolves.
        let mut project_config = config.clone();
        project_config.project = project.clone();
        let connection = project_config.open_events_db()?;
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
        Self::open_with(
            scope.config(),
            &scope.project().name,
            &scope.bookmark().name,
        )
        .await
    }

    /// Open directly from config + project + bookmark. Underlying
    /// primitive — used by the scope-form above and by paths that have
    /// a `Config` but no scope.
    pub(crate) async fn open_with(
        config: &Config,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<Self, BookmarkDbError> {
        config.platform().ensure_bookmarks_dir(project)?;
        let mut project_config = config.clone();
        project_config.project = project.clone();
        project_config.bookmark = bookmark.clone();
        let connection = project_config.bookmark_conn()?;
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

    fn test_config() -> (tempfile::TempDir, Config) {
        let dir = tempfile::tempdir().expect("create tempdir");
        let config = Config::builder()
            .data_dir(dir.path().to_path_buf())
            .project(ProjectName::new("test"))
            .output(OutputMode::Json)
            .service(
                ServiceConfig::builder()
                    .address("127.0.0.1:0".parse().unwrap())
                    .build(),
            )
            .build();
        (dir, config)
    }

    #[tokio::test]
    async fn host_db_opens_and_creates_host_db() {
        let (_dir, config) = test_config();
        let _db = HostDb::open_with(&config).await.unwrap();
        assert!(config.platform().host_db_path().exists());
    }

    #[tokio::test]
    async fn events_db_opens_and_creates_project_dir() {
        let (_dir, config) = test_config();
        let project = ProjectName::new("alpha");
        let _db = EventsDb::open_with(&config, &project).await.unwrap();
        assert!(config.platform().project_dir(&project).is_dir());
        assert!(config.platform().events_db_path(&project).exists());
    }

    #[tokio::test]
    async fn bookmark_db_opens_with_events_attached() {
        let (_dir, config) = test_config();
        let project = ProjectName::new("alpha");
        let bookmark = BookmarkName::main();

        // Pre-create events db so ATTACH points at a real file.
        let _events = EventsDb::open_with(&config, &project).await.unwrap();
        let db = BookmarkDb::open_with(&config, &project, &bookmark)
            .await
            .unwrap();

        // Verify the ATTACH worked — events schema should be queryable.
        let count: i64 = db
            .query_row("SELECT count(*) FROM events.sqlite_master", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert!(count >= 0);
    }
}
