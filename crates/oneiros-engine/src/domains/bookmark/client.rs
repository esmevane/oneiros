use crate::*;

pub struct BookmarkClient<'a> {
    client: &'a Client,
}

impl<'a> BookmarkClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, creation: &CreateBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks", creation).await
    }

    pub async fn switch(&self, switch: &SwitchBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/switch", switch).await
    }

    pub async fn merge(&self, merge: &MergeBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/merge", merge).await
    }

    pub async fn list(&self, listing: &ListBookmarks) -> Result<BookmarkResponse, ClientError> {
        let ListBookmarks::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/bookmarks?{query}")).await
    }

    pub async fn share(&self, share: &ShareBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/share", share).await
    }

    pub async fn follow(&self, follow: &FollowBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/follow", follow).await
    }

    pub async fn collect(
        &self,
        collect: &CollectBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/collect", collect).await
    }

    pub async fn unfollow(
        &self,
        unfollow: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/unfollow", unfollow).await
    }
}
