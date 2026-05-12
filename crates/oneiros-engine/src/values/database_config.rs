use bon::Builder;
use serde::{Deserialize, Serialize};

/// Database tuning knobs.
///
/// Lives inside [`Config`] as the `[database]` section. Currently
/// holds the SQLite `limit_attached` pragma — the ceiling on
/// concurrently-attached databases per connection.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct DatabaseConfig {
    /// Maximum number of concurrently-attached databases per connection
    /// (SQLite `limit_attached` pragma). Defaults to 125 — the SQLite
    /// compile-time default.
    #[builder(default = 125u32)]
    pub(crate) limit_attached: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
