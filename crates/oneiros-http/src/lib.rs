mod error;
mod extractors;
mod handlers;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub use error::Error;
pub use extractors::*;
pub use routes::router;

// Re-export service types that consumers need.
pub use oneiros_service::{self, ServiceState, projections};

use oneiros_trust::SecureServer;

/// Start the HTTP service, listening on the given TCP address.
///
/// Pass `None` for `tls` to use plain TCP (existing behaviour). Pass
/// `Some(SecureServer::Local(_))` for self-signed local CA TLS, or
/// `Some(SecureServer::Acme(_))` for ACME-managed TLS via Let's Encrypt.
///
/// This function blocks until the server is shut down via SIGINT or SIGTERM.
pub async fn serve(
    state: Arc<ServiceState>,
    addr: SocketAddr,
    tls: Option<SecureServer>,
) -> Result<(), std::io::Error> {
    let app = routes::router(state);

    match tls {
        None => {
            let listener = TcpListener::bind(addr).await?;
            tracing::info!("Service listening on {addr}");
            axum::serve(listener, app.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
        }
        Some(SecureServer::Local(config)) => {
            tracing::info!("Service listening on {addr} (TLS, local CA)");
            let tls_config = axum_server::tls_rustls::RustlsConfig::from_config(config);
            let handle = axum_server::Handle::new();
            tokio::spawn(shutdown_via_handle(handle.clone()));
            axum_server::bind_rustls(addr, tls_config)
                .handle(handle)
                .serve(app.into_make_service())
                .await
        }
        Some(SecureServer::Acme(acme_state)) => {
            tracing::info!("Service listening on {addr} (TLS, ACME)");
            let (mut acme_stream, acceptor) = acme_state.into_parts();

            tokio::spawn(async move {
                use tokio_stream::StreamExt;
                while let Some(event) = acme_stream.next().await {
                    match event {
                        Ok(ok) => tracing::debug!("ACME event: {:?}", ok),
                        Err(err) => tracing::error!("ACME error: {:?}", err),
                    }
                }
            });

            let handle = axum_server::Handle::new();
            tokio::spawn(shutdown_via_handle(handle.clone()));
            axum_server::bind(addr)
                .acceptor(acceptor)
                .handle(handle)
                .serve(app.into_make_service())
                .await
        }
    }
}

/// Drive graceful shutdown for an `axum_server::Handle` by waiting for SIGINT
/// or SIGTERM and then requesting a graceful shutdown with no deadline.
async fn shutdown_via_handle(handle: axum_server::Handle<SocketAddr>) {
    shutdown_signal().await;
    handle.graceful_shutdown(None);
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");

        tokio::select! {
            _ = ctrl_c => { tracing::info!("Received SIGINT, shutting down"); }
            _ = sigterm.recv() => { tracing::info!("Received SIGTERM, shutting down"); }
        }
    }

    #[cfg(not(unix))]
    {
        ctrl_c.await.ok();
        tracing::info!("Received SIGINT, shutting down");
    }
}
