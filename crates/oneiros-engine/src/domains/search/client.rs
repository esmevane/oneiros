use super::responses::SearchResponse;
use crate::client::{Client, ClientError};

pub struct SearchClient<'a> {
    client: &'a Client,
}

impl<'a> SearchClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn search(
        &self,
        query: &str,
        agent: Option<&str>,
    ) -> Result<SearchResponse, ClientError> {
        let path = match agent {
            Some(a) => format!("/search?q={query}&agent={a}"),
            None => format!("/search?q={query}"),
        };

        self.client.get(&path).await
    }
}
