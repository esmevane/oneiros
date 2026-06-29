# Databases — Connection Lifecycle Refactor

Status: **Design sketch — not yet implemented.**
Blocks: in-memory SQLite test mode (the remaining ~31% test speedup).
Predecessor: `database-memory-config.chat.log.md` (the conversation that
mapped the architecture wall and landed the 69% pragma speedup).

## The problem

`rusqlite::Connection` is opened and dropped at ~60 scattered sites across
the engine. Every repo method, every actor handler, every service
orchestration opens its own connection, queries, and drops it. This works
for file-backed SQLite — the data persists on disk between connections —
but it makes in-memory SQLite impossible: `:memory:` databases vanish when
their connection drops, and `cache=shared` URIs are deprecated and
unreliable in rusqlite.

The 69% test speedup from pragma tuning already landed. The remaining 31%
(in-memory mode for tests) is blocked on this architectural limitation.
Anything less than centralizing the connection lifecycle is deferring it.

## The design

### `Databases` — the pool, opaque and cheap to clone

```rust
/// Opaque database pool. `Send + Sync + Clone` — internally `Arc`'d.
/// Consumers never see `rusqlite` directly; they ask for a `DbHandle`
/// by `DbKey` and receive a borrowed connection they can query.
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
```

`Databases` lives in `ServerState` (which is already `Clone` via `Arc`
internals). Consumers clone the `Databases` handle cheaply; the `Arc`
keeps the underlying pool shared.

### `DbKey` — unified enum, three whereabouts

```rust
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
    /// `bookmark.db` for (project, bookmark) — projections + events ATTACHed.
    /// The ATTACH happens at open time; the consumer receives a connection
    /// that already has `events` as an attached schema.
    Bookmark(ProjectName, BookmarkName),
}
```

We deliberately do **not** split `Host` into `HostDb` / `HostLog` variants.
Both readings open the same `host.db` file. Splitting would force two pool
entries for one file and require coordinating writes between them. The
key is *where the connection lives*, not *what the caller wants to do
with it*. The host-db / host-log distinction is a consumer-intent concern
and is already expressed by `EventLog::new` (table `"events"`) vs
`EventLog::attached` (table `"events.events"`).

### `DbHandle` — checkout, not borrow

```rust
/// A checked-out connection. Owns the `rusqlite::Connection` for its
/// lifetime; returns it to the pool on drop. The pool slot is only
/// locked during checkout and checkin — never during the query itself,
/// so the query can span an `.await` freely.
pub(crate) struct DbHandle<'a> {
    connection: Option<rusqlite::Connection>,  // None after take() on drop
    key: DbKey,
    pool: &'a DatabasesInner,
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
```

The slot is `Mutex<Option<Connection>>`. The lock is held only during
checkout (take `Some` → `None`) and checkin (put `None` → `Some`). The
query itself runs with **no lock held** — the consumer owns the
connection for the duration of the handle, including across `.await`s.
The handle is `Send`.

Two consumers wanting the same key race on checkout. The loser waits
briefly on the mutex. For SQLite N=1 per key is fine — queries are fast,
contention is low, and we're not doing concurrent writes anyway.

### `DbConnection` — the async trait, the sqlx seam

```rust
/// The connection surface. This is the seam for sqlx-mode: rusqlite
/// implements it via `spawn_blocking`, sqlx implements it natively.
/// Consumers write `handle.query_row(...).await?` and it works for both.
pub(crate) trait DbConnection {
    type Statement: DbStatement;
    type Row: DbRow;
    type Error: Into<DbError>;

    async fn prepare(&self, sql: &str) -> Result<Self::Statement, Self::Error>;
    async fn execute(&self, sql: &str, params: &dyn DbParams) -> Result<usize, Self::Error>;
    async fn execute_batch(&self, sql: &str) -> Result<(), Self::Error>;
    async fn query_row<T, F>(
        &self,
        sql: &str,
        params: &dyn DbParams,
        f: F,
    ) -> Result<T, Self::Error>
    where
        F: FnOnce(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        T: Send + 'static;
}

pub(crate) trait DbStatement {
    type Row: DbRow;
    type Error: Into<DbError>;

    async fn query_row<T, F>(
        &mut self,
        params: &dyn DbParams,
        f: F,
    ) -> Result<T, Self::Error>
    where
        F: FnOnce(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        T: Send + 'static;

    async fn query_map<T, F>(
        &mut self,
        params: &dyn DbParams,
        f: F,
    ) -> Result<Vec<T>, Self::Error>
    where
        F: FnMut(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        T: Send + 'static;
}

pub(crate) trait DbRow {
    fn get<T: DbValue>(&self, idx: usize) -> Result<T, DbError>;
}
```

Associated types (not `Box<dyn>`) — we know the backend at compile time,
and the generic propagation is acceptable. Object safety would cost an
allocation per query; we can revisit if the propagation hurts.

The `Send + 'static` bounds on closures and return values are required
because the rusqlite impl runs the closure inside `spawn_blocking`. The
audit confirmed every closure in the codebase already meets these bounds
(they're all pure row-mapping: `|row| row.get(0)` or
`|row| Ok((row.get(0)?, row.get(1)?, ...))`).

### `DbTransaction` — separate concern (option B)

Transactions are rare (exactly one site today: `brains_to_projects`
migration). They have a different lifecycle (begin/commit/rollback) and
a different concurrency shape (they hold a write lock). Putting
`transaction()` on `DbConnection` would force every backend to implement
transaction semantics even if only one consumer needs them.

```rust
/// A checked-out connection in a transaction. Separate from `DbHandle`
/// because transactions have different lifecycle semantics and a
/// different concurrency shape. Today only migrations use this.
pub(crate) struct DbTransaction<'a> {
    handle: DbHandle<'a>,
    // rusqlite::Transaction is borrowed from the connection; we hold
    // the connection for the transaction's lifetime.
}

impl DbTransaction<'_> {
    pub(crate) async fn execute_batch(&self, sql: &str) -> Result<(), DbError> { /* ... */ }
    pub(crate) async fn commit(self) -> Result<(), DbError> { /* ... */ }
    pub(crate) async fn rollback(self) -> Result<(), DbError> { /* ... */ }
}

impl Databases {
    pub(crate) async fn transaction(&self, key: DbKey) -> Result<DbTransaction<'_>, DbError> {
        // Same checkout as handle(), but wraps in a transaction.
    }
}
```

### Lazy open + sweep — one lifecycle strategy

```rust
impl Databases {
    pub(crate) async fn handle(&self, key: DbKey) -> Result<DbHandle<'_>, DbError> {
        let mut pools = self.inner.pools.lock().unwrap();

        // Checkout: take the connection out of the slot.
        let connection = match pools.get_mut(&key) {
            Some(entry) => entry.connection.take().ok_or(DbError::CheckedOut)?,
            None => {
                // Lazy open. Drop the lock before any I/O.
                drop(pools);
                let conn = self.open_for_key(&key).await?;
                let mut pools = self.inner.pools.lock().unwrap();
                pools.entry(key.clone()).or_insert(DbEntry {
                    connection: None,
                    last_used: Instant::now(),
                });
                conn
            }
        };

        Ok(DbHandle {
            connection: Some(connection),
            key,
            pool: &self.inner,
        })
    }

    /// Sweep entries unused for longer than the configured TTL.
    /// Called periodically (background task or on idle). Entries that
    /// are checked out (`connection: None`) are skipped — they're in use.
    pub(crate) fn sweep(&self, ttl: Duration) {
        let mut pools = self.inner.pools.lock().unwrap();
        pools.retain(|_, entry| {
            entry.connection.is_some() && entry.last_used.elapsed() < ttl
        });
    }

    async fn open_for_key(&self, key: &DbKey) -> Result<rusqlite::Connection, DbError> {
        // Resolves path, applies pragmas, ATTACHes events for bookmark tier.
        // This is where the existing Config::open_database / bookmark_conn
        // logic lands, refactored to dispatch on DbKey.
    }
}
```

One lifecycle strategy for all keys — no eager host / lazy rest split.
The host db is opened on first `handle(DbKey::Host)` and stays in the
pool until swept. Sweep skips checked-out entries, so an in-use
connection is never dropped.

## The ATTACH question, resolved

`EventLog::attached(conn)` takes a `&rusqlite::Connection` and addresses
`events.events` as the table. The ATTACH happens at open time in
`bookmark_conn()`. So:

- `DbKey::Bookmark(project, bookmark)` → `open_for_key` opens the
  bookmark db, ATTACHes the events db, returns the connection.
- The consumer gets a `DbHandle` that already has events ATTACHed.
- `EventLog::attached(handle)` works as-is.

The pool key naturally encodes "bookmark + events ATTACHed" because
that's what opening a bookmark-tier connection *means*. We don't need
a separate `ProjectLog` entry for the ATTACHed events — the bookmark
connection IS the events connection, addressed via schema.

The only place we need a standalone `ProjectLog` connection (no ATTACH)
is `EventsDb::open` — `bridge/service.rs::handle_fetch_events`,
`project/actors/{import,log}.rs`, `CanonIndex::hydrate_project`. Those
get a `DbKey::ProjectLog(project)` handle with no ATTACH.

## The rusqlite impl — `spawn_blocking`, not `block_in_place`

The test harness uses plain `#[tokio::test]` (current-thread runtime).
`block_in_place` panics on current-thread. So the rusqlite impl of the
async trait must use `spawn_blocking`, which works on any runtime but
allocates a thread per call.

```rust
impl DbConnection for RusqliteConnection {
    async fn query_row<T, F>(&self, sql: &str, params: &dyn DbParams, f: F) -> Result<T, DbError>
    where
        F: FnOnce(&Self::Row) -> Result<T, Self::Error> + Send + 'static,
        T: Send + 'static,
    {
        let sql = sql.to_string();
        let params = params.to_owned();
        tokio::task::spawn_blocking(move || {
            self.conn.query_row(&sql, &params, |row| f(&RusqliteRow(row)))
        })
        .await
        .map_err(|e| DbError::Join(e))?
        .map_err(Into::into)
    }
}
```

The cost is a thread alloc per query (~10-50µs). For SQLite that's
acceptable — queries are fast, and the thread alloc is the dominant
overhead. If this becomes a problem later, the escape hatches are:

1. Flip to multi-thread runtime + `block_in_place` (no thread alloc, but
   requires test-harness change).
2. Batch queries within a single `spawn_blocking` call (one thread alloc
   for multiple queries — requires a "transaction-like" API on `DbHandle`).
3. Keep rusqlite sync for hot paths, only go async for the sqlx seam
   (hybrid — messier but possible).

We don't solve this now. `spawn_blocking` is the correct starting point
because it works everywhere.

## Migration path

The refactor is invasive but mechanical. The shape:

1. **Land `Databases`, `DbKey`, `DbHandle`, the trait, and the rusqlite
   impl.** No consumers change yet. The pool exists but is unused.
2. **Add `Databases` to `ServerState`.** Constructed in `ServerState::bind`,
   cloned per-request like `config` is today.
3. **Migrate repos first.** Each `XDb::open(scope).await?` becomes
   `databases.handle(key).await?`. The `DbKey` is derived from the scope
   tier: `AtHost` → `DbKey::Host`, `AtProject` → `DbKey::ProjectLog`,
   `AtBookmark` → `DbKey::Bookmark`. Repos already take `&Scope<T>`; we
   add a `&Databases` param (or thread it through `Scope` itself).
4. **Migrate actors.** Same pattern — `HostDb::open(scope)` →
   `databases.handle(DbKey::Host)`.
5. **Migrate services/scope/canon.** The orchestration sites. These
   often open multiple dbs in one method; they get multiple handles.
6. **Delete `HostDb` / `EventsDb` / `BookmarkDb` newtypes.** Their `open`
   methods are now just `Databases::handle` calls. The `Deref` to
   `rusqlite::Connection` is replaced by the `DbConnection` trait.
7. **Re-introduce `DatabaseMode::Memory`.** With the pool holding
   connections, `:memory:` works — the connection stays alive in the
   pool. Tests opt in via config.

Steps 1-2 are additive (no behavior change). Steps 3-6 are the mechanical
slog. Step 7 is the payoff.

## Open questions (deferred, not blocking)

- **`DbParams` abstraction.** `&dyn DbParams` works but is awkward. The
  alternative is making `DbConnection` generic over params too, which is
  worse. Keep `&dyn DbParams` for now.
- **Sweep trigger.** Background task vs. inline on `handle()`. Background
  is cleaner; inline is simpler. Decide at implementation time.
- **`Scope` threading.** Does `Databases` live on `Scope` (so repos get
  it from `self.scope.databases()`) or is it passed separately? `Scope`
  is the natural home since it already carries `Config` and the tier
  determines the key. But `Scope` is constructed per-request; `Databases`
  would be an `Arc` clone.
- **The `Config` clone-per-request pattern.** Today `Config` is cloned
  with `project`/`bookmark` set per request. Under `Databases`, the key
  carries that information. The `Config` clone might become unnecessary
  for db access (though it's still used for paths, pragmas, etc.).
