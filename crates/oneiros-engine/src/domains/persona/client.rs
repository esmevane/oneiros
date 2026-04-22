use crate::*;

pub struct PersonaClient<'a> {
    client: &'a Client,
}

impl<'a> PersonaClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, set: &SetPersona) -> Result<PersonaResponse, ClientError> {
        self.client
            .put(&format!("/personas/{}", set.name), set)
            .await
    }

    pub async fn get(&self, request: &GetPersona) -> Result<PersonaResponse, ClientError> {
        self.client.get(&format!("/personas/{}", request.key)).await
    }

    pub async fn list(&self, request: &ListPersonas) -> Result<PersonaResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/personas?{query}")).await
    }

    pub async fn remove(&self, name: &PersonaName) -> Result<PersonaResponse, ClientError> {
        self.client.delete(&format!("/personas/{name}")).await
    }
}
