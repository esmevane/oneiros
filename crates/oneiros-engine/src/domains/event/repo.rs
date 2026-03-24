//! Event repository — domain-level event queries.
//!
//! Infrastructure-level event persistence lives in EventLog (event_log.rs).
//! This module is the event domain's read model — queries that serve the
//! user-facing event resource (list, show, search).
//!
//! Currently empty: the event domain's CLI commands are not yet implemented.
//! When they are, this repo will provide query methods over the events table
//! (by type, by date range, by agent, etc.) — the same pattern as every
//! other domain repo, but reading from the event log's table directly.
