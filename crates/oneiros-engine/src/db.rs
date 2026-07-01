//! Database primitives.
//!
//! The old `HostDb`/`EventsDb`/`BookmarkDb` newtypes have been replaced
//! by the [`Databases`] pool. Consumers check out a [`DbHandle`] via
//! `scope.host_db()` / `scope.project_log()` / `scope.bookmark_db()`.
//!
//! See `databases.rs` for the pool implementation and
//! `docs/recipes/database-pool-design.md` for the design.
