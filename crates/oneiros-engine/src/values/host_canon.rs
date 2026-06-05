use crate::*;

/// The full state of the host database — tenants, actors, projects, tickets.
///
/// Reducers fold events into this struct as a pure function.
/// No database, no Loro — just the state.
#[derive(Default, Clone)]
pub(crate) struct HostCanon {
    pub(crate) actors: Actors,
    pub(crate) projects: Projects,
    pub(crate) tenants: Tenants,
    pub(crate) tickets: Tickets,
    pub(crate) peers: Peers,
    pub(crate) follows: Follows,
    pub(crate) remotes: Remotes,
}
