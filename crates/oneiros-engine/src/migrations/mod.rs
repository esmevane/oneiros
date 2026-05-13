//! Layout migrations — keep on-disk state in step with the engine's
//! current expectations.
//!
//! Each migration is a named struct that knows two things: how to detect
//! whether it has work to do for the current `data-dir` (`is_required`),
//! and how to do that work (`apply`). The orchestrator (`MigrationService`)
//! filters the inventory by `is_required`, snapshots the data-dir to a
//! timestamped backup when at least one migration has work, and runs the
//! required migrations in declaration order.
//!
//! The shape is shape-based, not version-numbered. Each migration reads
//! the disk to decide whether it applies. Re-runs are no-ops. There is no
//! sentinel table or version marker — the disk's shape is the marker.
//!
//! Migrations run on server boot, before any database connections open,
//! so the engine always sees current-layout state. Users never invoke
//! them directly.

mod backup;
mod error;
mod outcome;
mod service;

mod brains_to_projects;
mod system_db_to_host_db;

pub(crate) use backup::*;
pub(crate) use brains_to_projects::*;
pub(crate) use error::*;
pub(crate) use outcome::*;
pub(crate) use service::*;
pub(crate) use system_db_to_host_db::*;

use crate::Config;

/// A single layout migration — detect-then-apply against the data-dir.
///
/// Implementations carry no state; the on-disk layout is the source of
/// truth. `is_required` consults the filesystem and DB schema, `apply`
/// performs the transformation transactionally where possible.
pub(crate) trait Migration {
    /// Human-readable label, surfaced in logs and the boot report.
    fn name(&self) -> &'static str;

    /// True iff this migration has work to do against the current data-dir.
    fn is_required(&self, config: &Config) -> Result<bool, MigrationError>;

    /// Apply the migration. Must be safe to call when `is_required` is
    /// true; behavior is undefined when called against an already-current
    /// data-dir (callers should always gate on `is_required`).
    fn apply(&self, config: &Config) -> Result<(), MigrationError>;
}
