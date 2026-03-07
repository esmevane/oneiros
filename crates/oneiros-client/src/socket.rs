use http_body_util::{BodyExt, Full};
use hyper::Request;
use hyper::body::Bytes;
use hyper::client::conn::http1;
use hyper_util::rt::TokioIo;
use oneiros_model::Token;
use oneiros_trust::SecureClient;
use rustls_pki_types::ServerName;
use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use crate::*;

/// Combined supertrait for type-erased async IO, covering both plain TCP and
/// TLS streams.
trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}

/// Type-erased async IO — covers both plain TCP and TLS streams.
type DynIo = Box<dyn AsyncReadWrite>;

pub struct SocketClient {
    addr: SocketAddr,
    hostname: String,
    tls: Option<SecureClient>,
}

impl SocketClient {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            hostname: addr.ip().to_string(),
            addr,
            tls: None,
        }
    }

    pub fn with_tls(addr: SocketAddr, hostname: impl Into<String>, tls: SecureClient) -> Self {
        Self {
            addr,
            hostname: hostname.into(),
            tls: Some(tls),
        }
    }

    /// Establish a connection, optionally upgrading to TLS.
    ///
    /// Plain connections are returned as-is; TLS connections are wrapped with
    /// a [`TlsConnector`] using the hostname stored in `self.hostname` as the
    /// SNI server name.
    async fn connect(&self) -> Result<TokioIo<DynIo>, Error> {
        let stream = TcpStream::connect(self.addr).await?;

        let io: DynIo = match &self.tls {
            None => Box::new(stream),
            Some(secure) => {
                let connector = TlsConnector::from(secure.client_config().clone());
                let server_name = ServerName::try_from(self.hostname.clone())
                    .map_err(|_| Error::InvalidServerName)?;
                let tls_stream = connector.connect(server_name, stream).await?;
                Box::new(tls_stream)
            }
        };

        Ok(TokioIo::new(io))
    }

    pub(crate) async fn request(
        &self,
        method: impl AsRef<str>,
        uri: impl AsRef<str>,
        body: impl Into<Bytes>,
    ) -> Result<(u16, Vec<u8>), Error> {
        let io = self.connect().await?;
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
        let io = self.connect().await?;
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
        let io = self.connect().await?;
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
