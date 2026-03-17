//! AppBuilder — the registry that composes resources into transport surfaces.
//!
//! Resources mount themselves into the builder via `Mountable::mount`.
//! The builder collects features and produces ready-to-use transport surfaces.

use axum::Router;
use oneiros_db::Projection;
use oneiros_resource::Mountable;

use crate::mcp::{ToolError, ToolResult, ToolSurface};
use crate::ServiceState;

/// The application builder. Resources mount into this, and it produces
/// the composed transport surfaces.
pub struct AppBuilder {
    state: ServiceState,
    router: Router<ServiceState>,
    tools: Vec<ToolSurface>,
    projections: Vec<&'static [Projection]>,
}

impl AppBuilder {
    pub fn new(state: ServiceState) -> Self {
        Self {
            state,
            router: Router::new(),
            tools: Vec::new(),
            projections: Vec::new(),
        }
    }

    /// Mount a resource (or middleware) into the application.
    pub fn mount<M: Mountable<Self>>(mut self, mountable: M) -> Self {
        mountable.mount(&mut self);
        self
    }

    // ── Typed collection methods called by Mountable::mount impls ──

    /// Add an HTTP sub-router at the given path.
    pub fn nest(&mut self, path: &str, router: Router<ServiceState>) {
        // Take the current router, nest the new one, put it back
        let current = std::mem::replace(&mut self.router, Router::new());
        self.router = current.nest(path, router);
    }

    /// Register MCP tools from a resource.
    pub fn tools(&mut self, surface: ToolSurface) {
        self.tools.push(surface);
    }

    /// Register projections from a resource.
    pub fn projections(&mut self, projections: &'static [Projection]) {
        self.projections.push(projections);
    }

    // ── Build: produce the composed transport surfaces ──────────────

    /// Build the composed HTTP router with state applied.
    pub fn into_router(self) -> Router {
        self.router.with_state(self.state)
    }

    /// Get the collected projection slices.
    pub fn projection_slices(&self) -> Vec<&'static [Projection]> {
        self.projections.clone()
    }

    /// Dispatch an MCP tool call through the collected tool surfaces.
    pub fn dispatch_tool(
        &self,
        tool_name: &str,
        params: &str,
    ) -> Result<ToolResult, ToolError> {
        for surface in &self.tools {
            if surface.names.contains(&tool_name) {
                return (surface.handler)(&self.state, tool_name, params);
            }
        }
        Err(ToolError::UnknownTool(tool_name.to_string()))
    }
}
