//! Follow view — presentation authority for the follow domain.

use crate::*;

pub struct FollowView;

impl FollowView {
    pub fn table(follows: &Listed<Response<Follow>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("bookmark", "Bookmark"),
            Column::key("brain", "Brain"),
            Column::key("source", "Source"),
            Column::key("id", "ID"),
        ]);

        for wrapped in &follows.items {
            let follow = &wrapped.data;
            let source_label = match &follow.source {
                FollowSource::Local(_) => "local",
                FollowSource::Peer(_) => "peer",
            };
            table.push_row(vec![
                follow.bookmark.to_string(),
                follow.brain.to_string(),
                source_label.to_string(),
                follow.id.to_string(),
            ]);
        }

        table
    }

    pub fn detail(follow: &Follow) -> Detail {
        let source_label = match &follow.source {
            FollowSource::Local(_) => "local",
            FollowSource::Peer(_) => "peer",
        };
        Detail::new(follow.bookmark.to_string())
            .field("id:", follow.id.to_string())
            .field("brain:", follow.brain.to_string())
            .field("source:", source_label.to_string())
            .field(
                "checkpoint.sequence:",
                follow.checkpoint.sequence.to_string(),
            )
            .field("created_at:", follow.created_at.as_string())
    }
}
