use super::responses::ExperienceResponse;
use crate::client::{Client, ClientError};

pub struct ExperienceClient<'a> {
    client: &'a Client,
}

impl<'a> ExperienceClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        agent: impl Into<String>,
        sensation: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ExperienceResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            agent: String,
            sensation: String,
            description: String,
        }

        self.client
            .post(
                "/experiences",
                &Body {
                    agent: agent.into(),
                    sensation: sensation.into(),
                    description: description.into(),
                },
            )
            .await
    }

    pub async fn list(&self, agent: Option<&str>) -> Result<ExperienceResponse, ClientError> {
        let path = match agent {
            Some(a) => format!("/experiences?agent={a}"),
            None => "/experiences".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &str) -> Result<ExperienceResponse, ClientError> {
        self.client.get(&format!("/experiences/{id}")).await
    }

    pub async fn update_description(
        &self,
        id: &str,
        description: impl Into<String>,
    ) -> Result<ExperienceResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            description: String,
        }

        self.client
            .put(
                &format!("/experiences/{id}/description"),
                &Body {
                    description: description.into(),
                },
            )
            .await
    }

    pub async fn update_sensation(
        &self,
        id: &str,
        sensation: impl Into<String>,
    ) -> Result<ExperienceResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            sensation: String,
        }

        self.client
            .put(
                &format!("/experiences/{id}/sensation"),
                &Body {
                    sensation: sensation.into(),
                },
            )
            .await
    }
}
