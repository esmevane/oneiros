use axum::body::Body;
use axum::http::Request;
use axum::Router;
use http_body_util::BodyExt;
use oneiros_model::*;
use oneiros_resource::{Fulfill, Resource};
use tower::ServiceExt;

use crate::resource_agent::Agent;
use crate::resource_level::Level;

/// A scope that fulfills requests by making HTTP calls.
///
/// This is the mirror of ProjectScope: where ProjectScope fulfills via
/// database operations, HttpScope fulfills via HTTP requests. Same trait,
/// different backend — this is the Service<Backend> shape.
///
/// For the POC, it wraps an axum Router directly (via tower::Service)
/// instead of making real TCP connections. The shape is identical to
/// what a real HTTP client would look like.
#[derive(Clone)]
pub struct HttpScope {
    router: Router,
}

impl HttpScope {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    async fn get(&self, uri: &str) -> Result<Vec<u8>, HttpScopeError> {
        let request = Request::builder()
            .uri(uri)
            .body(Body::empty())
            .map_err(|e| HttpScopeError::Request(e.to_string()))?;

        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .map_err(|e| HttpScopeError::Transport(e.to_string()))?;

        let status = response.status();
        let bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| HttpScopeError::Body(e.to_string()))?
            .to_bytes()
            .to_vec();

        if !status.is_success() {
            return Err(HttpScopeError::Status(status.as_u16(), String::from_utf8_lossy(&bytes).into_owned()));
        }

        Ok(bytes)
    }

    async fn send(
        &self,
        method: &str,
        uri: &str,
        body: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, HttpScopeError> {
        let mut builder = Request::builder().method(method).uri(uri);

        if body.is_some() {
            builder = builder.header("content-type", "application/json");
        }

        let request = builder
            .body(match body {
                Some(b) => Body::from(b),
                None => Body::empty(),
            })
            .map_err(|e| HttpScopeError::Request(e.to_string()))?;

        let response = self
            .router
            .clone()
            .oneshot(request)
            .await
            .map_err(|e| HttpScopeError::Transport(e.to_string()))?;

        let status = response.status();
        let bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| HttpScopeError::Body(e.to_string()))?
            .to_bytes()
            .to_vec();

        if !status.is_success() {
            return Err(HttpScopeError::Status(status.as_u16(), String::from_utf8_lossy(&bytes).into_owned()));
        }

        Ok(bytes)
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, HttpScopeError> {
        serde_json::from_slice(bytes).map_err(|e| HttpScopeError::Deserialize(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HttpScopeError {
    #[error("Request build error: {0}")]
    Request(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Body read error: {0}")]
    Body(String),

    #[error("HTTP {0}: {1}")]
    Status(u16, String),

    #[error("Serialization error: {0}")]
    Serialize(String),

    #[error("Deserialization error: {0}")]
    Deserialize(String),
}

// ── Fulfill<Agent> for HttpScope ────────────────────────────────────
//
// The remote backend: same trait as ProjectScope, but fulfills by
// making HTTP calls instead of database operations.

impl Fulfill<Agent> for HttpScope {
    type Error = HttpScopeError;

    async fn fulfill(&self, request: AgentRequests) -> Result<AgentResponses, Self::Error> {
        match request {
            AgentRequests::CreateAgent(req) => {
                let body = serde_json::to_vec(&req)
                    .map_err(|e| HttpScopeError::Serialize(e.to_string()))?;
                let bytes = self.send("POST", "/agents", Some(body)).await?;
                self.parse(&bytes)
            }
            AgentRequests::ListAgents(_) => {
                let bytes = self.get("/agents").await?;
                self.parse(&bytes)
            }
            AgentRequests::GetAgent(req) => {
                let bytes = self.get(&format!("/agents/{}", req.name)).await?;
                self.parse(&bytes)
            }
            AgentRequests::UpdateAgent(req) => {
                let body = serde_json::to_vec(&req)
                    .map_err(|e| HttpScopeError::Serialize(e.to_string()))?;
                let bytes = self
                    .send("PUT", &format!("/agents/{}", req.name), Some(body))
                    .await?;
                self.parse(&bytes)
            }
            AgentRequests::RemoveAgent(req) => {
                self.send("DELETE", &format!("/agents/{}", req.name), None)
                    .await?;
                Ok(AgentResponses::AgentRemoved)
            }
        }
    }
}

// ── Fulfill<Level> for HttpScope ────────────────────────────────────

impl Fulfill<Level> for HttpScope {
    type Error = HttpScopeError;

    async fn fulfill(&self, request: LevelRequests) -> Result<LevelResponses, Self::Error> {
        match request {
            LevelRequests::SetLevel(level) => {
                let body = serde_json::to_vec(&level)
                    .map_err(|e| HttpScopeError::Serialize(e.to_string()))?;
                let bytes = self
                    .send("PUT", &format!("/levels/{}", level.name), Some(body))
                    .await?;
                self.parse(&bytes)
            }
            LevelRequests::ListLevels(_) => {
                let bytes = self.get("/levels").await?;
                self.parse(&bytes)
            }
            LevelRequests::GetLevel(req) => {
                let bytes = self.get(&format!("/levels/{}", req.name)).await?;
                self.parse(&bytes)
            }
            LevelRequests::RemoveLevel(req) => {
                self.send("DELETE", &format!("/levels/{}", req.name), None)
                    .await?;
                Ok(LevelResponses::LevelRemoved)
            }
        }
    }
}
