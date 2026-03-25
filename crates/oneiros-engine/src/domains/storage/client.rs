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
        key: &StorageKey,
        description: &Description,
        data: Vec<u8>,
    ) -> Result<StorageResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body<'b> {
            key: &'b StorageKey,
            description: &'b Description,
            data: Vec<u8>,
        }

        self.client
            .post(
                "/storage",
                &Body {
                    key,
                    description,
                    data,
                },
            )
            .await
    }

    pub async fn list(&self) -> Result<StorageResponse, ClientError> {
        self.client.get("/storage").await
    }

    pub async fn show(&self, key: &StorageKey) -> Result<StorageResponse, ClientError> {
        let ref_key = StorageRef::encode(key);
        self.client.get(&format!("/storage/{ref_key}")).await
    }

    pub async fn remove(&self, key: &StorageKey) -> Result<StorageResponse, ClientError> {
        let ref_key = StorageRef::encode(key);
        self.client.delete(&format!("/storage/{ref_key}")).await
    }
}
