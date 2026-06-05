use crate::*;

pub(crate) struct RemoteView {
    response: RemoteResponse,
}

impl RemoteView {
    pub(crate) fn new(response: RemoteResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<RemoteResponse> {
        match self.response {
            RemoteResponse::Added(RemoteAddedResponse::V1(added)) => {
                let prompt = Confirmation::new(
                    "Remote",
                    added.remote.name.to_string(),
                    "added",
                )
                .to_string();
                Rendered::new(
                    RemoteResponse::Added(RemoteAddedResponse::V1(added)),
                    prompt,
                    String::new(),
                )
            }
            RemoteResponse::Found(RemoteFoundResponse::V1(found)) => {
                let prompt = Confirmation::new(
                    "Remote",
                    found.remote.name.to_string(),
                    "found",
                )
                .to_string();
                Rendered::new(
                    RemoteResponse::Found(RemoteFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            RemoteResponse::Listed(RemotesResponse::V1(listed)) => {
                let mut table = Table::new(vec![Column::new("Name"), Column::new("Project")]);
                for remote in &listed.items {
                    table.push_row(vec![
                        remote.name.to_string(),
                        remote.project.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    RemoteResponse::Listed(RemotesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            RemoteResponse::Removed(RemoteRemovedResponse::V1(removed)) => {
                let prompt = Confirmation::new(
                    "Remote",
                    removed.name.to_string(),
                    "removed",
                )
                .to_string();
                Rendered::new(
                    RemoteResponse::Removed(RemoteRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
            }
            RemoteResponse::Bookmarks(RemoteBookmarkListResponse::V1(list)) => {
                let mut table = Table::new(vec![Column::new("Bookmark")]);
                for name in &list.bookmarks {
                    table.push_row(vec![name.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} bookmarks", list.bookmarks.len()).muted(),
                );
                Rendered::new(
                    RemoteResponse::Bookmarks(RemoteBookmarkListResponse::V1(list)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
