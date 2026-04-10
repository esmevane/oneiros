use crate::*;

pub struct BookmarkClient<'a> {
    client: &'a Client,
}

impl<'a> BookmarkClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        brain: &BrainName,
        request: &CreateBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client
            .post(&format!("/brains/{brain}/bookmarks"), request)
            .await
    }

    pub async fn switch(
        &self,
        brain: &BrainName,
        request: &SwitchBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client
            .post(
                &format!("/brains/{brain}/bookmarks/{}/switch", request.name),
                &(),
            )
            .await
    }

    pub async fn merge(
        &self,
        brain: &BrainName,
        request: &MergeBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client
            .post(
                &format!("/brains/{brain}/bookmarks/{}/merge", request.source),
                &(),
            )
            .await
    }

    pub async fn list(
        &self,
        brain: &BrainName,
        request: &ListBookmarks,
    ) -> Result<BookmarkResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client
            .get(&format!("/brains/{brain}/bookmarks?{query}"))
            .await
    }

    /// Share a bookmark and receive the minted ticket and composed URI.
    pub async fn share(
        &self,
        brain: &BrainName,
        request: &ShareBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        let body = serde_json::json!({ "actor_id": request.actor_id });
        self.client
            .post(
                &format!("/brains/{brain}/bookmarks/{}/share", request.name),
                &body,
            )
            .await
    }

    /// Follow a bookmark via a URI. Parses the URI, ensures any
    /// referenced Peer is known, and creates a local Follow record.
    pub async fn follow(
        &self,
        brain: &BrainName,
        request: &FollowBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client
            .post(&format!("/brains/{brain}/bookmarks/follow"), request)
            .await
    }

    /// Collect events from a followed bookmark's source and apply them
    /// to the local bookmark. For Peer follows this opens an iroh
    /// connection and runs the sync protocol.
    pub async fn collect(
        &self,
        brain: &BrainName,
        request: &CollectBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client
            .post(
                &format!("/brains/{brain}/bookmarks/{}/collect", request.name),
                &(),
            )
            .await
    }

    /// Remove a follow from a bookmark. Previously collected events
    /// remain in place; only the remote binding is severed.
    pub async fn unfollow(
        &self,
        brain: &BrainName,
        request: &UnfollowBookmark,
    ) -> Result<BookmarkResponse, ClientError> {
        self.client
            .post(
                &format!("/brains/{brain}/bookmarks/{}/unfollow", request.name),
                &(),
            )
            .await
    }
}
