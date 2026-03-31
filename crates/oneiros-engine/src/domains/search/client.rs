use crate::*;

pub struct SearchClient<'a> {
    client: &'a Client,
}

impl<'a> SearchClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, request: &SearchQuery) -> Result<SearchResponse, ClientError> {
        let path = match &request.agent {
            Some(a) => format!("/search?query={}&agent={a}", request.query),
            None => format!("/search?query={}", request.query),
        };

        self.client.get(&path).await
    }
}
