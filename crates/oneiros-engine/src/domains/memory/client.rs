use crate::*;

pub struct MemoryClient<'a> {
    client: &'a Client,
}

impl<'a> MemoryClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, addition: &AddMemory) -> Result<MemoryResponse, ClientError> {
        self.client.post("/memories", addition).await
    }

    pub async fn list(&self, listing: &ListMemories) -> Result<MemoryResponse, ClientError> {
        let ListMemories::V1(listing) = listing;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(level_name) = &listing.level {
            params.push(("level", level_name.to_string()));
        }

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

        self.client.get(&format!("/memories?{query}")).await
    }

    pub async fn get(&self, lookup: &GetMemory) -> Result<MemoryResponse, ClientError> {
        let GetMemory::V1(lookup) = lookup;
        self.client.get(&format!("/memories/{}", lookup.key)).await
    }
}
