use crate::*;

pub struct CognitionClient<'a> {
    client: &'a Client,
}

impl<'a> CognitionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(
        &self,
        agent: AgentName,
        texture: TextureName,
        content: Content,
    ) -> Result<CognitionResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            agent: AgentName,
            texture: TextureName,
            content: Content,
        }

        self.client
            .post(
                "/cognitions",
                &Body {
                    agent,
                    texture,
                    content,
                },
            )
            .await
    }

    pub async fn list(
        &self,
        agent: Option<&AgentName>,
        texture: Option<&TextureName>,
    ) -> Result<CognitionResponse, ClientError> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(a) = agent {
            params.push(("agent", a.as_str()));
        }
        if let Some(t) = texture {
            params.push(("texture", t.as_str()));
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

    pub async fn get(&self, id: &CognitionId) -> Result<CognitionResponse, ClientError> {
        self.client.get(&format!("/cognitions/{id}")).await
    }
}
