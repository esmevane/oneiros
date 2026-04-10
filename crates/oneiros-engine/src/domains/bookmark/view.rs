//! Bookmark view — presentation authority for the bookmark domain.
//!
//! Maps bookmark responses into formatted strings and table primitives.
//! The bookmark domain has unique operation shapes that don't map cleanly
//! to the standard Confirmation pattern, so each method returns its own
//! formatted string.

use crate::*;

pub struct BookmarkView;

impl BookmarkView {
    /// Table of bookmarks with standard columns.
    pub fn table(bookmarks: &Listed<Bookmark>) -> Table {
        let mut table = Table::new(vec![Column::key("name", "Name")]);

        for bookmark in &bookmarks.items {
            table.push_row(vec![bookmark.name.to_string()]);
        }

        table
    }

    /// Formatted string for a bookmark creation.
    pub fn created(created: &BookmarkCreated) -> String {
        format!("{} Bookmark '{}' created.", "✓".success(), created.name,)
    }

    /// Formatted string for a bookmark fork.
    pub fn forked(forked: &BookmarkForked) -> String {
        format!(
            "{} Bookmark '{}' forked from '{}'.",
            "✓".success(),
            forked.name,
            forked.from,
        )
    }

    /// Formatted string for a bookmark switch.
    pub fn switched(switched: &BookmarkSwitched) -> String {
        format!(
            "{} Switched to bookmark '{}'.",
            "✓".success(),
            switched.name,
        )
    }

    /// Formatted string for a bookmark merge.
    pub fn merged(merged: &BookmarkMerged) -> String {
        format!(
            "{} Merged '{}' into '{}'.",
            "✓".success(),
            merged.source,
            merged.target,
        )
    }
}
