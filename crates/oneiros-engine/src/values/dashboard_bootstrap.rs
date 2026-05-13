use serde::{Deserialize, Serialize};

use crate::*;

/// Everything the dashboard needs to render its host view and to
/// authenticate into individual projects. Returned by
/// `/dashboard/config` as a single unauthenticated call.
///
/// The client joins `projects` against `tickets` by `project_name`: a
/// project is "enterable" iff the host holds a ticket for it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct DashboardBootstrap {
    /// Host identity — ed25519 key + bound iroh address.
    pub(crate) host: HostIdentity,
    /// Engine crate version — for display and cache-busting.
    pub(crate) version: String,
    /// The project this server instance was started with as its default
    /// context. The dashboard uses it as the landing-project hint when
    /// multiple are available. Honest for single-project deployments;
    /// an advisory for multi-project ones.
    pub(crate) current_project: ProjectName,
    /// Every tenant on this host.
    pub(crate) tenants: Vec<Tenant>,
    /// Every actor on this host. The dashboard filters by `tenant_id`
    /// to render tenant-detail pages.
    pub(crate) actors: Vec<Actor>,
    /// Every project on this host, whether or not the host holds a ticket.
    pub(crate) projects: Vec<Project>,
    /// Every ticket the host knows about. The token lives at
    /// `ticket.link.token`; the dashboard uses it to auth per-project.
    pub(crate) tickets: Vec<Ticket>,
    /// Every peer this host knows — other hosts it has been told about
    /// for distribution. Does not imply live reachability.
    pub(crate) peers: Vec<Peer>,
}
