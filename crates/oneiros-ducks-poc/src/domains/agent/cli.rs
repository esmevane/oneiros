//! Agent CLI driving adapter — translates CLI through HTTP (remote calls).

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use oneiros_model::*;
use tower::ServiceExt;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("HTTP {0}: {1}")]
    Status(u16, String),

    #[error("Request error: {0}")]
    Request(String),

    #[error("Body error: {0}")]
    Body(String),

    #[error("Deserialization error: {0}")]
    Deserialize(String),
}

/// Remote agent client — dispatches through HTTP router.
///
/// In production this would use reqwest to a real server.
/// For the POC, it wraps the in-memory router via tower::oneshot.
#[derive(Clone)]
pub struct RemoteAgents {
    router: Router,
}

impl RemoteAgents {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    async fn request(&self, req: Request<Body>) -> Result<(u16, Vec<u8>), CliError> {
        let response = self
            .router
            .clone()
            .oneshot(req)
            .await
            .map_err(|e| CliError::Request(e.to_string()))?;
        let status = response.status().as_u16();
        let bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| CliError::Body(e.to_string()))?
            .to_bytes()
            .to_vec();
        if status >= 400 {
            return Err(CliError::Status(
                status,
                String::from_utf8_lossy(&bytes).into_owned(),
            ));
        }
        Ok((status, bytes))
    }

    pub async fn create(&self, req: CreateAgentRequest) -> Result<AgentResponses, CliError> {
        let body = serde_json::to_vec(&req).map_err(|e| CliError::Request(e.to_string()))?;
        let request = Request::builder()
            .method("POST")
            .uri("/agents")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .map_err(|e| CliError::Request(e.to_string()))?;
        let (_, bytes) = self.request(request).await?;
        serde_json::from_slice(&bytes).map_err(|e| CliError::Deserialize(e.to_string()))
    }

    pub async fn list(&self) -> Result<AgentResponses, CliError> {
        let request = Request::builder()
            .uri("/agents")
            .body(Body::empty())
            .map_err(|e| CliError::Request(e.to_string()))?;
        let (_, bytes) = self.request(request).await?;
        serde_json::from_slice(&bytes).map_err(|e| CliError::Deserialize(e.to_string()))
    }

    pub async fn get(&self, name: &AgentName) -> Result<AgentResponses, CliError> {
        let request = Request::builder()
            .uri(format!("/agents/{name}"))
            .body(Body::empty())
            .map_err(|e| CliError::Request(e.to_string()))?;
        let (_, bytes) = self.request(request).await?;
        serde_json::from_slice(&bytes).map_err(|e| CliError::Deserialize(e.to_string()))
    }
}
