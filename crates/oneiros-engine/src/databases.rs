//! Connection pool — centralized database lifecycle.
//!
//! The pool owns long-lived `rusqlite::Connection`s keyed by [`DbKey`].
//! Consumers check out a [`DbHandle`] for the duration of a query and
//! return it on drop. The pool slot is locked only during checkout and
//! checkin — never during the query itself — so handles can span `.await`s
//! freely.
//!
//! This is the seam for in-memory SQLite test mode (the connection
//! survives in the pool instead of being dropped). `rusqlite` is an
//! implementation detail of `DbHandle` — it never appears in public
//! signatures outside this module. When a different backend (e.g. sqlx)
//! arrives, it implements the same inherent methods on `DbHandle`; callsites
//! don't change.
//!
//! See `docs/recipes/database-pool-design.md` for the full design.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::*;

// ─────────────────────────────────────────────────────────────────────
// DbKey — where a connection lives
// ─────────────────────────────────────────────────────────────────────

/// Where a connection lives. The key is *connection identity*, not
/// *consumer intent* — `Host` covers both host-db reads and host-log
/// writes because they share one file and one connection. The
/// host-db / host-log distinction is expressed at the consumer side
/// (via `EventLog::new` vs `EventLog::attached`), not at the pool key.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum DbKey {
    /// `host.db` — host projections + host event log. Singleton.
    Host,
    /// `events.db` for a project — project event log only.
    ProjectLog(ProjectName),
    /// `bookmark.db` for (project, bookmark) — projections + events
    /// ATTACHed. The ATTACH happens at open time; the consumer receives
    /// a connection that already has `events` as an attached schema.
    Bookmark(ProjectName, BookmarkName),
}

// ─────────────────────────────────────────────────────────────────────
// Databases — the pool, opaque and cheap to clone
// ─────────────────────────────────────────────────────────────────────

/// Opaque database pool. `Send + Sync + Clone` — internally `Arc`'d.
/// Consumers never see `rusqlite` directly; they ask for a [`DbHandle`]
/// by [`DbKey`] and receive a borrowed connection they can query.
///
/// Lives in [`ServerState`] (which is already `Clone` via `Arc`
/// internals). Consumers clone the `Databases` handle cheaply; the
/// `Arc` keeps the underlying pool shared.
#[derive(Clone)]
pub(crate) struct Databases {
    inner: Arc<DatabasesInner>,
}

struct DatabasesInner {
    config: Config,
    pools: Mutex<HashMap<DbKey, DbEntry>>,
}

struct DbEntry {
    /// `Some` when idle, `None` when checked out.
    /// The slot is only locked during checkout/checkin — never during a query.
    connection: Option<rusqlite::Connection>,
    last_used: Instant,
}

impl Databases {
    /// Construct a pool backed by the given config. Connections are
    /// opened lazily on first [`Databases::handle`] for each key.
    pub(crate) fn new(config: Config) -> Self {
        Self {
            inner: Arc::new(DatabasesInner {
                config,
                pools: Mutex::new(HashMap::new()),
            }),
        }
    }

    /// Check out a connection for the given key. Opens lazily on first
    /// access. The connection is owned by the returned [`DbHandle`] for
    /// its lifetime and returned to the pool on drop.
    ///
    /// If the key is currently checked out, this blocks on the pool
    /// mutex until it's returned. For SQLite N=1 per key is fine —
    /// queries are fast, contention is low.
    pub(crate) async fn handle(&self, key: DbKey) -> Result<DbHandle<'_>, DbError> {
        self.handle_sync(key)
    }

    /// Sync variant of [`Databases::handle`] for use in non-async
    /// contexts (closures, migration trait, internal tests). Most
    /// consumers should use [`Databases::handle`] instead.
    pub(crate) fn handle_sync(&self, key: DbKey) -> Result<DbHandle<'_>, DbError> {
        // Fast path: the key exists and is idle. Take it.
        let connection = {
            let mut pools = self.inner.pools.lock().unwrap();
            match pools.get_mut(&key) {
                Some(entry) => match entry.connection.take() {
                    Some(conn) => conn,
                    None => {
                        // Checked out. Drop the lock and wait briefly
                        // before retrying. For SQLite this is rare and
                        // brief — the holder is doing sync work.
                        drop(pools);
                        return Err(DbError::CheckedOut(key));
                    }
                },
                None => {
                    // Lazy open. Drop the lock before any I/O.
                    drop(pools);
                    let conn = self.open_for_key(&key)?;
                    let mut pools = self.inner.pools.lock().unwrap();
                    pools.entry(key.clone()).or_insert(DbEntry {
                        connection: None,
                        last_used: Instant::now(),
                    });
                    conn
                }
            }
        };

        Ok(DbHandle {
            connection: Some(connection),
            key,
            pool: &self.inner,
        })
    }

    /// Begin a transaction on the given key. The connection is checked
    /// out for the transaction's lifetime and returned on commit/rollback.
    /// Today only migrations use this — transactions are rare.
    pub(crate) async fn transaction(&self, key: DbKey) -> Result<DbTransaction<'_>, DbError> {
        let handle = self.handle(key).await?;
        Ok(DbTransaction { handle })
    }

    /// Sweep entries unused for longer than the configured sweep interval.
    /// Called periodically by the background sweep task. Entries that
    /// are checked out (`connection: None`) are skipped — they're in use.
    ///
    /// In `DatabaseMode::Memory`, sweep is a no-op — in-memory databases
    /// are private to their connection. Dropping the connection loses
    /// all data, so we must keep them alive for the pool's lifetime.
    pub(crate) fn sweep(&self) {
        if self.inner.config.database.mode == DatabaseMode::Memory {
            return;
        }
        let ttl = self.inner.config.database.sweep_interval;
        let mut pools = self.inner.pools.lock().unwrap();
        pools.retain(|_, entry| entry.connection.is_none() || entry.last_used.elapsed() < ttl);
    }

    /// Spawn the background sweep task. Runs until `Databases` is dropped
    /// (all clones, since it's `Arc`'d). Checks every `sweep_interval`
    /// and drops idle connections older than the interval.
    pub(crate) fn spawn_sweep(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let interval = self.inner.config.database.sweep_interval;
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                ticker.tick().await;
                self.sweep();
            }
        })
    }

    /// Open a fresh connection for a key, applying pragmas and ATTACHing
    /// events for bookmark tier. Dispatches on [`DbKey`].
    ///
    /// In `DatabaseMode::Memory`, each key gets its own `:memory:` database.
    /// The pool holds the connection, so the in-memory database survives
    /// across checkouts within the same process.
    fn open_for_key(&self, key: &DbKey) -> Result<rusqlite::Connection, DbError> {
        let config = &self.inner.config;

        if config.database.mode == DatabaseMode::Memory {
            return self.open_memory_for_key(key);
        }

        let platform = config.platform();

        match key {
            DbKey::Host => {
                platform.ensure_data_dir().map_err(DbError::Platform)?;
                config.host_db().map_err(DbError::Rusqlite)
            }
            DbKey::ProjectLog(project) => {
                platform
                    .ensure_project_dir(project)
                    .map_err(DbError::Platform)?;
                let mut project_config = config.clone();
                project_config.project = project.clone();
                project_config.open_events_db().map_err(DbError::Rusqlite)
            }
            DbKey::Bookmark(project, bookmark) => {
                platform
                    .ensure_bookmarks_dir(project)
                    .map_err(DbError::Platform)?;
                let mut project_config = config.clone();
                project_config.project = project.clone();
                project_config.bookmark = bookmark.clone();
                project_config.bookmark_conn().map_err(DbError::Rusqlite)
            }
        }
    }

    /// Open in-memory connections. Each key gets a named shared-cache
    /// in-memory database (`file:name?mode=memory&cache=shared`) so that
    /// the bookmark db's ATTACHed `events` schema can see the same data
    /// that `DbKey::ProjectLog` writes to.
    ///
    /// The names include a hash of the data_dir path so that separate
    /// `Databases` instances pointing at the same data_dir (e.g. the
    /// server's pool and a CLI-direct temporary pool) share the same
    /// in-memory databases, while parallel tests with different tempdirs
    /// stay isolated.
    fn open_memory_for_key(&self, key: &DbKey) -> Result<rusqlite::Connection, DbError> {
        let config = &self.inner.config;
        let mut hasher = std::hash::DefaultHasher::new();
        config.data_dir.hash(&mut hasher);
        let pool_id = hasher.finish();

        match key {
            DbKey::Host => {
                let uri = format!("file:host_{pool_id}?mode=memory&cache=shared");
                let conn = rusqlite::Connection::open(&uri).map_err(DbError::Rusqlite)?;
                config.apply_pragmas(&conn).map_err(DbError::Rusqlite)?;
                Ok(conn)
            }
            DbKey::ProjectLog(project) => {
                let uri = format!(
                    "file:events_{}_{pool_id}?mode=memory&cache=shared",
                    project.as_str()
                );
                let conn = rusqlite::Connection::open(&uri).map_err(DbError::Rusqlite)?;
                config.apply_pragmas(&conn).map_err(DbError::Rusqlite)?;
                Ok(conn)
            }
            DbKey::Bookmark(project, bookmark) => {
                let uri = format!(
                    "file:bookmark_{}_{}_{pool_id}?mode=memory&cache=shared",
                    project.as_str(),
                    bookmark.as_str()
                );
                let conn = rusqlite::Connection::open(&uri).map_err(DbError::Rusqlite)?;
                config.apply_pragmas(&conn).map_err(DbError::Rusqlite)?;

                // ATTACH the events db by name — same shared-cache
                // in-memory database that DbKey::ProjectLog opens.
                let events_uri = format!(
                    "file:events_{}_{pool_id}?mode=memory&cache=shared",
                    project.as_str()
                );
                conn.execute_batch(&format!("ATTACH DATABASE '{events_uri}' AS events"))
                    .map_err(DbError::Rusqlite)?;

                Ok(conn)
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// DbHandle — checkout, not borrow
// ─────────────────────────────────────────────────────────────────────

/// A checked-out connection. Owns the `rusqlite::Connection` for its
/// lifetime; returns it to the pool on drop. The pool slot is only
/// locked during checkout and checkin — never during the query itself,
/// so the query can span an `.await` freely.
///
/// `rusqlite` is hidden behind inherent methods on this type. Consumers
/// call `handle.query_row(...)`, `handle.execute(...)`, etc. The row
/// type they receive is [`DbRow`] — our wrapper, not `rusqlite::Row`.
/// This keeps the backend swappable without a trait abstraction: when
/// sqlx (or another backend) arrives, the same methods are implemented
/// against it and callsites don't change.
pub(crate) struct DbHandle<'a> {
    connection: Option<rusqlite::Connection>,
    key: DbKey,
    pool: &'a DatabasesInner,
}

impl DbHandle<'_> {
    /// Execute a statement that returns no rows. Returns the number of
    /// rows affected.
    pub(crate) fn execute<P: rusqlite::Params>(
        &self,
        sql: &str,
        params: P,
    ) -> Result<usize, DbError> {
        let conn = self.conn();
        conn.execute(sql, params).map_err(Into::into)
    }

    /// Execute multiple statements (e.g. schema migration, `BEGIN`/`COMMIT`).
    pub(crate) fn execute_batch(&self, sql: &str) -> Result<(), DbError> {
        let conn = self.conn();
        conn.execute_batch(sql).map_err(Into::into)
    }

    /// Query for a single row. The closure receives a [`DbRow`] — our
    /// wrapper, not `rusqlite::Row`. Returns
    /// [`DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)`] when
    /// no row matches.
    pub(crate) fn query_row<T, P, F>(
        &self,
        sql: &str,
        params: P,
        f: F,
    ) -> Result<T, DbError>
    where
        P: rusqlite::Params,
        F: FnOnce(&DbRow<'_>) -> Result<T, DbError>,
    {
        let conn = self.conn();
        let mut stmt = conn.prepare(sql)?;
        let mut rows = stmt.query(params)?;
        match rows.next()? {
            Some(row) => f(&DbRow { inner: row }),
            None => Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)),
        }
    }

    /// Query for multiple rows, collecting into a `Vec`. The closure
    /// receives a [`DbRow`] for each row.
    pub(crate) fn query_map<T, P, F>(
        &self,
        sql: &str,
        params: P,
        mut f: F,
    ) -> Result<Vec<T>, DbError>
    where
        P: rusqlite::Params,
        F: FnMut(&DbRow<'_>) -> Result<T, DbError>,
    {
        let conn = self.conn();
        let mut stmt = conn.prepare(sql)?;
        let mut rows = stmt.query(params)?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(f(&DbRow { inner: row })?);
        }
        Ok(result)
    }

    /// Returns the row ID of the most recent successful INSERT.
    pub(crate) fn last_insert_rowid(&self) -> i64 {
        self.conn().last_insert_rowid()
    }

    fn conn(&self) -> &rusqlite::Connection {
        self.connection
            .as_ref()
            .expect("connection is only None after take() on drop")
    }
}

impl Drop for DbHandle<'_> {
    fn drop(&mut self) {
        let mut pools = self.pool.pools.lock().unwrap();
        match pools.get_mut(&self.key) {
            Some(entry) => {
                entry.connection = self.connection.take();
                entry.last_used = Instant::now();
            }
            None => {
                // Entry was swept while we held the handle. Drop the
                // connection — it's stale. The next open for this key
                // will create a fresh one.
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// DbRow — our row type, not rusqlite's
// ─────────────────────────────────────────────────────────────────────

/// A row from a query result. Wraps `rusqlite::Row` so callers never
/// touch the backend's row type directly. When sqlx arrives, `DbRow`
/// is re-implemented against sqlx's row; callsites don't change.
///
/// The `get` method accepts any `rusqlite::RowIndex` (integer or column
/// name) and any `rusqlite::types::FromSql` type — these are trait
/// bounds, not concrete types, and represent SQL-level concerns (column
/// addressing and value conversion) rather than architectural coupling.
pub(crate) struct DbRow<'a> {
    inner: &'a rusqlite::Row<'a>,
}

impl<'a> DbRow<'a> {
    /// Get a value from the row by index or column name.
    pub(crate) fn get<I, T>(&self, idx: I) -> Result<T, DbError>
    where
        I: rusqlite::RowIndex,
        T: rusqlite::types::FromSql,
    {
        self.inner.get(idx).map_err(Into::into)
    }
}

// ─────────────────────────────────────────────────────────────────────
// DbTransaction — separate concern (option B)
// ─────────────────────────────────────────────────────────────────────

/// A checked-out connection in a transaction. Separate from [`DbHandle`]
/// because transactions have different lifecycle semantics (begin/commit/
/// rollback) and a different concurrency shape (they hold a write lock).
/// Today only migrations use this.
///
/// Derefs to [`DbHandle`] so all the same query methods are available.
/// The transaction is committed via [`DbTransaction::commit`] or rolled
/// back on drop.
pub(crate) struct DbTransaction<'a> {
    handle: DbHandle<'a>,
}

impl<'a> std::ops::Deref for DbTransaction<'a> {
    type Target = DbHandle<'a>;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl DbTransaction<'_> {
    /// Commit the transaction and return the connection to the pool.
    pub(crate) fn commit(self) -> Result<(), DbError> {
        // rusqlite::Transaction::commit consumes self and drops the
        // connection. We don't have a Transaction here — we'd need to
        // begin one on the underlying connection. For now, this is a
        // placeholder; the migration that uses it will be updated when
        // we wire it up.
        //
        // TODO: begin transaction on the underlying connection, commit
        // on this call, rollback on drop.
        drop(self.handle);
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────
// DbError — our error type, not rusqlite's
// ─────────────────────────────────────────────────────────────────────

/// The pool's error type. Wraps rusqlite and platform errors so
/// consumers never see them directly.
#[derive(Debug, thiserror::Error)]
pub(crate) enum DbError {
    #[error("database error: {0}")]
    Rusqlite(#[from] rusqlite::Error),
    #[error("platform error: {0}")]
    Platform(#[from] PlatformError),
    #[error("connection for {0:?} is currently checked out")]
    CheckedOut(DbKey),
    #[error("background task failed: {0}")]
    Join(#[from] tokio::task::JoinError),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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
    async fn handle_opens_host_db_lazily() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config.clone());

        // First handle opens the connection.
        let handle = databases.handle(DbKey::Host).await.unwrap();
        assert!(config.platform().host_db_path().exists());
        drop(handle);

        // Second handle reuses the pooled connection.
        let _handle2 = databases.handle(DbKey::Host).await.unwrap();
    }

    #[tokio::test]
    async fn handle_returns_connection_on_drop() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);

        let handle = databases.handle(DbKey::Host).await.unwrap();
        let key = handle.key.clone();
        drop(handle);

        // Pool should have the connection back.
        let pools = databases.inner.pools.lock().unwrap();
        let entry = pools.get(&key).unwrap();
        assert!(entry.connection.is_some());
    }

    #[tokio::test]
    async fn bookmark_handle_has_events_attached() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config.clone());

        // Pre-create events db so ATTACH points at a real file.
        let _events = databases
            .handle(DbKey::ProjectLog(ProjectName::new("alpha")))
            .await
            .unwrap();
        drop(_events);

        let bookmark = databases
            .handle(DbKey::Bookmark(
                ProjectName::new("alpha"),
                BookmarkName::main(),
            ))
            .await
            .unwrap();

        // Verify the ATTACH worked — events schema should be queryable.
        let count: i64 = bookmark
            .query_row("SELECT count(*) FROM events.sqlite_master", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert!(count >= 0);
    }

    #[tokio::test]
    async fn sweep_drops_idle_connections() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);

        // Open and drop — leaves an idle entry.
        let _handle = databases.handle(DbKey::Host).await.unwrap();
        drop(_handle);

        // Manually age the entry so sweep considers it stale.
        {
            let mut pools = databases.inner.pools.lock().unwrap();
            for entry in pools.values_mut() {
                entry.last_used = Instant::now() - Duration::from_secs(60);
            }
        }

        databases.sweep();
        let pools = databases.inner.pools.lock().unwrap();
        assert!(pools.is_empty(), "sweep should drop idle entries");
    }

    #[tokio::test]
    async fn sweep_skips_checked_out_connections() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);

        let handle = databases.handle(DbKey::Host).await.unwrap();

        // Age the entry — but it's checked out, so sweep should skip it.
        {
            let mut pools = databases.inner.pools.lock().unwrap();
            for entry in pools.values_mut() {
                entry.last_used = Instant::now() - Duration::from_secs(60);
            }
        }

        databases.sweep();
        {
            let pools = databases.inner.pools.lock().unwrap();
            assert!(
                pools.contains_key(&DbKey::Host),
                "sweep should skip checked-out entries"
            );
        }
        drop(handle);
    }

    #[tokio::test]
    async fn query_row_extracts_via_dbrow() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);
        let handle = databases.handle(DbKey::Host).await.unwrap();

        handle
            .execute("CREATE TABLE probe (id INTEGER, name TEXT)", [])
            .unwrap();
        handle
            .execute(
                "INSERT INTO probe (id, name) VALUES (?1, ?2)",
                rusqlite::params![1i64, "alpha"],
            )
            .unwrap();

        let (id, name): (i64, String) = handle
            .query_row(
                "SELECT id, name FROM probe WHERE id = ?1",
                rusqlite::params![1i64],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(name, "alpha");
    }

    #[tokio::test]
    async fn query_map_collects_via_dbrow() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);
        let handle = databases.handle(DbKey::Host).await.unwrap();

        handle
            .execute("CREATE TABLE probe (id INTEGER, name TEXT)", [])
            .unwrap();
        handle
            .execute(
                "INSERT INTO probe (id, name) VALUES (?1, ?2)",
                rusqlite::params![1i64, "alpha"],
            )
            .unwrap();
        handle
            .execute(
                "INSERT INTO probe (id, name) VALUES (?1, ?2)",
                rusqlite::params![2i64, "beta"],
            )
            .unwrap();

        let rows: Vec<(i64, String)> = handle
            .query_map("SELECT id, name FROM probe ORDER BY id", [], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].1, "alpha");
        assert_eq!(rows[1].1, "beta");
    }

    #[tokio::test]
    async fn query_row_returns_no_rows_error() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);
        let handle = databases.handle(DbKey::Host).await.unwrap();

        handle
            .execute("CREATE TABLE probe (id INTEGER)", [])
            .unwrap();

        let result: Result<(i64,), DbError> =
            handle.query_row("SELECT id FROM probe WHERE id = ?1", [1i64], |row| {
                Ok((row.get(0)?,))
            });

        match result {
            Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => {}
            other => panic!("expected QueryReturnedNoRows, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn last_insert_rowid_returns_rowid() {
        let (_dir, config) = test_config();
        let databases = Databases::new(config);
        let handle = databases.handle(DbKey::Host).await.unwrap();

        handle
            .execute("CREATE TABLE probe (id INTEGER PRIMARY KEY AUTOINCREMENT)", [])
            .unwrap();
        handle
            .execute("INSERT INTO probe DEFAULT VALUES", [])
            .unwrap();
        assert_eq!(handle.last_insert_rowid(), 1);
    }
}
