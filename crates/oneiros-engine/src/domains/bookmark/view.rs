use crate::*;

pub struct BookmarkView {
    response: BookmarkResponse,
}

impl BookmarkView {
    pub fn new(response: BookmarkResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<BookmarkResponse> {
        match self.response {
            BookmarkResponse::Created(BookmarkCreatedResponse::V1(created)) => {
                let prompt =
                    Confirmation::new("Bookmark", created.bookmark.name.to_string(), "created")
                        .to_string();
                Rendered::new(
                    BookmarkResponse::Created(BookmarkCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            BookmarkResponse::Forked(BookmarkForkedResponse::V1(forked)) => {
                let prompt = Confirmation::new(
                    "Bookmark",
                    forked.bookmark.name.to_string(),
                    format!("forked from '{}'", forked.from),
                )
                .to_string();
                Rendered::new(
                    BookmarkResponse::Forked(BookmarkForkedResponse::V1(forked)),
                    prompt,
                    String::new(),
                )
            }
            BookmarkResponse::Switched(BookmarkSwitchedResponse::V1(switched)) => {
                let prompt =
                    Confirmation::new("Bookmark", switched.name.to_string(), "switched to")
                        .to_string();
                Rendered::new(
                    BookmarkResponse::Switched(BookmarkSwitchedResponse::V1(switched)),
                    prompt,
                    String::new(),
                )
            }
            BookmarkResponse::Merged(BookmarkMergedResponse::V1(merged)) => {
                let prompt = Confirmation::new(
                    "Bookmark",
                    merged.source.to_string(),
                    format!("merged into '{}'", merged.target),
                )
                .to_string();
                Rendered::new(
                    BookmarkResponse::Merged(BookmarkMergedResponse::V1(merged)),
                    prompt,
                    String::new(),
                )
            }
            BookmarkResponse::Bookmarks(listed) => {
                let mut table = Table::new(vec![Column::key("name", "Name")]);
                for bookmark in &listed.items {
                    table.push_row(vec![bookmark.name.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(BookmarkResponse::Bookmarks(listed), prompt, String::new())
            }
            BookmarkResponse::Shared(result) => {
                let prompt = result.uri.clone();
                Rendered::new(BookmarkResponse::Shared(result), prompt, String::new())
            }
            BookmarkResponse::Followed(follow) => {
                let prompt = Confirmation::new("Bookmark", follow.bookmark.to_string(), "followed")
                    .to_string();
                Rendered::new(BookmarkResponse::Followed(follow), prompt, String::new())
            }
            BookmarkResponse::Collected(result) => {
                let prompt = Confirmation::new(
                    "Bookmark",
                    format!("{} events", result.events_received),
                    format!("collected (sequence {})", result.checkpoint.sequence),
                )
                .to_string();
                Rendered::new(BookmarkResponse::Collected(result), prompt, String::new())
            }
            BookmarkResponse::Unfollowed(BookmarkUnfollowedResponse::V1(unfollowed)) => {
                let prompt =
                    Confirmation::new("Bookmark", unfollowed.bookmark.to_string(), "unfollowed")
                        .to_string();
                Rendered::new(
                    BookmarkResponse::Unfollowed(BookmarkUnfollowedResponse::V1(unfollowed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
