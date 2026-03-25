//! CLI surface — dispatches through HTTP (remote actor handle).
//!
//! In production, CLI → HTTP client → HTTP server → Actor.
//! For the POC, CLI → tower::oneshot on in-memory Router → Actor.
//! Same code path, no TCP.

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use oneiros_model::*;
use tower::ServiceExt;

/// A remote agent handle — dispatches through HTTP.
///
/// This is the actor model's equivalent of HttpScope from the trait spike.
/// Instead of implementing a Fulfill trait, it sends HTTP requests to
/// the router (which dispatches to the actor through the registry).
#[derive(Clone)]
pub struct RemoteAgents {
    router: Router,
}

#[derive(Debug, thiserror::Error)]
pub enum RemoteError {
    #[error("HTTP {0}: {1}")]
    Status(u16, String),

    #[error("Request error: {0}")]
    Request(String),

    #[error("Body error: {0}")]
    Body(String),

    #[error("Deserialization error: {0}")]
    Deserialize(String),
}

impl RemoteAgents {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    async fn send_request(&self, request: Request<Body>) -> Result<(u16, Vec<u8>), RemoteError> {
        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .map_err(|e| RemoteError::Request(e.to_string()))?;

        let status = response.status().as_u16();
        let bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| RemoteError::Body(e.to_string()))?
            .to_bytes()
            .to_vec();

        if status >= 400 {
            return Err(RemoteError::Status(
                status,
                String::from_utf8_lossy(&bytes).into_owned(),
            ));
        }

        Ok((status, bytes))
    }

    pub async fn create(&self, request: CreateAgentRequest) -> Result<AgentResponses, RemoteError> {
        let body = serde_json::to_vec(&request).map_err(|e| RemoteError::Request(e.to_string()))?;
        let req = Request::builder()
            .method("POST")
            .uri("/agents")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .map_err(|e| RemoteError::Request(e.to_string()))?;

        let (_, bytes) = self.send_request(req).await?;
        serde_json::from_slice(&bytes).map_err(|e| RemoteError::Deserialize(e.to_string()))
    }

    pub async fn list(&self) -> Result<AgentResponses, RemoteError> {
        let req = Request::builder()
            .uri("/agents")
            .body(Body::empty())
            .map_err(|e| RemoteError::Request(e.to_string()))?;

        let (_, bytes) = self.send_request(req).await?;
        serde_json::from_slice(&bytes).map_err(|e| RemoteError::Deserialize(e.to_string()))
    }

    pub async fn get(&self, name: &AgentName) -> Result<AgentResponses, RemoteError> {
        let req = Request::builder()
            .uri(format!("/agents/{name}"))
            .body(Body::empty())
            .map_err(|e| RemoteError::Request(e.to_string()))?;

        let (_, bytes) = self.send_request(req).await?;
        serde_json::from_slice(&bytes).map_err(|e| RemoteError::Deserialize(e.to_string()))
    }
}
