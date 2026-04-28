use crate::*;

pub struct ConnectionClient<'a> {
    client: &'a Client,
}

impl<'a> ConnectionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        creation: &CreateConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        self.client.post("/connections", creation).await
    }

    pub async fn list(&self, listing: &ListConnections) -> Result<ConnectionResponse, ClientError> {
        let ListConnections::V1(listing) = listing;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(entity) = &listing.entity {
            params.push(("entity", entity.to_string()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        self.client.get(&format!("/connections?{query}")).await
    }

    pub async fn get(&self, lookup: &GetConnection) -> Result<ConnectionResponse, ClientError> {
        let GetConnection::V1(lookup) = lookup;
        self.client
            .get(&format!("/connections/{}", lookup.key))
            .await
    }

    pub async fn remove(
        &self,
        removal: &RemoveConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        let RemoveConnection::V1(removal) = removal;
        self.client
            .delete(&format!("/connections/{}", removal.id))
            .await
    }
}
