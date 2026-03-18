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
        name: impl Into<String>,
        content_type: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<StorageResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            name: String,
            content_type: String,
            data: Vec<u8>,
        }

        self.client
            .post(
                "/storage",
                &Body {
                    name: name.into(),
                    content_type: content_type.into(),
                    data,
                },
            )
            .await
    }

    pub async fn list(&self) -> Result<StorageResponse, ClientError> {
        self.client.get("/storage").await
    }

    pub async fn get(&self, id: &str) -> Result<StorageResponse, ClientError> {
        self.client.get(&format!("/storage/{id}")).await
    }

    pub async fn remove(&self, id: &str) -> Result<StorageResponse, ClientError> {
        self.client.delete(&format!("/storage/{id}")).await
    }
}
