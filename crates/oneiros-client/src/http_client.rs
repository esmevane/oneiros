use http_body_util::{BodyExt, Full};
use hyper::Request;
use hyper::body::Bytes;
use hyper::client::conn::http1;
use hyper_util::rt::TokioIo;
use std::path::{Path, PathBuf};
use tokio::net::UnixStream;

use crate::{
    BrainInfo, ConnectionError, CreateBrainRequest, Error, RequestError, ServiceClient,
    ServiceResponseError,
};

pub struct HttpClient {
    socket_path: PathBuf,
}

impl HttpClient {
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
        }
    }

    async fn request(
        &self,
        method: &str,
        uri: &str,
        body: Option<Vec<u8>>,
    ) -> Result<(u16, Vec<u8>), Error> {
        let stream = UnixStream::connect(&self.socket_path).await?;

        let io = TokioIo::new(stream);

        let (mut sender, conn) = http1::handshake(io).await.map_err(ConnectionError::from)?;

        tokio::spawn(async move {
            if let Err(error) = conn.await {
                tracing::error!("Connection error: {error}");
            }
        });

        let body_bytes = body.unwrap_or_default();
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Full::new(Bytes::from(body_bytes)))
            .map_err(RequestError::from)?;

        let response = sender
            .send_request(request)
            .await
            .map_err(RequestError::from)?;

        let status = response.status().as_u16();
        let response_body = response
            .into_body()
            .collect()
            .await
            .map_err(RequestError::from)?
            .to_bytes()
            .to_vec();

        Ok((status, response_body))
    }
}

impl ServiceClient for HttpClient {
    async fn create_brain(&self, request: CreateBrainRequest) -> Result<BrainInfo, Error> {
        let body = serde_json::to_vec(&request)?;
        let (status, response_body) = self.request("POST", "/brains", Some(body)).await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ServiceResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(serde_json::from_slice(&response_body)?)
    }

    async fn health(&self) -> Result<(), Error> {
        let (status, response_body) = self.request("GET", "/health", None).await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ServiceResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(())
    }
}
