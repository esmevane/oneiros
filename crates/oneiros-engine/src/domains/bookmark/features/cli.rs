use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum BookmarkCommands {
    Create(CreateBookmark),
    Switch(SwitchBookmark),
    Merge(MergeBookmark),
    List(ListBookmarks),
}

impl BookmarkCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
        brain: &BrainName,
    ) -> Result<Rendered<Responses>, BookmarkError> {
        let client = context.client();
        let bookmark_client = BookmarkClient::new(&client);

        let response = match self {
            BookmarkCommands::Create(create) => bookmark_client.create(brain, create).await?,
            BookmarkCommands::Switch(switch) => bookmark_client.switch(brain, switch).await?,
            BookmarkCommands::Merge(merge) => bookmark_client.merge(brain, merge).await?,
            BookmarkCommands::List(list) => bookmark_client.list(brain, list).await?,
        };

        let prompt = match &response {
            BookmarkResponse::Created(created) => BookmarkView::created(created),
            BookmarkResponse::Forked(forked) => BookmarkView::forked(forked),
            BookmarkResponse::Switched(switched) => BookmarkView::switched(switched),
            BookmarkResponse::Merged(merged) => BookmarkView::merged(merged),
            BookmarkResponse::Bookmarks(listed) => {
                let table = BookmarkView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
