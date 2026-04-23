//! Dashboard bootstrap — the host-shaped config endpoint.
//!
//! Returns everything the web UI needs to render the host view and
//! authenticate into individual brains, in a single unauthenticated
//! call. Aggregates from the `SystemContext` — tenants, brains, and
//! tickets — plus the host's iroh identity.

use axum::{Json, extract::State};

use crate::*;

/// GET `/dashboard/config` — host-shaped bootstrap for the web UI.
///
/// Intentionally public: the host view is unauthenticated (same
/// posture as `SystemContext`-scoped endpoints like `/brains`).
/// The tokens returned in `tickets` are the same tokens anyone
/// with local access to the ticket DB can already read.
pub async fn dashboard_config(State(state): State<ServerState>) -> Json<DashboardBootstrap> {
    let system = state.system_context();

    let tenants = TenantRepo::new(&system)
        .list(&SearchFilters::default())
        .await
        .map(|listed| listed.items)
        .unwrap_or_default();

    let brains = BrainRepo::new(&system)
        .list(&SearchFilters::default())
        .await
        .map(|listed| listed.items)
        .unwrap_or_default();

    let tickets = TicketRepo::new(&system)
        .list(&SearchFilters::default())
        .await
        .map(|listed| listed.items)
        .unwrap_or_default();

    Json(DashboardBootstrap {
        host: state.host_identity(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        current_brain: state.brain_name().clone(),
        tenants,
        brains,
        tickets,
    })
}
