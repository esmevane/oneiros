//! Experience view — presentation authority for the experience domain.
//!
//! Maps experience responses into shared view primitives (Table, Detail).
//! The domain knows its own shape; the rendering layer decides how to
//! display it.

use crate::*;

pub struct ExperienceView;

impl ExperienceView {
    /// Confirmation string for a successful create, using the response ref token.
    pub fn recorded(wrapped: &Response<Experience>) -> String {
        wrapped
            .meta()
            .ref_token()
            .map(|ref_token| {
                format!(
                    "{} Experience recorded: {}",
                    "✓".success(),
                    ref_token.muted()
                )
            })
            .unwrap_or_default()
    }

    /// Confirmation string for a successful update, using the response ref token.
    pub fn updated(wrapped: &Response<Experience>) -> String {
        wrapped
            .meta()
            .ref_token()
            .map(|ref_token| {
                format!(
                    "{} Experience updated: {}",
                    "✓".success(),
                    ref_token.muted()
                )
            })
            .unwrap_or_default()
    }

    /// Table of experiences with standard columns.
    pub fn table(items: &Listed<Response<Experience>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("sensation", "Sensation"),
            Column::key("description", "Description").max(60),
            Column::key("ref_token", "Ref"),
        ]);

        for wrapped in &items.items {
            let ref_token = wrapped
                .meta()
                .ref_token()
                .map(|t| t.to_string())
                .unwrap_or_default();
            table.push_row(vec![
                wrapped.data.sensation.to_string(),
                wrapped.data.description.to_string(),
                ref_token,
            ]);
        }

        table
    }

    /// Detail view for a single experience.
    pub fn detail(item: &Experience) -> Detail {
        Detail::new(item.sensation.to_string()).field("description:", item.description.to_string())
    }
}
