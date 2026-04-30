//! Dashboard bootstrap — the host-shaped config endpoint.
//!
//! Returns everything the web UI needs to render the host view and
//! authenticate into individual brains, in a single unauthenticated
//! call. Aggregates from the `HostLog` — tenants, brains, and
//! tickets — plus the host's iroh identity.

use axum::{Json, extract::State};

use crate::*;

/// GET `/dashboard/config` — host-shaped bootstrap for the web UI.
///
/// Intentionally public: the host view is unauthenticated (same
/// posture as `HostLog`-scoped endpoints like `/brains`).
/// The tokens returned in `tickets` are the same tokens anyone
/// with local access to the ticket DB can already read.
pub async fn dashboard_config(State(state): State<ServerState>) -> Json<DashboardBootstrap> {
    let system = state.host_log();
    let scope = system.scope().ok();

    let tenants = match scope.as_ref() {
        Some(s) => TenantRepo::new(s)
            .list(&SearchFilters::default())
            .await
            .map(|listed| listed.items)
            .unwrap_or_default(),
        None => Vec::new(),
    };

    let actors = match scope.as_ref() {
        Some(s) => ActorRepo::new(s)
            .list(&SearchFilters::default())
            .await
            .map(|listed| listed.items)
            .unwrap_or_default(),
        None => Vec::new(),
    };

    let brains = match scope.as_ref() {
        Some(s) => BrainRepo::new(s)
            .list(&SearchFilters::default())
            .await
            .map(|listed| listed.items)
            .unwrap_or_default(),
        None => Vec::new(),
    };

    let tickets = match scope.as_ref() {
        Some(s) => TicketRepo::new(s)
            .list(&SearchFilters::default())
            .await
            .map(|listed| listed.items)
            .unwrap_or_default(),
        None => Vec::new(),
    };

    let peers = match scope.as_ref() {
        Some(s) => PeerRepo::new(s)
            .list(&SearchFilters::default())
            .await
            .map(|listed| listed.items)
            .unwrap_or_default(),
        None => Vec::new(),
    };

    Json(DashboardBootstrap {
        host: state.host_identity(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        current_brain: state.brain_name().clone(),
        tenants,
        actors,
        brains,
        tickets,
        peers,
    })
}
