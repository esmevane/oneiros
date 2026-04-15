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
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, BookmarkError> {
        let client = context.client();
        let bookmark_client = BookmarkClient::new(&client);

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

        Ok(BookmarkView::new(response).render().map(Into::into))
    }
}
