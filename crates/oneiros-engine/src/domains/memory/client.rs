use super::responses::MemoryResponse;
use crate::client::{Client, ClientError};

pub struct MemoryClient<'a> {
    client: &'a Client,
}

impl<'a> MemoryClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(
        &self,
        agent: impl Into<String>,
        level: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<MemoryResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            agent: String,
            level: String,
            content: String,
        }

        self.client
            .post(
                "/memories",
                &Body {
                    agent: agent.into(),
                    level: level.into(),
                    content: content.into(),
                },
            )
            .await
    }

    pub async fn list(&self, agent: Option<&str>) -> Result<MemoryResponse, ClientError> {
        let path = match agent {
            Some(a) => format!("/memories?agent={a}"),
            None => "/memories".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &str) -> Result<MemoryResponse, ClientError> {
        self.client.get(&format!("/memories/{id}")).await
    }
}
