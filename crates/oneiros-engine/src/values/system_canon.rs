use std::collections::HashMap;

use crate::*;

/// The full state of the system database — tenants, actors, brains, tickets.
///
/// Reducers fold events into this struct as a pure function.
/// No database, no Loro — just the state.
#[derive(Default, Clone)]
pub struct SystemCanon {
    pub tenants: HashMap<String, Tenant>,
    pub actors: HashMap<String, Actor>,
    pub brains: HashMap<String, Brain>,
    pub tickets: HashMap<String, Ticket>,
}
