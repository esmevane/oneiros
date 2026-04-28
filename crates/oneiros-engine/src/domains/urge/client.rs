use crate::*;

pub struct UrgeClient<'a> {
    client: &'a Client,
}

impl<'a> UrgeClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, setting: &SetUrge) -> Result<UrgeResponse, ClientError> {
        let SetUrge::V1(body) = setting;
        self.client
            .put(&format!("/urges/{}", body.name), setting)
            .await
    }

    pub async fn get(&self, lookup: &GetUrge) -> Result<UrgeResponse, ClientError> {
        let GetUrge::V1(lookup) = lookup;
        self.client.get(&format!("/urges/{}", lookup.key)).await
    }

    pub async fn list(&self, listing: &ListUrges) -> Result<UrgeResponse, ClientError> {
        let ListUrges::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/urges?{query}")).await
    }

    pub async fn remove(&self, removal: &RemoveUrge) -> Result<UrgeResponse, ClientError> {
        let RemoveUrge::V1(removal) = removal;
        self.client
            .delete(&format!("/urges/{}", removal.name))
            .await
    }
}
