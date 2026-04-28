use crate::*;

pub struct SensationClient<'a> {
    client: &'a Client,
}

impl<'a> SensationClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, setting: &SetSensation) -> Result<SensationResponse, ClientError> {
        let SetSensation::V1(body) = setting;
        self.client
            .put(&format!("/sensations/{}", body.name), setting)
            .await
    }

    pub async fn get(&self, lookup: &GetSensation) -> Result<SensationResponse, ClientError> {
        let GetSensation::V1(lookup) = lookup;
        self.client
            .get(&format!("/sensations/{}", lookup.key))
            .await
    }

    pub async fn list(&self, listing: &ListSensations) -> Result<SensationResponse, ClientError> {
        let ListSensations::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/sensations?{query}")).await
    }

    pub async fn remove(
        &self,
        removal: &RemoveSensation,
    ) -> Result<SensationResponse, ClientError> {
        let RemoveSensation::V1(removal) = removal;
        self.client
            .delete(&format!("/sensations/{}", removal.name))
            .await
    }
}
