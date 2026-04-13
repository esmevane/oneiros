//! Service view — presentation authority for the service domain.
//!
//! Maps service responses into styled strings. Success and neutral outcomes
//! are prefixed with a success checkmark; warning conditions use a warning
//! prefix. The Display text on ServiceResponse provides the message body.

use crate::*;

pub(crate) struct ServiceView;

impl ServiceView {
    /// Render a service response with appropriate styling.
    pub(crate) fn render(response: &ServiceResponse) -> String {
        match response {
            ServiceResponse::ServiceInstalled(_) => {
                format!("{} {response}", "✓".success())
            }
            ServiceResponse::ServiceUninstalled => {
                format!("{} {response}", "✓".success())
            }
            ServiceResponse::ServiceStarted => {
                format!("{} {response}", "✓".success())
            }
            ServiceResponse::ServiceHealthy(_) => {
                format!("{} {response}", "✓".success())
            }
            ServiceResponse::ServiceStopped => {
                format!("{} {response}", "✓".success())
            }
            ServiceResponse::ServiceRunning(_) => {
                format!("{} {response}", "✓".success())
            }
            ServiceResponse::ServiceNotRunning(_) => {
                format!("{} {response}", "!".warning())
            }
        }
    }
}
