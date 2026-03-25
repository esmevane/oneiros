//! Server — wraps an `Engine` and serves it over HTTP.
//!
//! `Server` is the network-facing entry point. It takes a bootstrapped
//! `Engine`, builds the combined router, and serves on a TCP listener.

use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use crate::*;

/// An HTTP server backed by an `Engine`.
///
/// Create with `Server::new(engine)`, then call `serve(listener)` to
/// run on a pre-bound listener, or `run(addr)` to bind and serve.
pub struct Server {
    engine: Engine,
}

impl Server {
    /// Wrap a bootstrapped engine in a server.
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    /// Build the combined router (system + project routes).
    pub fn router(&self) -> Result<Router, Error> {
        let mut app = Router::new();

        // System routes are always available
        app = app.merge(self.engine.system_router());

        // Project routes are available if a project has been initialized
        if let Ok(router) = self.engine.project_router() {
            app = app.merge(router);
        }

        Ok(app)
    }

    /// Serve on a pre-bound TCP listener.
    ///
    /// This is the primary entry point for tests — bind to port 0,
    /// discover the address, then pass the listener here.
    pub async fn serve(self, listener: TcpListener) -> Result<(), Error> {
        let app = self.router()?;

        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| Error::Context(format!("serve: {e}")))?;

        Ok(())
    }

    /// Bind to an address and serve.
    pub async fn run(self, addr: SocketAddr) -> Result<(), Error> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| Error::Context(format!("bind: {e}")))?;

        self.serve(listener).await
    }

    /// Access the underlying engine.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}
