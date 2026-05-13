//! Follow view — presentation authority for the follow domain.

use crate::*;

pub(crate) struct FollowView {
    response: FollowResponse,
}

impl FollowView {
    pub(crate) fn new(response: FollowResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<FollowResponse> {
        match self.response {
            FollowResponse::Found(FollowFoundResponse::V1(found)) => {
                let prompt = Self::detail(&found.follow).to_string();
                Rendered::new(
                    FollowResponse::Found(FollowFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            FollowResponse::Listed(FollowsResponse::V1(listed)) => {
                let prompt = format!(
                    "{}\n\n{}",
                    format_args!(
                        "{} of {} total",
                        listed.follows.items.len(),
                        listed.follows.total
                    )
                    .muted(),
                    Self::table(&listed.follows),
                );
                Rendered::new(
                    FollowResponse::Listed(FollowsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }

    fn table(follows: &Listed<Response<FollowFoundResponse>>) -> Table {
        let mut table = Table::new(vec![
            Column::new("Bookmark"),
            Column::new("Project"),
            Column::new("Source"),
            Column::new("ID"),
        ]);

        for wrapped in &follows.items {
            let FollowFoundResponse::V1(found) = &wrapped.data;
            let follow = &found.follow;
            let source_label = match &follow.source {
                FollowSource::Local(_) => "local",
                FollowSource::Peer(_) => "peer",
            };
            table.push_row(vec![
                follow.bookmark.to_string(),
                follow.project.to_string(),
                source_label.to_string(),
                follow.id.to_string(),
            ]);
        }

        table
    }

    fn detail(follow: &Follow) -> Detail {
        let source_label = match &follow.source {
            FollowSource::Local(_) => "local",
            FollowSource::Peer(_) => "peer",
        };
        Detail::new(follow.bookmark.to_string())
            .field("id:", follow.id.to_string())
            .field("project:", follow.project.to_string())
            .field("source:", source_label.to_string())
            .field(
                "checkpoint.sequence:",
                follow.checkpoint.sequence.to_string(),
            )
            .field("created_at:", follow.created_at.as_string())
    }
}
