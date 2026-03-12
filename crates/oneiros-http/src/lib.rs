mod error;
mod handlers;
mod mcp_session;
mod oneiros_context;
mod routes;

use oneiros_service::OneirosService;

use error::Error;
use mcp_session::McpSession;
use oneiros_context::{OneirosContext, OneirosContextError};

pub use http_service::*;
pub use routes::router;

mod http_service {
    use oneiros_context::*;
    use oneiros_model::*;
    use oneiros_service::OneirosService;
    use oneiros_service::*;
    use std::{net::SocketAddr, sync::Arc, time::Duration};
    use tokio::net::TcpListener;

    #[derive(Debug, thiserror::Error)]
    pub enum HttpServiceError {
        #[error("Database error: {0}")]
        Database(#[from] oneiros_db::DatabaseError),

        #[error(transparent)]
        Io(#[from] std::io::Error),

        #[error(transparent)]
        MalformedId(#[from] oneiros_model::IdParseError),

        #[error("Missing tenant or actor ID in database.")]
        MissingId,
    }

    pub struct HttpService {
        pub address: SocketAddr,
        pub source: Source,
        pub context: Context,
    }

    impl HttpService {
        pub fn init(context: Context) -> Result<Self, HttpServiceError> {
            let database = context.database()?;

            let tenant_id: TenantId = database
                .get_tenant_id()?
                .ok_or(HttpServiceError::MissingId)?
                .parse()?;

            let actor_id: ActorId = database
                .get_actor_id(tenant_id.to_string())?
                .ok_or(HttpServiceError::MissingId)?
                .parse()?;

            let source = Source {
                actor_id,
                tenant_id,
            };

            Ok(Self {
                address: context.config().service_addr(),
                context,
                source,
            })
        }

        /// Start the HTTP service, binding and listening on the configured
        /// TCP address.
        ///
        /// This function blocks until the server is shut down via SIGINT or
        /// SIGTERM. After receiving the signal, in-flight connections have
        /// `grace_period` to close before the process exits.
        pub async fn run(self) -> Result<(), HttpServiceError> {
            let listener = TcpListener::bind(self.address).await?;

            tracing::info!("Service listening on {}", self.address);

            self.serve(listener).await
        }

        /// Serve on a pre-bound listener.
        pub async fn serve(self, listener: TcpListener) -> Result<(), HttpServiceError> {
            let service = OneirosService::system(Arc::new(ServiceState::new(
                self.context.database()?,
                self.context.data_dir().to_path_buf(),
                self.source,
            )));

            let app = crate::routes::router(service);

            axum::serve(listener, app.into_make_service())
                .with_graceful_shutdown(shutdown_signal(
                    self.context.config().service.grace_period(),
                ))
                .await?;

            Ok(())
        }
    }

    async fn shutdown_signal(grace_period: Duration) {
        let ctrl_c = tokio::signal::ctrl_c();

        #[cfg(unix)]
        {
            let mut sigterm =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
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
}
