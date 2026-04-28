use crate::*;

pub struct LevelClient<'a> {
    client: &'a Client,
}

impl<'a> LevelClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, setting: &SetLevel) -> Result<LevelResponse, ClientError> {
        let SetLevel::V1(body) = setting;
        self.client
            .put(&format!("/levels/{}", body.name), setting)
            .await
    }

    pub async fn get(&self, lookup: &GetLevel) -> Result<LevelResponse, ClientError> {
        let GetLevel::V1(lookup) = lookup;
        self.client.get(&format!("/levels/{}", lookup.key)).await
    }

    pub async fn list(&self, listing: &ListLevels) -> Result<LevelResponse, ClientError> {
        let ListLevels::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/levels?{query}")).await
    }

    pub async fn remove(&self, removal: &RemoveLevel) -> Result<LevelResponse, ClientError> {
        let RemoveLevel::V1(removal) = removal;
        self.client
            .delete(&format!("/levels/{}", removal.name))
            .await
    }
}
