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
        request: &CreateConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        self.client.post("/connections", request).await
    }

    pub async fn list(&self, request: &ListConnections) -> Result<ConnectionResponse, ClientError> {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(entity) = &request.entity {
            params.push(("entity", entity.to_string()));
        }

        params.push(("limit", request.filters.limit.to_string()));
        params.push(("offset", request.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        self.client.get(&format!("/connections?{query}")).await
    }

    pub async fn get(&self, request: &GetConnection) -> Result<ConnectionResponse, ClientError> {
        self.client
            .get(&format!("/connections/{}", request.key))
            .await
    }

    pub async fn remove(
        &self,
        request: &RemoveConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        self.client
            .delete(&format!("/connections/{}", request.id))
            .await
    }
}
