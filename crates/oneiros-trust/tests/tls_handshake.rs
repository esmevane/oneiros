//! Integration test: full TLS handshake using locally-generated CA certificates.
//!
//! This test validates the complete certificate pipeline:
//! CA generation → leaf issuance → ServerConfig → ClientConfig → actual TCP+TLS
//! handshake → data exchange.

use std::sync::Arc;

use oneiros_trust::{LocalCa, SecureClient, SecureServer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::test]
async fn local_ca_produces_working_tls_handshake() {
    let dir = tempfile::TempDir::new().unwrap();
    let trust_dir = dir.path().join("trust");
    let ca = LocalCa::init(&trust_dir).unwrap();
    let leaf = ca.issue_leaf("localhost").unwrap();

    // Build server config from the leaf cert.
    let server = SecureServer::local(&leaf).unwrap();
    let server_config = match server {
        SecureServer::Local(config) => config,
        _ => panic!("expected Local variant"),
    };

    // Build client config using SecureClient::local — trusts the local CA root.
    let client = SecureClient::local(&ca).unwrap();
    let client_config = Arc::clone(client.client_config());

    // Bind on a random port to avoid conflicts in parallel test runs.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Server task: accept one connection, send a greeting, then shut down cleanly.
    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let acceptor = tokio_rustls::TlsAcceptor::from(server_config);
        let mut tls = acceptor.accept(stream).await.unwrap();
        tls.write_all(b"hello from server").await.unwrap();
        tls.shutdown().await.unwrap();
    });

    // Client: connect, complete the TLS handshake, read until EOF.
    let connector = tokio_rustls::TlsConnector::from(client_config);
    let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let server_name = rustls::pki_types::ServerName::try_from("localhost").unwrap();
    let mut tls = connector.connect(server_name, stream).await.unwrap();

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf).await.unwrap();

    assert_eq!(buf, b"hello from server");

    server_handle.await.unwrap();
}
