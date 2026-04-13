//! Actor view — presentation authority for the actor domain.
//!
//! Maps actor responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering
//! layer decides how to display it.

use crate::*;

pub(crate) struct ActorView;

impl ActorView {
    /// Table of actors with standard columns.
    pub(crate) fn table(actors: &Listed<Response<Actor>>) -> Table {
        let mut table = Table::new(vec![Column::key("name", "Name"), Column::key("id", "ID")]);

        for wrapped in &actors.items {
            let actor = &wrapped.data;
            table.push_row(vec![actor.name.to_string(), actor.id.to_string()]);
        }

        table
    }

    /// Detail view for a single actor.
    pub(crate) fn detail(actor: &Actor) -> Detail {
        Detail::new(actor.name.to_string())
            .field("id:", actor.id.to_string())
            .field("tenant_id:", actor.tenant_id.to_string())
    }

    /// Confirmation for a mutation.
    pub(crate) fn confirmed(verb: &str, name: &ActorName) -> Confirmation {
        Confirmation::new("Actor", name.to_string(), verb)
    }
}
