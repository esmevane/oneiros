use crate::*;

pub(crate) struct BookmarkClient<'a> {
    client: &'a Client,
}

impl<'a> BookmarkClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(&self, creation: &CreateBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks", creation).await
    }

    pub(crate) async fn switch(&self, switch: &SwitchBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/switch", switch).await
    }

    pub(crate) async fn merge(&self, merge: &MergeBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/merge", merge).await
    }

    pub(crate) async fn list(&self, listing: &ListBookmarks) -> Result<BookmarkResponse, ClientError> {
        let ListBookmarks::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/bookmarks?{query}")).await
    }

    pub(crate) async fn share(&self, share: &ShareBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/share", share).await
    }

    pub(crate) async fn follow(&self, follow: &FollowBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/follow", follow).await
    }

    pub(crate) async fn collect(
        &self,
        collect: &CollectBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/collect", collect).await
    }

    pub(crate) async fn unfollow(
        &self,
        unfollow: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/unfollow", unfollow).await
    }
}
