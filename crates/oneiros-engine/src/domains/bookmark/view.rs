//! Bookmark view — presentation authority for the bookmark domain.
//!
//! Maps bookmark responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering
//! layer decides how to display it.

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

    pub fn created(created: &BookmarkCreated) -> Confirmation {
        Confirmation::new("Bookmark", created.name.to_string(), "created")
    }

    pub fn forked(forked: &BookmarkForked) -> Confirmation {
        Confirmation::new(
            "Bookmark",
            forked.name.to_string(),
            format!("forked from '{}'", forked.from),
        )
    }

    pub fn switched(switched: &BookmarkSwitched) -> Confirmation {
        Confirmation::new("Bookmark", switched.name.to_string(), "switched to")
    }

    pub fn merged(merged: &BookmarkMerged) -> Confirmation {
        Confirmation::new(
            "Bookmark",
            merged.source.to_string(),
            format!("merged into '{}'", merged.target),
        )
    }

    /// Share returns the URI directly — it's the produced artifact
    /// that callers pipe into follow commands.
    pub fn shared(result: &BookmarkShareResult) -> String {
        result.uri.clone()
    }

    pub fn followed(follow: &Follow) -> Confirmation {
        Confirmation::new("Bookmark", follow.bookmark.to_string(), "followed")
    }

    pub fn collected(result: &BookmarkCollectResult) -> Confirmation {
        Confirmation::new(
            "Bookmark",
            format!("{} events", result.events_received),
            format!("collected (sequence {})", result.checkpoint.sequence),
        )
    }

    pub fn unfollowed(unfollowed: &BookmarkUnfollowed) -> Confirmation {
        Confirmation::new("Bookmark", unfollowed.bookmark.to_string(), "unfollowed")
    }
}
