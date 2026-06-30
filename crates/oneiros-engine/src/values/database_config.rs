use bon::Builder;
use serde::{Deserialize, Serialize};

/// Database tuning knobs.
///
/// Lives inside [`Config`] as the `[database]` section. Carries every
/// SQLite pragma the engine needs so that all open sites go through one
/// config-driven path instead of scattered `rusqlite::Connection::open`
/// calls with ad-hoc pragma strings.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct DatabaseConfig {
    // ── SQLite pragmas ──────────────────────────────────────────────
    /// Journal mode. Always `"wal"` — write-ahead log for concurrency.
    #[builder(default = String::from("wal"))]
    pub(crate) journal_mode: String,

    /// Synchronous setting. `"normal"` (2) balances safety and speed —
    /// fsyncs at critical moments but not every write. `"full"` (2 fsyncs
    /// per transaction) is safer but much slower; `"off"` is dangerous.
    #[builder(default = String::from("normal"))]
    pub(crate) synchronous: String,

    /// SQLite page cache size in KB. Negative means kibibytes (e.g.
    /// `-2000` = 2 MiB). Positive means number of pages.
    #[builder(default = -2000i64)]
    pub(crate) cache_size: i64,

    /// Where SQLite stores temp tables and indices. `"memory"` (2)
    /// keeps them in RAM; `"file"` (1) writes to disk.
    #[builder(default = String::from("memory"))]
    pub(crate) temp_store: String,

    /// Memory-mapped I/O size in bytes. `0` disables mmap. Setting this
    /// to a large value (e.g. `268435456` = 256 MiB) can improve read
    /// performance but keeps the database pinned in virtual memory.
    #[builder(default = 0u64)]
    pub(crate) mmap_size: u64,

    /// Maximum number of concurrently-attached databases per connection
    /// (SQLite `limit_attached` pragma). Defaults to 125 — the SQLite
    /// compile-time default.
    #[builder(default = 125u32)]
    pub(crate) limit_attached: u32,

    /// How often the background sweep task checks for idle connections to
    /// expire. Connections unused for longer than this interval are
    /// dropped from the pool. Defaults to 100ms.
    #[builder(default = std::time::Duration::from_millis(100))]
    #[serde(with = "humantime_serde")]
    pub(crate) sweep_interval: std::time::Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
