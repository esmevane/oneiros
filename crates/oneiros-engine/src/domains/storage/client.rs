use crate::*;

pub struct StorageClient<'a> {
    client: &'a Client,
}

impl<'a> StorageClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn upload(
        &self,
        name: &StorageName,
        content_type: &Label,
        data: Vec<u8>,
    ) -> Result<StorageResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body<'b> {
            name: &'b StorageName,
            content_type: &'b Label,
            data: Vec<u8>,
        }

        self.client
            .post(
                "/storage",
                &Body {
                    name,
                    content_type,
                    data,
                },
            )
            .await
    }

    pub async fn list(&self) -> Result<StorageResponse, ClientError> {
        self.client.get("/storage").await
    }

    pub async fn get(&self, id: &StorageId) -> Result<StorageResponse, ClientError> {
        self.client.get(&format!("/storage/{id}")).await
    }

    pub async fn remove(&self, id: &StorageId) -> Result<StorageResponse, ClientError> {
        self.client.delete(&format!("/storage/{id}")).await
    }
}
