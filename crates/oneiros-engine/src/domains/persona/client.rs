use crate::*;

pub(crate) struct PersonaClient<'a> {
    client: &'a Client,
}

impl<'a> PersonaClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, set: &SetPersona) -> Result<PersonaResponse, ClientError> {
        self.client
            .put(&format!("/personas/{}", set.name), set)
            .await
    }

    pub(crate) async fn get(&self, name: &PersonaName) -> Result<PersonaResponse, ClientError> {
        self.client.get(&format!("/personas/{name}")).await
    }

    pub(crate) async fn list(&self, request: &ListPersonas) -> Result<PersonaResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/personas?{query}")).await
    }

    pub(crate) async fn remove(&self, name: &PersonaName) -> Result<PersonaResponse, ClientError> {
        self.client.delete(&format!("/personas/{name}")).await
    }
}
