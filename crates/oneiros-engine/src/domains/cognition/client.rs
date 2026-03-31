use crate::*;

pub struct CognitionClient<'a> {
    client: &'a Client,
}

impl<'a> CognitionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, request: &AddCognition) -> Result<CognitionResponse, ClientError> {
        self.client.post("/cognitions", request).await
    }

    pub async fn list(&self, request: &ListCognitions) -> Result<CognitionResponse, ClientError> {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &request.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(texture_name) = &request.texture {
            params.push(("texture", texture_name.to_string()));
        }

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        let prefix = "/cognitions".to_string();
        let path = if query.is_empty() {
            prefix
        } else {
            format!("{prefix}?{query}")
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, request: &GetCognition) -> Result<CognitionResponse, ClientError> {
        self.client
            .get(&format!("/cognitions/{}", request.id))
            .await
    }
}
