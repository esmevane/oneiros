//! Memory view — presentation authority for the memory domain.
//!
//! Maps memory responses into shared view primitives (Table, Detail).
//! The domain knows its own shape; the rendering layer decides how to
//! display it.

use crate::*;

pub(crate) struct MemoryView;

impl MemoryView {
    /// Confirmation string for a successful add, using the response ref token.
    pub(crate) fn recorded(wrapped: &Response<Memory>) -> String {
        wrapped
            .meta()
            .ref_token()
            .map(|ref_token| format!("{} Memory recorded: {}", "✓".success(), ref_token.muted()))
            .unwrap_or_default()
    }

    /// Table of memories with standard columns.
    pub(crate) fn table(items: &Listed<Response<Memory>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("level", "Level"),
            Column::key("content", "Content").max(60),
            Column::key("ref_token", "Ref"),
        ]);

        for wrapped in &items.items {
            let ref_token = wrapped
                .meta()
                .ref_token()
                .map(|t| t.to_string())
                .unwrap_or_default();
            table.push_row(vec![
                wrapped.data.level.to_string(),
                wrapped.data.content.to_string(),
                ref_token,
            ]);
        }

        table
    }

    /// Detail view for a single memory.
    pub(crate) fn detail(item: &Memory) -> Detail {
        Detail::new(item.level.to_string()).field("content:", item.content.to_string())
    }
}
