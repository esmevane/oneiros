use crate::*;

pub struct SearchClient<'a> {
    client: &'a Client,
}

impl<'a> SearchClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, request: &SearchQuery) -> Result<SearchResponse, ClientError> {
        let details = request.current()?;
        let path = match &details.agent {
            Some(a) => format!("/search?query={}&agent={a}", details.query),
            None => format!("/search?query={}", details.query),
        };

        self.client.get(&path).await
    }
}
