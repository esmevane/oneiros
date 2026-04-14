use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum BookmarkCommands {
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
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, BookmarkError> {
        
        let bookmark_client = BookmarkClient::new(client);

        let response = match self {
            BookmarkCommands::Create(create) => bookmark_client.create(create).await?,
            BookmarkCommands::Switch(switch) => bookmark_client.switch(switch).await?,
            BookmarkCommands::Merge(merge) => bookmark_client.merge(merge).await?,
            BookmarkCommands::List(list) => bookmark_client.list(list).await?,
            BookmarkCommands::Share(share) => bookmark_client.share(share).await?,
            BookmarkCommands::Follow(follow) => bookmark_client.follow(follow).await?,
            BookmarkCommands::Collect(collect) => bookmark_client.collect(collect).await?,
            BookmarkCommands::Unfollow(unfollow) => bookmark_client.unfollow(unfollow).await?,
        };

        let prompt = match &response {
            BookmarkResponse::Created(created) => BookmarkView::created(created).to_string(),
            BookmarkResponse::Forked(forked) => BookmarkView::forked(forked).to_string(),
            BookmarkResponse::Switched(switched) => BookmarkView::switched(switched).to_string(),
            BookmarkResponse::Merged(merged) => BookmarkView::merged(merged).to_string(),
            BookmarkResponse::Bookmarks(listed) => {
                let table = BookmarkView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            BookmarkResponse::Shared(result) => BookmarkView::shared(result),
            BookmarkResponse::Followed(follow) => BookmarkView::followed(follow).to_string(),
            BookmarkResponse::Collected(result) => BookmarkView::collected(result).to_string(),
            BookmarkResponse::Unfollowed(unfollowed) => {
                BookmarkView::unfollowed(unfollowed).to_string()
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
