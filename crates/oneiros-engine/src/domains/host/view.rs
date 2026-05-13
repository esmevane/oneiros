//! Host view — presentation authority for the host domain.
//!
//! Maps host responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub(crate) struct HostView {
    response: HostResponse,
}

impl HostView {
    pub(crate) fn new(response: HostResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<HostResponse> {
        match self.response {
            HostResponse::HostInitialized(HostInitializedResponse::V1(details)) => {
                let prompt = Confirmation::new("Host", details.tenant.to_string(), "initialized")
                    .to_string();
                Rendered::new(
                    HostResponse::HostInitialized(
                        HostInitializedResponse::builder_v1()
                            .tenant(details.tenant)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            HostResponse::HostAlreadyInitialized => {
                let prompt = format!("{}", "Host already initialized.".muted());
                Rendered::new(HostResponse::HostAlreadyInitialized, prompt, String::new())
            }
            HostResponse::ServiceInstalled(ServiceInstalledResponse::V1(details)) => {
                let response = HostResponse::ServiceInstalled(
                    ServiceInstalledResponse::builder_v1()
                        .name(details.name)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "✓".success(), response);
                Rendered::new(response, prompt, String::new())
            }
            HostResponse::ServiceUninstalled => {
                let prompt = format!("{} {}", "✓".success(), HostResponse::ServiceUninstalled);
                Rendered::new(HostResponse::ServiceUninstalled, prompt, String::new())
            }
            HostResponse::ServiceStarted => {
                let prompt = format!("{} {}", "✓".success(), HostResponse::ServiceStarted);
                Rendered::new(HostResponse::ServiceStarted, prompt, String::new())
            }
            HostResponse::ServiceHealthy(ServiceHealthyResponse::V1(details)) => {
                let response = HostResponse::ServiceHealthy(
                    ServiceHealthyResponse::builder_v1()
                        .address(details.address)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "✓".success(), response);
                Rendered::new(response, prompt, String::new())
            }
            HostResponse::ServiceStopped => {
                let prompt = format!("{} {}", "✓".success(), HostResponse::ServiceStopped);
                Rendered::new(HostResponse::ServiceStopped, prompt, String::new())
            }
            HostResponse::ServiceRunning(ServiceRunningResponse::V1(details)) => {
                let response = HostResponse::ServiceRunning(
                    ServiceRunningResponse::builder_v1()
                        .address(details.address)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "✓".success(), response);
                Rendered::new(response, prompt, String::new())
            }
            HostResponse::ServiceNotRunning(ServiceNotRunningResponse::V1(details)) => {
                let response = HostResponse::ServiceNotRunning(
                    ServiceNotRunningResponse::builder_v1()
                        .reason(details.reason)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "!".warning(), response);
                Rendered::new(response, prompt, String::new())
            }
        }
    }
}
