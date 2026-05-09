//! HTTP client for the follow domain.

use crate::*;

pub(crate) struct FollowClient<'a> {
    client: &'a Client,
}

impl<'a> FollowClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn get(&self, lookup: &GetFollow) -> Result<FollowResponse, ClientError> {
        let GetFollow::V1(lookup) = lookup;
        self.client.get(&format!("/follows/{}", lookup.key)).await
    }

    pub(crate) async fn list(&self, listing: &ListFollows) -> Result<FollowResponse, ClientError> {
        let ListFollows::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/follows?{query}")).await
    }
}
