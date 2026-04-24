use serde::{Deserialize, Serialize};

use crate::*;

/// Everything the dashboard needs to render its host view and to
/// authenticate into individual brains. Returned by
/// `/dashboard/config` as a single unauthenticated call.
///
/// The client joins `brains` against `tickets` by `brain_name`: a
/// brain is "enterable" iff the host holds a ticket for it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardBootstrap {
    /// Host identity — ed25519 key + bound iroh address.
    pub host: HostIdentity,
    /// Engine crate version — for display and cache-busting.
    pub version: String,
    /// The brain this server instance was started with as its default
    /// context. The dashboard uses it as the landing-brain hint when
    /// multiple are available. Honest for single-brain deployments;
    /// an advisory for multi-brain ones.
    pub current_brain: BrainName,
    /// Every tenant on this host.
    pub tenants: Vec<Tenant>,
    /// Every actor on this host. The dashboard filters by `tenant_id`
    /// to render tenant-detail pages.
    pub actors: Vec<Actor>,
    /// Every brain on this host, whether or not the host holds a ticket.
    pub brains: Vec<Brain>,
    /// Every ticket the host knows about. The token lives at
    /// `ticket.link.token`; the dashboard uses it to auth per-brain.
    pub tickets: Vec<Ticket>,
    /// Every peer this host knows — other hosts it has been told about
    /// for distribution. Does not imply live reachability.
    pub peers: Vec<Peer>,
}
