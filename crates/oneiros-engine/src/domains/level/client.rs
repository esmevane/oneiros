use crate::*;

pub(crate) struct LevelClient<'a> {
    client: &'a Client,
}

impl<'a> LevelClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, set: &SetLevel) -> Result<LevelResponse, ClientError> {
        self.client.put(&format!("/levels/{}", set.name), set).await
    }

    pub(crate) async fn get(&self, name: &LevelName) -> Result<LevelResponse, ClientError> {
        self.client.get(&format!("/levels/{name}")).await
    }

    pub(crate) async fn list(&self, request: &ListLevels) -> Result<LevelResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/levels?{query}")).await
    }

    pub(crate) async fn remove(&self, name: &LevelName) -> Result<LevelResponse, ClientError> {
        self.client.delete(&format!("/levels/{name}")).await
    }
}
