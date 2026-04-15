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
            ServiceResponse::ServiceInstalled(name) => {
                let prompt = format!(
                    "{} {}",
                    "✓".success(),
                    ServiceResponse::ServiceInstalled(name.clone())
                );
                Rendered::new(
                    ServiceResponse::ServiceInstalled(name),
                    prompt,
                    String::new(),
                )
            }
            ServiceResponse::ServiceUninstalled => {
                let prompt = format!("{} {}", "✓".success(), ServiceResponse::ServiceUninstalled);
                Rendered::new(ServiceResponse::ServiceUninstalled, prompt, String::new())
            }
            ServiceResponse::ServiceStarted => {
                let prompt = format!("{} {}", "✓".success(), ServiceResponse::ServiceStarted);
                Rendered::new(ServiceResponse::ServiceStarted, prompt, String::new())
            }
            ServiceResponse::ServiceHealthy(addr) => {
                let prompt = format!(
                    "{} {}",
                    "✓".success(),
                    ServiceResponse::ServiceHealthy(addr.clone())
                );
                Rendered::new(ServiceResponse::ServiceHealthy(addr), prompt, String::new())
            }
            ServiceResponse::ServiceStopped => {
                let prompt = format!("{} {}", "✓".success(), ServiceResponse::ServiceStopped);
                Rendered::new(ServiceResponse::ServiceStopped, prompt, String::new())
            }
            ServiceResponse::ServiceRunning(addr) => {
                let prompt = format!(
                    "{} {}",
                    "✓".success(),
                    ServiceResponse::ServiceRunning(addr.clone())
                );
                Rendered::new(ServiceResponse::ServiceRunning(addr), prompt, String::new())
            }
            ServiceResponse::ServiceNotRunning(reason) => {
                let prompt = format!(
                    "{} {}",
                    "!".warning(),
                    ServiceResponse::ServiceNotRunning(reason.clone())
                );
                Rendered::new(
                    ServiceResponse::ServiceNotRunning(reason),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
