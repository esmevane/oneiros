use crate::*;

pub struct AgentClient<'a> {
    client: &'a Client,
}

impl<'a> AgentClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, creation: &CreateAgent) -> Result<AgentResponse, ClientError> {
        self.client.post("/agents", creation).await
    }

    pub async fn list(&self, listing: &ListAgents) -> Result<AgentResponse, ClientError> {
        let ListAgents::V1(listing) = listing;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(query) = &listing.query {
            params.push(("query", query.clone()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");
        self.client.get(&format!("/agents?{query}")).await
    }

    pub async fn get(&self, lookup: &GetAgent) -> Result<AgentResponse, ClientError> {
        let GetAgent::V1(lookup) = lookup;
        self.client.get(&format!("/agents/{}", lookup.key)).await
    }

    pub async fn update(&self, update: &UpdateAgent) -> Result<AgentResponse, ClientError> {
        let UpdateAgent::V1(body) = update;
        self.client
            .put(&format!("/agents/{name}", name = body.name), update)
            .await
    }

    pub async fn remove(&self, removal: &RemoveAgent) -> Result<AgentResponse, ClientError> {
        let RemoveAgent::V1(removal) = removal;
        self.client
            .delete(&format!("/agents/{}", removal.name))
            .await
    }
}
