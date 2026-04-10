use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum BookmarkCommands {
    Create(CreateBookmark),
    Switch(SwitchBookmark),
    Merge(MergeBookmark),
    List(ListBookmarks),
    Share(ShareBookmark),
    Follow(FollowBookmark),
    Collect(CollectBookmark),
    Unfollow(UnfollowBookmark),
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
            BookmarkCommands::Share(share) => bookmark_client.share(brain, share).await?,
            BookmarkCommands::Follow(follow) => bookmark_client.follow(brain, follow).await?,
            BookmarkCommands::Collect(collect) => bookmark_client.collect(brain, collect).await?,
            BookmarkCommands::Unfollow(unfollow) => {
                bookmark_client.unfollow(brain, unfollow).await?
            }
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
            BookmarkResponse::Shared(result) => result.uri.clone(),
            BookmarkResponse::Followed(follow) => {
                format!("Following as '{}'", follow.bookmark)
            }
            BookmarkResponse::Collected(result) => {
                format!(
                    "Collected {} events (sequence {})",
                    result.events_received, result.checkpoint.sequence
                )
            }
            BookmarkResponse::Unfollowed(unfollowed) => {
                format!("Unfollowed '{}'", unfollowed.bookmark)
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
