//! Seed view — presentation authority for the seed domain.
//!
//! Maps seed responses into formatted confirmation strings.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub(crate) struct SeedView {
    response: SeedResponse,
}

impl SeedView {
    pub(crate) fn new(response: SeedResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<SeedResponse> {
        match self.response {
            SeedResponse::SeedComplete => {
                let prompt = Confirmation::new("Core", "vocabulary", "seeded").to_string();
                Rendered::new(SeedResponse::SeedComplete, prompt, String::new())
            }
            SeedResponse::AgentsSeedComplete => {
                let prompt = Confirmation::new("Canonical", "agents", "seeded").to_string();
                Rendered::new(SeedResponse::AgentsSeedComplete, prompt, String::new())
            }
        }
    }
}
