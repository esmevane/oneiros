//! System view — presentation authority for the system domain.
//!
//! Maps system responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub(crate) struct SystemView;

impl SystemView {
    /// Confirmation that the system has been initialized.
    pub(crate) fn initialized(name: &TenantName) -> String {
        Confirmation::new("System", name.to_string(), "initialized").to_string()
    }

    /// Message when the system host is already initialized.
    pub(crate) fn already_initialized() -> String {
        format!("{}", "System already initialized.".muted())
    }
}
