mod error;
mod extractors;
mod handlers;
mod routes;
mod state;

use std::path::Path;
use std::sync::Arc;
use tokio::net::UnixListener;

pub use error::*;
pub use extractors::*;
pub use routes::router;
pub use state::*;

/// Start the service, listening on the given Unix socket path.
///
/// This function blocks until the server is shut down via SIGINT or
/// SIGTERM. The caller is responsible for ensuring the socket path's
/// parent directory exists and for cleaning up stale socket files.
pub async fn serve(state: Arc<ServiceState>, socket_path: &Path) -> Result<(), std::io::Error> {
    if socket_path.exists() {
        tokio::fs::remove_file(socket_path).await?;
    }

    if let Some(parent) = socket_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let listener = UnixListener::bind(socket_path)?;

    tracing::info!("Service listening on {}", socket_path.display());

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
