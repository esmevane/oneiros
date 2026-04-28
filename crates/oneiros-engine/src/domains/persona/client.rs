use crate::*;

pub struct PersonaClient<'a> {
    client: &'a Client,
}

impl<'a> PersonaClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, setting: &SetPersona) -> Result<PersonaResponse, ClientError> {
        let SetPersona::V1(body) = setting;
        self.client
            .put(&format!("/personas/{}", body.name), setting)
            .await
    }

    pub async fn get(&self, lookup: &GetPersona) -> Result<PersonaResponse, ClientError> {
        let GetPersona::V1(lookup) = lookup;
        self.client.get(&format!("/personas/{}", lookup.key)).await
    }

    pub async fn list(&self, listing: &ListPersonas) -> Result<PersonaResponse, ClientError> {
        let ListPersonas::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/personas?{query}")).await
    }

    pub async fn remove(&self, removal: &RemovePersona) -> Result<PersonaResponse, ClientError> {
        let RemovePersona::V1(removal) = removal;
        self.client
            .delete(&format!("/personas/{}", removal.name))
            .await
    }
}
