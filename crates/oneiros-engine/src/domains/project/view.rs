//! Project view — presentation authority for the project domain.
//!
//! Maps project responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub struct ProjectView;

impl ProjectView {
    /// Confirmation that a brain has been created.
    pub fn initialized(result: &InitResult) -> String {
        Confirmation::new("Brain", result.brain_name.to_string(), "created").to_string()
    }

    /// Message when a brain with that name already exists.
    pub fn already_exists(name: &BrainName) -> String {
        format!("{}", format!("Brain '{name}' already exists.").muted())
    }

    /// Confirmation that an export was written.
    pub fn exported(path: &ExportPath) -> String {
        format!("{} Export written to '{path}'.", "✓".success())
    }

    /// Confirmation that events were imported and replayed.
    pub fn imported(result: &ImportResult) -> String {
        format!(
            "{} Imported {} events, replayed {}.",
            "✓".success(),
            result.imported,
            result.replayed,
        )
    }

    /// Confirmation that events were replayed.
    pub fn replayed(result: &ReplayResult) -> String {
        format!("{} Replayed {} events.", "✓".success(), result.replayed)
    }
}
