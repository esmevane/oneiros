//! Brain view — presentation authority for the brain domain.
//!
//! Maps brain responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering
//! layer decides how to display it.

use crate::*;

pub(crate) struct BrainView;

impl BrainView {
    /// Table of brains with standard columns.
    pub(crate) fn table(brains: &Listed<Response<Brain>>) -> Table {
        let mut table = Table::new(vec![Column::key("name", "Name")]);

        for wrapped in &brains.items {
            let brain = &wrapped.data;
            table.push_row(vec![brain.name.to_string()]);
        }

        table
    }

    /// Detail view for a single brain.
    pub(crate) fn detail(brain: &Brain) -> Detail {
        Detail::new(brain.name.to_string())
    }

    /// Confirmation for a mutation.
    pub(crate) fn confirmed(verb: &str, name: &BrainName) -> Confirmation {
        Confirmation::new("Brain", name.to_string(), verb)
    }
}
