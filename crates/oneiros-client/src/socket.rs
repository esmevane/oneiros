use http_body_util::{BodyExt, Full};
use hyper::Request;
use hyper::body::Bytes;
use hyper::client::conn::http1;
use hyper_util::rt::TokioIo;
use oneiros_model::Token;
use std::net::SocketAddr;
use tokio::net::TcpStream;

use crate::*;

pub struct SocketClient {
    addr: SocketAddr,
}

impl SocketClient {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub(crate) async fn request(
        &self,
        method: impl AsRef<str>,
        uri: impl AsRef<str>,
        body: impl Into<Bytes>,
    ) -> Result<(u16, Vec<u8>), Error> {
        let stream = TcpStream::connect(self.addr).await?;
        let io = TokioIo::new(stream);
        let (mut sender, conn) = http1::handshake(io).await.map_err(ConnectionError::from)?;

        tokio::spawn(async move {
            if let Err(error) = conn.await {
                tracing::error!("Connection error: {error}");
            }
        });

        let request = Request::builder()
            .method(method.as_ref())
            .uri(uri.as_ref())
            .header("host", self.addr.to_string())
            .header("content-type", "application/json")
            .body(Full::new(body.into()))
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

    pub(crate) async fn authenticated_binary_request(
        &self,
        method: &str,
        uri: &str,
        token: &Token,
        body: Vec<u8>,
        extra_headers: &[(&str, &str)],
    ) -> Result<(u16, Vec<u8>), Error> {
        let stream = TcpStream::connect(self.addr).await?;
        let io = TokioIo::new(stream);
        let (mut sender, conn) = http1::handshake(io).await.map_err(ConnectionError::from)?;

        tokio::spawn(async move {
            if let Err(error) = conn.await {
                tracing::error!("Connection error: {error}");
            }
        });

        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("host", self.addr.to_string())
            .header("content-type", "application/octet-stream")
            .header("authorization", format!("Bearer {token}"));

        for (name, value) in extra_headers {
            builder = builder.header(*name, *value);
        }

        let request = builder
            .body(Full::new(Bytes::from(body)))
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

    pub(crate) async fn authenticated_request(
        &self,
        method: &str,
        uri: &str,
        token: &Token,
        body: Option<Vec<u8>>,
    ) -> Result<(u16, Vec<u8>), Error> {
        let stream = TcpStream::connect(self.addr).await?;
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
            .header("host", self.addr.to_string())
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {token}"))
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
