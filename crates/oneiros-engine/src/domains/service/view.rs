//! Service view — presentation authority for the service domain.
//!
//! Maps service responses into styled strings. Success and neutral outcomes
//! are prefixed with a success checkmark; warning conditions use a warning
//! prefix. The Display text on ServiceResponse provides the message body.

use crate::*;

pub struct ServiceView {
    response: ServiceResponse,
}

impl ServiceView {
    pub fn new(response: ServiceResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<ServiceResponse> {
        match self.response {
            ServiceResponse::ServiceInstalled(ServiceInstalledResponse::V1(details)) => {
                let response = ServiceResponse::ServiceInstalled(
                    ServiceInstalledResponse::builder_v1()
                        .name(details.name)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "✓".success(), response);
                Rendered::new(response, prompt, String::new())
            }
            ServiceResponse::ServiceUninstalled => {
                let prompt = format!("{} {}", "✓".success(), ServiceResponse::ServiceUninstalled);
                Rendered::new(ServiceResponse::ServiceUninstalled, prompt, String::new())
            }
            ServiceResponse::ServiceStarted => {
                let prompt = format!("{} {}", "✓".success(), ServiceResponse::ServiceStarted);
                Rendered::new(ServiceResponse::ServiceStarted, prompt, String::new())
            }
            ServiceResponse::ServiceHealthy(ServiceHealthyResponse::V1(details)) => {
                let response = ServiceResponse::ServiceHealthy(
                    ServiceHealthyResponse::builder_v1()
                        .address(details.address)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "✓".success(), response);
                Rendered::new(response, prompt, String::new())
            }
            ServiceResponse::ServiceStopped => {
                let prompt = format!("{} {}", "✓".success(), ServiceResponse::ServiceStopped);
                Rendered::new(ServiceResponse::ServiceStopped, prompt, String::new())
            }
            ServiceResponse::ServiceRunning(ServiceRunningResponse::V1(details)) => {
                let response = ServiceResponse::ServiceRunning(
                    ServiceRunningResponse::builder_v1()
                        .address(details.address)
                        .build()
                        .into(),
                );
                let prompt = format!("{} {}", "✓".success(), response);
                Rendered::new(response, prompt, String::new())
            }
            ServiceResponse::ServiceNotRunning(ServiceNotRunningResponse::V1(details)) => {
                let response = ServiceResponse::ServiceNotRunning(
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
