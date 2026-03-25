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
        name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    ) -> Result<AgentResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            name: AgentName,
            persona: PersonaName,
            description: Description,
            prompt: Prompt,
        }

        self.client
            .post(
                "/agents",
                &Body {
                    name,
                    persona,
                    description,
                    prompt,
                },
            )
            .await
    }

    pub async fn list(&self) -> Result<AgentResponse, ClientError> {
        self.client.get("/agents").await
    }

    pub async fn get(&self, name: &AgentName) -> Result<AgentResponse, ClientError> {
        self.client.get(&format!("/agents/{name}")).await
    }

    pub async fn update(
        &self,
        name: &AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    ) -> Result<AgentResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            persona: PersonaName,
            description: Description,
            prompt: Prompt,
        }

        self.client
            .put(
                &format!("/agents/{name}"),
                &Body {
                    persona,
                    description,
                    prompt,
                },
            )
            .await
    }

    pub async fn remove(&self, name: &AgentName) -> Result<AgentResponse, ClientError> {
        self.client.delete(&format!("/agents/{name}")).await
    }
}
