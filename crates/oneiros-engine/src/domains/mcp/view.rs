//! MCP view — presentation authority for the MCP config domain.
//!
//! Maps MCP config responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use std::path::Path;

use crate::*;

pub(crate) struct McpView;

impl McpView {
    /// Confirmation that the MCP config was written.
    pub(crate) fn written(path: &Path) -> String {
        format!(
            "{} MCP config written to {}.",
            "✓".success(),
            path.display()
        )
    }

    /// Message when the MCP config already exists and was skipped.
    pub(crate) fn exists(path: &Path) -> String {
        format!(
            "{}",
            format!("MCP config already exists at {}. Skipped.", path.display()).muted()
        )
    }
}
