use crate::*;

/// The full state of the system database — tenants, actors, brains, tickets.
///
/// Reducers fold events into this struct as a pure function.
/// No database, no Loro — just the state.
#[derive(Default, Clone)]
pub(crate) struct SystemCanon {
    pub(crate) actors: Actors,
    pub(crate) brains: Brains,
    pub(crate) tenants: Tenants,
    pub(crate) tickets: Tickets,
    pub(crate) peers: Peers,
    pub(crate) follows: Follows,
}
