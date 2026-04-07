use crate::*;

/// The full state of the system database — tenants, actors, brains, tickets.
///
/// Reducers fold events into this struct as a pure function.
/// No database, no Loro — just the state.
#[derive(Default, Clone)]
pub struct SystemCanon {
    pub actors: Actors,
    pub brains: Brains,
    pub tenants: Tenants,
    pub tickets: Tickets,
}
