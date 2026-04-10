//! Cognition view — presentation authority for the cognition domain.
//!
//! Maps cognition responses into shared view primitives (Table, Detail).
//! The domain knows its own shape; the rendering layer decides how to
//! display it.

use crate::*;

pub struct CognitionView;

impl CognitionView {
    /// Confirmation string for a successful add, using the response ref token.
    pub fn recorded(wrapped: &Response<Cognition>) -> String {
        wrapped
            .meta()
            .ref_token()
            .map(|ref_token| {
                format!(
                    "{} Cognition recorded: {}",
                    "✓".success(),
                    ref_token.muted()
                )
            })
            .unwrap_or_default()
    }

    /// Table of cognitions with standard columns.
    pub fn table(items: &Listed<Response<Cognition>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("texture", "Texture"),
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
                wrapped.data.texture.to_string(),
                wrapped.data.content.to_string(),
                ref_token,
            ]);
        }

        table
    }

    /// Detail view for a single cognition.
    pub fn detail(item: &Cognition) -> Detail {
        Detail::new(item.texture.to_string()).field("content:", item.content.to_string())
    }
}
