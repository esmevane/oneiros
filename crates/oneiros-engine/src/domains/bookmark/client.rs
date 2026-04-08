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
}
