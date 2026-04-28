//! System view — presentation authority for the system domain.
//!
//! Maps system responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub struct SystemView {
    response: SystemResponse,
}

impl SystemView {
    pub fn new(response: SystemResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<SystemResponse> {
        match self.response {
            SystemResponse::SystemInitialized(SystemInitializedResponse::V1(details)) => {
                let prompt = Confirmation::new("System", details.tenant.to_string(), "initialized")
                    .to_string();
                Rendered::new(
                    SystemResponse::SystemInitialized(
                        SystemInitializedResponse::builder_v1()
                            .tenant(details.tenant)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            SystemResponse::HostAlreadyInitialized => {
                let prompt = format!("{}", "System already initialized.".muted());
                Rendered::new(
                    SystemResponse::HostAlreadyInitialized,
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
