use crate::*;

pub struct AgentClient<'a> {
    client: &'a Client,
}

impl<'a> AgentClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        name: impl Into<String>,
        persona: impl Into<String>,
        description: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<AgentResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            name: String,
            persona: String,
            description: String,
            prompt: String,
        }

        self.client
            .post(
                "/agents",
                &Body {
                    name: name.into(),
                    persona: persona.into(),
                    description: description.into(),
                    prompt: prompt.into(),
                },
            )
            .await
    }

    pub async fn list(&self) -> Result<AgentResponse, ClientError> {
        self.client.get("/agents").await
    }

    pub async fn get(&self, name: &str) -> Result<AgentResponse, ClientError> {
        self.client.get(&format!("/agents/{name}")).await
    }

    pub async fn update(
        &self,
        name: &str,
        persona: impl Into<String>,
        description: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Result<AgentResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            persona: String,
            description: String,
            prompt: String,
        }

        self.client
            .put(
                &format!("/agents/{name}"),
                &Body {
                    persona: persona.into(),
                    description: description.into(),
                    prompt: prompt.into(),
                },
            )
            .await
    }

    pub async fn remove(&self, name: &str) -> Result<AgentResponse, ClientError> {
        self.client.delete(&format!("/agents/{name}")).await
    }
}
