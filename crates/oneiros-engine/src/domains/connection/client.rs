use crate::*;

pub(crate) struct ConnectionClient<'a> {
    client: &'a Client,
}

impl<'a> ConnectionClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        self.client.post("/connections", request).await
    }

    pub(crate) async fn list(&self, request: &ListConnections) -> Result<ConnectionResponse, ClientError> {
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

    pub(crate) async fn get(&self, request: &GetConnection) -> Result<ConnectionResponse, ClientError> {
        self.client
            .get(&format!("/connections/{}", request.id))
            .await
    }

    pub(crate) async fn remove(
        &self,
        request: &RemoveConnection,
    ) -> Result<ConnectionResponse, ClientError> {
        self.client
            .delete(&format!("/connections/{}", request.id))
            .await
    }
}
