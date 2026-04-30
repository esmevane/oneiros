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
        context: &ProjectLog,
    ) -> Result<Rendered<Responses>, BookmarkError> {
        let client = context.client();
        let bookmark_client = BookmarkClient::new(&client);

        let response = match self {
            Self::Create(creation) => bookmark_client.create(creation).await?,
            Self::Switch(switch) => bookmark_client.switch(switch).await?,
            Self::Merge(merge) => bookmark_client.merge(merge).await?,
            Self::List(listing) => bookmark_client.list(listing).await?,
            Self::Share(share) => bookmark_client.share(share).await?,
            Self::Follow(follow) => bookmark_client.follow(follow).await?,
            Self::Collect(collect) => bookmark_client.collect(collect).await?,
            Self::Unfollow(unfollow) => bookmark_client.unfollow(unfollow).await?,
        };

        Ok(BookmarkView::new(response).render().map(Into::into))
    }
}
