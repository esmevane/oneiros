//! Dashboard assets — the compiled Astro SPA, embedded at compile time.
//!
//! The xtask `dashboard-build` copies `apps/dashboard/dist/` into
//! `templates/dashboard/` before the engine binary is built. This module
//! embeds that directory so the server can serve the SPA without
//! external file dependencies.

use axum::{
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/templates/dashboard/"]
pub(crate) struct DashboardAssets;

impl DashboardAssets {
    /// Serve `index.html` for the root route `/`.
    pub(crate) fn index_html() -> Response {
        Self::serve("index.html")
    }

    /// Look up an embedded asset by path (relative to the dashboard root).
    /// Returns a 404 if the asset isn't found.
    pub(crate) fn serve(path: &str) -> Response {
        match Self::get(path) {
            Some(file) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                let mut response = Response::new(Body::from(file.data));
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_str(mime.as_ref()).unwrap(),
                );
                response
            }
            None => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
