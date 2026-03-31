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
        let path = match &request.entity {
            Some(e) => format!("/connections?entity={e}"),
            None => "/connections".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, request: &GetConnection) -> Result<ConnectionResponse, ClientError> {
        self.client
            .get(&format!("/connections/{}", request.id))
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
