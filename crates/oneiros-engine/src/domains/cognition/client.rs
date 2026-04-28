use crate::*;

pub struct CognitionClient<'a> {
    client: &'a Client,
}

impl<'a> CognitionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, addition: &AddCognition) -> Result<CognitionResponse, ClientError> {
        self.client.post("/cognitions", addition).await
    }

    pub async fn list(&self, listing: &ListCognitions) -> Result<CognitionResponse, ClientError> {
        let ListCognitions::V1(listing) = listing;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(texture_name) = &listing.texture {
            params.push(("texture", texture_name.to_string()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        self.client.get(&format!("/cognitions?{query}")).await
    }

    pub async fn get(&self, lookup: &GetCognition) -> Result<CognitionResponse, ClientError> {
        let GetCognition::V1(lookup) = lookup;
        self.client
            .get(&format!("/cognitions/{}", lookup.key))
            .await
    }
}
