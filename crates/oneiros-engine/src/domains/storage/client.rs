use crate::*;

pub struct StorageClient<'a> {
    client: &'a Client,
}

impl<'a> StorageClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn upload(&self, upload: &UploadStorage) -> Result<StorageResponse, ClientError> {
        self.client.post("/storage", upload).await
    }

    pub async fn list(&self, listing: &ListStorage) -> Result<StorageResponse, ClientError> {
        let ListStorage::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset
        );
        self.client.get(&format!("/storage?{query}")).await
    }

    pub async fn show(&self, lookup: &GetStorage) -> Result<StorageResponse, ClientError> {
        let GetStorage::V1(lookup) = lookup;
        let path = match &lookup.key {
            ResourceKey::Key(key) => StorageRef::encode(key).to_string(),
            ResourceKey::Ref(token) => token.to_string(),
        };
        self.client.get(&format!("/storage/{path}")).await
    }

    pub async fn remove(&self, removal: &RemoveStorage) -> Result<StorageResponse, ClientError> {
        let RemoveStorage::V1(removal) = removal;
        let ref_key = StorageRef::encode(&removal.key);
        self.client.delete(&format!("/storage/{ref_key}")).await
    }
}
