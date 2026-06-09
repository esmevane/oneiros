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
    Submit(SubmitBookmark),
}

impl BookmarkCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, BookmarkError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Create(creation) => creation.execute_request(&client).await?,
            Self::Switch(switch) => switch.execute_request(&client).await?,
            Self::Merge(merge) => merge.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
            Self::Share(share) => share.execute_request(&client).await?,
            Self::Follow(follow) => follow.execute_request(&client).await?,
            Self::Collect(collect) => collect.execute_request(&client).await?,
            Self::Unfollow(unfollow) => unfollow.execute_request(&client).await?,
            Self::Submit(submit) => submit.execute_request(&client).await?,
        };

        let response: BookmarkResponse = serde_json::from_slice(&bytes)?;
        Ok(BookmarkView::new(response).render().map(Into::into))
    }
}
