use crate::*;

pub struct MemoryClient<'a> {
    client: &'a Client,
}

impl<'a> MemoryClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(
        &self,
        agent: AgentName,
        level: LevelName,
        content: Content,
    ) -> Result<MemoryResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            agent: AgentName,
            level: LevelName,
            content: Content,
        }

        self.client
            .post(
                "/memories",
                &Body {
                    agent,
                    level,
                    content,
                },
            )
            .await
    }

    pub async fn list(&self, agent: Option<&AgentName>) -> Result<MemoryResponse, ClientError> {
        let path = match agent {
            Some(a) => format!("/memories?agent={a}"),
            None => "/memories".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &MemoryId) -> Result<MemoryResponse, ClientError> {
        self.client.get(&format!("/memories/{id}")).await
    }
}
