mod error;
mod extractors;
mod handlers;
mod routes;
mod state;

pub mod projections;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub use error::*;
pub use extractors::*;
pub use projections::{brain, system};
pub use routes::router;
pub use state::*;

/// Start the service, listening on the given TCP address.
///
/// This function blocks until the server is shut down via SIGINT or
/// SIGTERM.
pub async fn serve(state: Arc<ServiceState>, addr: SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Service listening on {addr}");

    let app = routes::router(state);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
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
