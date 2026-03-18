use super::responses::CognitionResponse;
use crate::client::{Client, ClientError};

pub struct CognitionClient<'a> {
    client: &'a Client,
}

impl<'a> CognitionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(
        &self,
        agent: impl Into<String>,
        texture: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<CognitionResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            agent: String,
            texture: String,
            content: String,
        }

        self.client
            .post(
                "/cognitions",
                &Body {
                    agent: agent.into(),
                    texture: texture.into(),
                    content: content.into(),
                },
            )
            .await
    }

    pub async fn list(
        &self,
        agent: Option<&str>,
        texture: Option<&str>,
    ) -> Result<CognitionResponse, ClientError> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(a) = agent {
            params.push(("agent", a));
        }
        if let Some(t) = texture {
            params.push(("texture", t));
        }

        let path = if params.is_empty() {
            "/cognitions".to_string()
        } else {
            let query = params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            format!("/cognitions?{query}")
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &str) -> Result<CognitionResponse, ClientError> {
        self.client.get(&format!("/cognitions/{id}")).await
    }
}
