//! Connection view — presentation authority for the connection domain.
//!
//! Maps connection responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering layer
//! decides how to display it.

use crate::*;

pub struct ConnectionView;

impl ConnectionView {
    /// Confirmation string for a successful create, using the response ref token.
    pub fn recorded(wrapped: &Response<Connection>) -> String {
        wrapped
            .meta()
            .ref_token()
            .map(|ref_token| {
                format!(
                    "{} Connection recorded: {}",
                    "✓".success(),
                    ref_token.muted()
                )
            })
            .unwrap_or_default()
    }

    /// Confirmation for a removal by id.
    pub fn removed(id: &ConnectionId) -> Confirmation {
        Confirmation::new("Connection", id.to_string(), "removed")
    }

    /// Table of connections with standard columns.
    pub fn table(items: &Listed<Response<Connection>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("nature", "Nature"),
            Column::key("from_ref", "From"),
            Column::key("to_ref", "To"),
            Column::key("ref_token", "Ref"),
        ]);

        for wrapped in &items.items {
            let ref_token = wrapped
                .meta()
                .ref_token()
                .map(|t| t.to_string())
                .unwrap_or_default();
            table.push_row(vec![
                wrapped.data.nature.to_string(),
                wrapped.data.from_ref.to_string(),
                wrapped.data.to_ref.to_string(),
                ref_token,
            ]);
        }

        table
    }

    /// Detail view for a single connection.
    pub fn detail(item: &Connection) -> Detail {
        Detail::new(item.nature.to_string())
            .field("from:", item.from_ref.to_string())
            .field("to:", item.to_ref.to_string())
    }
}
