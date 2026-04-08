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
            BookmarkResponse::Created(created) => {
                format!(
                    "Bookmark '{}' created for brain '{}'.",
                    created.name, created.brain
                )
            }
            BookmarkResponse::Forked(forked) => {
                format!(
                    "Bookmark '{}' forked from '{}' for brain '{}'.",
                    forked.name, forked.from, forked.brain
                )
            }
            BookmarkResponse::Switched(switched) => {
                format!(
                    "Switched to bookmark '{}' for brain '{}'.",
                    switched.name, switched.brain
                )
            }
            BookmarkResponse::Merged(merged) => {
                format!(
                    "Merged '{}' into '{}' for brain '{}'.",
                    merged.source, merged.target, merged.brain
                )
            }
            BookmarkResponse::Bookmarks(listed) => {
                let mut out = format!("{} bookmarks found.\n\n", listed.len());
                for bookmark in &listed.items {
                    out.push_str(&format!("  {}\n", bookmark.name));
                }
                out
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
