mod error;
mod handlers;
mod routes;
mod state;

pub use error::Error;
pub use routes::router;
pub use state::ServiceState;

use std::path::Path;
use std::sync::Arc;

use tokio::net::UnixListener;

/// Start the service, listening on the given Unix socket path.
///
/// This function blocks until the server is shut down. The caller is
/// responsible for ensuring the socket path's parent directory exists
/// and for cleaning up stale socket files.
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
    axum::serve(listener, app.into_make_service()).await
}
