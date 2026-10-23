//! HTTP client infrastructure — shared base for typed request dispatch.

use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::*;

/// Client errors.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Server error: {status} — {body}")]
    Server { status: u16, body: String },

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error(transparent)]
    Upcast(#[from] UpcastError),
}

impl IntoResponse for ClientError {
    fn into_response(self) -> axum::response::Response {
        todo!(
            "this isn't unreachable - the client can be leveraged by a server, we need to impl this"
        )
    }
}

/// A configured HTTP client for communicating with the engine service.
#[derive(Debug, Clone)]
pub(crate) struct Client {
    http: reqwest::Client,
    base_url: String,
    /// A separate client carrying the host token, used for host-tier
    /// routes. Present only when the host key is available locally.
    host: Option<Box<Client>>,
}

impl Client {
    pub(crate) fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.as_ref().into(),
            host: None,
        }
    }

    /// The client to use for host-tier routes. Uses the host credential
    /// when one is available; otherwise falls back to this client, whose
    /// project token the server will refuse on host-tier routes.
    pub(crate) fn host_tier(&self) -> &Client {
        self.host.as_deref().unwrap_or(self)
    }

    pub(crate) fn with_bearer(
        base_url: impl Into<String>,
        bearer: &str,
    ) -> Result<Self, ClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let value = format!("Bearer {bearer}").parse().map_err(|_| {
            ClientError::InvalidRequest("token contains invalid header characters".into())
        })?;
        headers.insert(reqwest::header::AUTHORIZATION, value);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            base_url: base_url.into(),
            host: None,
        })
    }

    pub(crate) fn with_token(
        base_url: impl Into<String>,
        token: Token,
    ) -> Result<Self, ClientError> {
        Self::with_bearer(base_url, &token.to_string())
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Send a GET and return the response bytes.
    #[tracing::instrument(skip(self), fields(path = %path))]
    pub(crate) async fn get(&self, path: &str) -> Result<Vec<u8>, ClientError> {
        let resp = self.http.get(self.url(path)).send().await?;
        Self::handle_response(resp).await
    }

    /// Send a POST with JSON body and return the response bytes.
    #[tracing::instrument(skip(self, body), fields(path = %path))]
    pub(crate) async fn post<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<Vec<u8>, ClientError> {
        let resp = self.http.post(self.url(path)).json(body).send().await?;
        Self::handle_response(resp).await
    }

    /// Send a PUT with JSON body and return the response bytes.
    #[tracing::instrument(skip(self, body), fields(path = %path))]
    pub(crate) async fn put<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<Vec<u8>, ClientError> {
        let resp = self.http.put(self.url(path)).json(body).send().await?;
        Self::handle_response(resp).await
    }

    /// Send a DELETE and return the response bytes.
    #[tracing::instrument(skip(self), fields(path = %path))]
    pub(crate) async fn delete(&self, path: &str) -> Result<Vec<u8>, ClientError> {
        let resp = self.http.delete(self.url(path)).send().await?;
        Self::handle_response(resp).await
    }

    async fn handle_response(resp: reqwest::Response) -> Result<Vec<u8>, ClientError> {
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

        Ok(resp.bytes().await?.to_vec())
    }
}

impl FromConfig for Client {
    type Error = ClientError;

    /// Build a client from a config — token-aware. Uses the project token
    /// from disk when available; falls back to a host token (derived from
    /// the host key) so that bootstrap operations (host init, project
    /// create) can authenticate before any project exists.
    ///
    /// When the host key is available, host-tier routes always use the
    /// host token (see [`Client::host_tier`]), since project tokens are
    /// refused there.
    fn from_config(config: &Config) -> Result<Self, ClientError> {
        let host_token = HostKey::new(config.platform())
            .load()
            .ok()
            .flatten()
            .map(|secret| HostToken::generate(&secret).to_string());

        match (config.token(), host_token) {
            (Some(token), Some(host_token)) => {
                let mut client = Self::with_bearer(config.base_url(), &token.to_string())?;
                client.host = Some(Box::new(Self::with_bearer(config.base_url(), &host_token)?));
                Ok(client)
            }
            (Some(token), None) => Self::with_bearer(config.base_url(), &token.to_string()),
            (None, Some(host_token)) => Self::with_bearer(config.base_url(), &host_token),
            (None, None) => Ok(Self::new(config.base_url())),
        }
    }
}

/// A typed request that knows how to execute itself against a `Client`,
/// returning raw response bytes. Each impl chooses its verb, path, and body
/// shape; deserialization is the caller's concern.
pub(crate) trait ClientRequest {
    type Error: From<ClientError>;

    fn execute_request(
        &self,
        client: &Client,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>>;
}
