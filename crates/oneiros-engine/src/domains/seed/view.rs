//! Seed view — presentation authority for the seed domain.
//!
//! Maps seed responses into formatted confirmation strings.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub(crate) struct SeedView;

impl SeedView {
    /// Confirmation that core vocabulary has been seeded.
    pub(crate) fn core_complete() -> String {
        Confirmation::new("Core", "vocabulary", "seeded").to_string()
    }

    /// Confirmation that canonical agents have been seeded.
    pub(crate) fn agents_complete() -> String {
        Confirmation::new("Canonical", "agents", "seeded").to_string()
    }
}
