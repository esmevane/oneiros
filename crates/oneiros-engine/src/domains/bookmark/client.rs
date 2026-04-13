use crate::*;

pub(crate) struct BookmarkClient<'a> {
    client: &'a Client,
}

impl<'a> BookmarkClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(&self, request: &CreateBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks", request).await
    }

    pub(crate) async fn switch(&self, request: &SwitchBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/switch", request).await
    }

    pub(crate) async fn merge(&self, request: &MergeBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/merge", request).await
    }

    pub(crate) async fn list(&self, request: &ListBookmarks) -> Result<BookmarkResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/bookmarks?{query}")).await
    }

    pub(crate) async fn share(&self, request: &ShareBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/share", request).await
    }

    pub(crate) async fn follow(&self, request: &FollowBookmark) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/follow", request).await
    }

    pub(crate) async fn collect(
        &self,
        request: &CollectBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/collect", request).await
    }

    pub(crate) async fn unfollow(
        &self,
        request: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client.post("/bookmarks/unfollow", request).await
    }
}
