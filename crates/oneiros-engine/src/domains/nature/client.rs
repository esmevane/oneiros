use crate::*;

pub(crate) struct NatureClient<'a> {
    client: &'a Client,
}

impl<'a> NatureClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, setting: &SetNature) -> Result<NatureResponse, ClientError> {
        let SetNature::V1(body) = setting;
        self.client
            .put(&format!("/natures/{}", body.name), setting)
            .await
    }

    pub(crate) async fn get(&self, lookup: &GetNature) -> Result<NatureResponse, ClientError> {
        let GetNature::V1(lookup) = lookup;
        self.client.get(&format!("/natures/{}", lookup.key)).await
    }

    pub(crate) async fn list(&self, listing: &ListNatures) -> Result<NatureResponse, ClientError> {
        let ListNatures::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/natures?{query}")).await
    }

    pub(crate) async fn remove(&self, removal: &RemoveNature) -> Result<NatureResponse, ClientError> {
        let RemoveNature::V1(removal) = removal;
        self.client
            .delete(&format!("/natures/{}", removal.name))
            .await
    }
}
