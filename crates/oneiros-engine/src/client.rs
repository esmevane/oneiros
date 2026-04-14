//! HTTP client infrastructure — shared base for per-domain clients.

use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::*;

/// A configured HTTP client for communicating with the engine service.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

/// Client errors.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Server error: {status} — {body}")]
    Server { status: u16, body: String },

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        unreachable!()
    }
}

impl Client {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.as_ref().into(),
        }
    }

    pub fn with_token(base_url: impl Into<String>, token: Token) -> Result<Self, ClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let value = format!("Bearer {token}").parse().map_err(|_| {
            ClientError::InvalidRequest("token contains invalid header characters".into())
        })?;
        headers.insert(reqwest::header::AUTHORIZATION, value);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            base_url: base_url.into(),
        })
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Send a GET and deserialize the response.
    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ClientError> {
        let resp = self.http.get(self.url(path)).send().await?;
        Self::handle_response(resp).await
    }

    /// Send a POST with JSON body and deserialize the response.
    pub(crate) async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ClientError> {
        let resp = self.http.post(self.url(path)).json(body).send().await?;
        Self::handle_response(resp).await
    }

    /// Send a PUT with JSON body and deserialize the response.
    pub(crate) async fn put<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ClientError> {
        let resp = self.http.put(self.url(path)).json(body).send().await?;
        Self::handle_response(resp).await
    }

    /// Send a DELETE and deserialize the response.
    pub(crate) async fn delete<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ClientError> {
        let resp = self.http.delete(self.url(path)).send().await?;
        Self::handle_response(resp).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, ClientError> {
        let status = resp.status();

        if status == StatusCode::NOT_FOUND {
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::NotFound(body));
        }

        if status == StatusCode::CONFLICT {
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Conflict(body));
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Server {
                status: status.as_u16(),
                body,
            });
        }

        let body = resp.text().await?;
        let parsed = serde_json::from_str(&body)?;
        Ok(parsed)
    }
}
