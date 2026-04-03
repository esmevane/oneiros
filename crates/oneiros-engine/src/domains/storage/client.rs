use crate::*;

pub struct StorageClient<'a> {
    client: &'a Client,
}

impl<'a> StorageClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn upload(&self, request: &UploadStorage) -> Result<StorageResponse, ClientError> {
        self.client.post("/storage", request).await
    }

    pub async fn list(&self, request: &ListStorage) -> Result<StorageResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset
        );
        self.client.get(&format!("/storage?{query}")).await
    }

    pub async fn show(&self, request: &GetStorage) -> Result<StorageResponse, ClientError> {
        let ref_key = StorageRef::encode(&request.key);
        self.client.get(&format!("/storage/{ref_key}")).await
    }

    pub async fn remove(&self, request: &RemoveStorage) -> Result<StorageResponse, ClientError> {
        let ref_key = StorageRef::encode(&request.key);
        self.client.delete(&format!("/storage/{ref_key}")).await
    }
}
