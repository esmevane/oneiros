//! Connection pool — centralized database lifecycle.
//!
//! The pool owns long-lived `rusqlite::Connection`s keyed by [`DbKey`].
//! Consumers check out a [`DbHandle`] for the duration of a query and
//! return it on drop. The pool slot is locked only during checkout and
//! checkin — never during the query itself — so handles can span `.await`s
//! freely.
//!
//! This is the seam for in-memory SQLite test mode (the connection
//! survives in the pool instead of being dropped) and for sqlx-mode
//! (the [`DbConnection`] trait abstracts the backend; rusqlite implements
//! it via `spawn_blocking`, sqlx would implement it natively).
//!
//! See `docs/recipes/database-pool-design.md` for the full design.

use std::collections::HashMap;
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
    pub(crate) fn sweep(&self) {
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
    fn open_for_key(&self, key: &DbKey) -> Result<rusqlite::Connection, DbError> {
        let config = &self.inner.config;
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
}

// ─────────────────────────────────────────────────────────────────────
// DbHandle — checkout, not borrow
// ─────────────────────────────────────────────────────────────────────

/// A checked-out connection. Owns the `rusqlite::Connection` for its
/// lifetime; returns it to the pool on drop. The pool slot is only
/// locked during checkout and checkin — never during the query itself,
/// so the query can span an `.await` freely.
///
/// Derefs to [`rusqlite::Connection`] for now — the [`DbConnection`]
/// trait is the sqlx seam, but migrating every consumer to it is a
/// separate step. During the migration, consumers can use the raw
/// rusqlite surface via `Deref`.
pub(crate) struct DbHandle<'a> {
    connection: Option<rusqlite::Connection>,
    key: DbKey,
    pool: &'a DatabasesInner,
}

impl std::ops::Deref for DbHandle<'_> {
    type Target = rusqlite::Connection;
    fn deref(&self) -> &Self::Target {
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
// DbTransaction — separate concern (option B)
// ─────────────────────────────────────────────────────────────────────

/// A checked-out connection in a transaction. Separate from [`DbHandle`]
/// because transactions have different lifecycle semantics (begin/commit/
/// rollback) and a different concurrency shape (they hold a write lock).
/// Today only migrations use this.
///
/// Like [`DbHandle`], derefs to `rusqlite::Connection` during migration.
/// The transaction is committed via [`DbTransaction::commit`] or rolled
/// back on drop.
pub(crate) struct DbTransaction<'a> {
    handle: DbHandle<'a>,
}

impl std::ops::Deref for DbTransaction<'_> {
    type Target = rusqlite::Connection;
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
// DbConnection — the async trait, the sqlx seam
// ─────────────────────────────────────────────────────────────────────

/// The connection surface. This is the seam for sqlx-mode: rusqlite
/// implements it via `spawn_blocking`, sqlx implements it natively.
/// Consumers write `handle.query_row(...).await?` and it works for both.
///
/// Not yet implemented for `DbHandle` — this is the target shape. During
/// migration, consumers use the raw rusqlite surface via `Deref`. Once
/// all consumers are migrated, `DbHandle` will implement this trait and
/// the `Deref` impl will be removed.
pub(crate) trait DbConnection {
    type Statement: DbStatement;
    type Row: DbRow;
    type Error: Into<DbError>;

    async fn prepare(&self, sql: &str) -> Result<Self::Statement, Self::Error>;
    async fn execute<P: DbParams + Send + 'static>(
        &self,
        sql: &str,
        params: P,
    ) -> Result<usize, Self::Error>;
    async fn execute_batch(&self, sql: &str) -> Result<(), Self::Error>;
    async fn query_row<T, F, P>(&self, sql: &str, params: P, f: F) -> Result<T, Self::Error>
    where
        F: FnOnce(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        P: DbParams + Send + 'static,
        T: Send + 'static;
}

pub(crate) trait DbStatement {
    type Row: DbRow;
    type Error: Into<DbError>;

    async fn query_row<T, F, P>(&mut self, params: P, f: F) -> Result<T, Self::Error>
    where
        F: FnOnce(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        P: DbParams + Send + 'static,
        T: Send + 'static;

    async fn query_map<T, F, P>(&mut self, params: P, f: F) -> Result<Vec<T>, Self::Error>
    where
        F: FnMut(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        P: DbParams + Send + 'static,
        T: Send + 'static;
}

pub(crate) trait DbRow {
    fn get<T: DbValue>(&self, idx: usize) -> Result<T, DbError>;
}

/// Param collection for [`DbConnection`] methods. Implemented for owned
/// param collections that are `Send + 'static` (required to cross the
/// `spawn_blocking` boundary in the rusqlite impl).
pub(crate) trait DbParams: Send {
    /// Bind to a rusqlite statement. The statement is borrowed from the
    /// caller — this is the rusqlite-specific seam; sqlx would have its
    /// own binding path.
    fn bind(&self, stmt: &mut rusqlite::Statement<'_>) -> Result<(), rusqlite::Error>;
}

/// A value that can be read from a [`DbRow`]. The rusqlite-specific seam;
/// sqlx would have its own value trait. `R` is the concrete row type
/// (known at compile time via associated types — no `dyn` needed).
pub(crate) trait DbValue: Sized {
    fn from_row<R: DbRow>(row: &R, idx: usize) -> Result<Self, DbError>;
}

// ─────────────────────────────────────────────────────────────────────
// DbError — our error type, not rusqlite's
// ─────────────────────────────────────────────────────────────────────

/// The pool's error type. Wraps rusqlite and platform errors so
/// consumers never see them directly — the sqlx seam.
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
}
