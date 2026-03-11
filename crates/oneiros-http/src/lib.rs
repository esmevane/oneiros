mod error;
mod handlers;
mod mcp_session;
mod oneiros_context;
mod routes;

use oneiros_service::OneirosService;
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;

use error::Error;
use mcp_session::McpSession;
use oneiros_context::{OneirosContext, OneirosContextError};

pub use routes::router;

/// Start the HTTP service, listening on the given TCP address.
///
/// This function blocks until the server is shut down via SIGINT or
/// SIGTERM. After receiving the signal, in-flight connections have
/// `grace_period` to close before the process exits.
pub async fn serve(
    state: OneirosService,
    addr: SocketAddr,
    grace_period: Duration,
) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Service listening on {addr}");

    let app = routes::router(state);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(grace_period))
        .await
}

async fn shutdown_signal(grace_period: Duration) {
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

    // Force exit after the grace period so long-lived streams (SSE, MCP)
    // don't block shutdown indefinitely.
    tokio::spawn(async move {
        tokio::time::sleep(grace_period).await;
        tracing::info!("Grace period elapsed, forcing exit");
        std::process::exit(0);
    });
}
