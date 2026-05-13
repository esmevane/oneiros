//! Migration workflow — pre-rename layouts come forward intact.
//!
//! The contract: drop a pre-rename data-dir on disk (system.db with
//! `brains` table, bookmark/follow/ticket tables with old column names,
//! per-project events.db with `brain-created` / `init-system` events
//! and `brain_name` fields), boot a `TestApp` against that data-dir,
//! and the production migration hook on server boot brings everything
//! forward. The test then makes typed-client assertions through the
//! same surface a real client uses.
//!
//! If this test passes, the rename survives file moves for an existing
//! user. If it fails, the migration regressed — bisect, don't release.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn pre_rename_data_dir_boots_into_current_layout() -> Result<(), Box<dyn core::error::Error>>
{
    let dir = tempfile::tempdir().expect("create tempdir");
    fixture::stage_pre_rename(dir.path())?;

    let app = TestApp::with_data_dir(dir).await?;
    let data_dir = app.config().data_dir.clone();

    // File rename landed.
    assert!(
        data_dir.join("host.db").exists(),
        "host.db should exist post-migration"
    );
    assert!(
        !data_dir.join("system.db").exists(),
        "system.db should be gone post-migration"
    );

    // Schema renames in host.db landed.
    let host = rusqlite::Connection::open(data_dir.join("host.db"))?;
    assert!(
        fixture::table_exists(&host, "projects"),
        "projects table should exist"
    );
    assert!(
        !fixture::table_exists(&host, "brains"),
        "brains table should be gone"
    );
    assert!(
        fixture::column_exists(&host, "bookmarks", "project"),
        "bookmarks.project column should exist"
    );
    assert!(
        !fixture::column_exists(&host, "bookmarks", "brain"),
        "bookmarks.brain column should be gone"
    );
    assert!(
        fixture::column_exists(&host, "tickets", "project_name"),
        "tickets.project_name column should exist"
    );
    assert!(
        !fixture::column_exists(&host, "tickets", "brain_name"),
        "tickets.brain_name column should be gone"
    );

    // The migrated project row carries forward as a Project entity at
    // the typed client level.
    let client = app.client();
    let listed = client
        .project()
        .list(&ListProjects::builder_v1().build().into())
        .await?;
    match listed {
        ProjectResponse::Listed(ProjectsResponse::V1(projects)) => {
            assert_eq!(projects.items.len(), 1, "one project should survive");
            assert_eq!(projects.items[0].name.as_str(), fixture::PROJECT_NAME);
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    // Event rows are preserved as-is on disk — type tags and data fields
    // keep their pre-rename form. Forward compatibility is handled by the
    // event protocol's versioning (type-tag aliases + V1 historical structs
    // with TryFrom upcasts), not by data-rewriting migrations.
    let events_db =
        rusqlite::Connection::open(data_dir.join(fixture::PROJECT_NAME).join("events.db"))?;
    let preserved_tags: Vec<String> = events_db
        .prepare("SELECT event_type FROM events ORDER BY event_type")?
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        preserved_tags,
        vec!["bookmark-created".to_string(), "brain-created".to_string()],
        "legacy event_type column values are preserved on disk"
    );

    // Each preserved row decodes through the current Events enum without
    // landing in `Event::Unknown` — proving the versioning layer handles
    // legacy data end-to-end.
    let rows: Vec<(String, String)> = events_db
        .prepare("SELECT event_type, data FROM events")?
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    for (event_type, data) in &rows {
        let value: serde_json::Value = serde_json::from_str(data)?;
        let known: Events = serde_json::from_value(value).unwrap_or_else(|err| {
            panic!("legacy {event_type} should decode via current Events: {err}")
        });
        match (event_type.as_str(), known) {
            ("brain-created", Events::Project(ProjectEvents::ProjectCreated(inner))) => {
                let current = inner.current().expect("upcast brain-created");
                assert_eq!(current.project.name.as_str(), fixture::PROJECT_NAME);
            }
            ("bookmark-created", Events::Bookmark(BookmarkEvents::BookmarkCreated(inner))) => {
                let current = inner.current().expect("upcast bookmark-created");
                assert_eq!(current.bookmark.project.as_str(), fixture::PROJECT_NAME);
            }
            (other, _) => panic!("unexpected legacy event_type: {other}"),
        }
    }

    // A backup snapshot was taken — the directory should exist as a
    // sibling of the data-dir.
    let backup_count = fixture::count_backups(&data_dir);
    assert_eq!(backup_count, 1, "exactly one backup snapshot should exist");

    Ok(())
}

#[tokio::test]
async fn pristine_data_dir_takes_no_backup() -> Result<(), Box<dyn core::error::Error>> {
    // Fresh tempdir, no fixture — the migration hook should run and
    // detect nothing to do, leaving no backup behind.
    let app = TestApp::new().await?;
    let backup_count = fixture::count_backups(&app.config().data_dir);
    assert_eq!(
        backup_count, 0,
        "a pristine boot should not snapshot the data-dir"
    );
    Ok(())
}

mod fixture {
    use std::path::Path;

    use crate::Platform;

    pub(crate) const PROJECT_NAME: &str = "test-project";
    pub(crate) const PROJECT_ID: &str = "01999999-9999-7999-8999-999999999999";
    pub(crate) const BOOKMARK_ID: &str = "01888888-8888-7888-8888-888888888888";

    /// Write a pre-rename layout into `dir`: `system.db` with the
    /// legacy schema and a single project row, plus a per-project
    /// `events.db` with legacy-tagged events.
    pub(crate) fn stage_pre_rename(dir: &Path) -> std::io::Result<()> {
        write_legacy_system_db(dir).expect("stage system.db");
        write_legacy_events_db(dir).expect("stage events.db");
        Ok(())
    }

    fn write_legacy_system_db(dir: &Path) -> Result<(), rusqlite::Error> {
        let conn = rusqlite::Connection::open(dir.join("system.db"))?;

        conn.execute_batch(
            "CREATE TABLE brains (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT ''
            );
            CREATE TABLE bookmarks (
                id TEXT PRIMARY KEY,
                brain TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(brain, name)
            );
            CREATE TABLE tickets (
                id TEXT PRIMARY KEY,
                actor_id TEXT NOT NULL,
                brain_name TEXT NOT NULL,
                brain_id TEXT NOT NULL,
                token TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE follows (
                id TEXT PRIMARY KEY,
                brain TEXT NOT NULL,
                bookmark TEXT NOT NULL,
                source TEXT NOT NULL,
                created_at TEXT NOT NULL
            );",
        )?;

        conn.execute(
            "INSERT INTO brains (id, name, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![PROJECT_ID, PROJECT_NAME, "2026-01-01T00:00:00Z"],
        )?;

        conn.execute(
            "INSERT INTO bookmarks (id, brain, name, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![BOOKMARK_ID, PROJECT_NAME, "main", "2026-01-01T00:00:00Z"],
        )?;

        Ok(())
    }

    fn write_legacy_events_db(dir: &Path) -> Result<(), rusqlite::Error> {
        let project_dir = dir.join(PROJECT_NAME);
        Platform::new(dir.to_path_buf())
            .ensure_dir(&project_dir)
            .expect("project dir");
        let conn = rusqlite::Connection::open(project_dir.join("events.db"))?;

        conn.execute_batch(
            "CREATE TABLE events (
                id TEXT PRIMARY KEY,
                event_type TEXT NOT NULL,
                data TEXT NOT NULL,
                source TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL
            )",
        )?;

        // brain-created event with the legacy flattened payload.
        let brain_created = serde_json::json!({
            "type": "brain-created",
            "data": {
                "id": PROJECT_ID,
                "name": PROJECT_NAME,
                "created_at": "2026-01-01T00:00:00Z"
            }
        });
        conn.execute(
            "INSERT INTO events (id, event_type, data, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                "01000000-0000-7000-8000-000000000001",
                "brain-created",
                brain_created.to_string(),
                "2026-01-01T00:00:00Z",
            ],
        )?;

        // bookmark-created event carrying the old `brain` field.
        let bookmark_created = serde_json::json!({
            "type": "bookmark-created",
            "data": {
                "id": BOOKMARK_ID,
                "brain": PROJECT_NAME,
                "name": "main",
                "created_at": "2026-01-01T00:00:00Z"
            }
        });
        conn.execute(
            "INSERT INTO events (id, event_type, data, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                "01000000-0000-7000-8000-000000000002",
                "bookmark-created",
                bookmark_created.to_string(),
                "2026-01-01T00:00:00Z",
            ],
        )?;

        Ok(())
    }

    pub(crate) fn table_exists(conn: &rusqlite::Connection, name: &str) -> bool {
        let count: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                rusqlite::params![name],
                |row| row.get(0),
            )
            .unwrap_or(0);
        count > 0
    }

    pub(crate) fn column_exists(conn: &rusqlite::Connection, table: &str, column: &str) -> bool {
        if !table_exists(conn, table) {
            return false;
        }
        let sql = format!("PRAGMA table_info({table})");
        let mut stmt = match conn.prepare(&sql) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let mut rows = match stmt.query([]) {
            Ok(r) => r,
            Err(_) => return false,
        };
        while let Ok(Some(row)) = rows.next() {
            let name: String = match row.get(1) {
                Ok(n) => n,
                Err(_) => continue,
            };
            if name == column {
                return true;
            }
        }
        false
    }

    pub(crate) fn count_backups(data_dir: &Path) -> usize {
        let Some(parent) = data_dir.parent() else {
            return 0;
        };
        let Some(stem) = data_dir.file_name().and_then(|n| n.to_str()) else {
            return 0;
        };
        let prefix = format!("{stem}.backup-");
        let platform = Platform::new(parent.to_path_buf());
        let Ok(entries) = platform.read_dir(parent) else {
            return 0;
        };
        entries
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .map(|name| name.starts_with(&prefix))
                    .unwrap_or(false)
            })
            .count()
    }
}
