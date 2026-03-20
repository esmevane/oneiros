use crate::*;

pub struct ExperienceClient<'a> {
    client: &'a Client,
}

impl<'a> ExperienceClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        agent: AgentName,
        sensation: SensationName,
        description: Description,
    ) -> Result<ExperienceResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            agent: AgentName,
            sensation: SensationName,
            description: Description,
        }

        self.client
            .post(
                "/experiences",
                &Body {
                    agent,
                    sensation,
                    description,
                },
            )
            .await
    }

    pub async fn list(&self, agent: Option<&AgentName>) -> Result<ExperienceResponse, ClientError> {
        let path = match agent {
            Some(a) => format!("/experiences?agent={a}"),
            None => "/experiences".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &ExperienceId) -> Result<ExperienceResponse, ClientError> {
        self.client.get(&format!("/experiences/{id}")).await
    }

    pub async fn update_description(
        &self,
        id: &ExperienceId,
        description: Description,
    ) -> Result<ExperienceResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            description: Description,
        }

        self.client
            .put(
                &format!("/experiences/{id}/description"),
                &Body { description },
            )
            .await
    }

    pub async fn update_sensation(
        &self,
        id: &ExperienceId,
        sensation: SensationName,
    ) -> Result<ExperienceResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            sensation: SensationName,
        }

        self.client
            .put(
                &format!("/experiences/{id}/sensation"),
                &Body { sensation },
            )
            .await
    }
}
