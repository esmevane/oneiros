use clap::Args;
use oneiros_http::*;
use oneiros_outcomes::{Outcome, Outcomes};
use std::net::SocketAddr;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RunServiceOutcomes {
    #[outcome(message("Service starting on {0}."))]
    ServiceStarting(SocketAddr),
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}

#[derive(Clone, Args)]
pub struct RunService;

impl RunService {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RunServiceOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        if !context.is_initialized() {
            return Err(ServiceCommandError::NotInitialized);
        }

        let service = HttpService::init(context)?;
        let address = service.address;

        outcomes.emit(RunServiceOutcomes::ServiceStarting(address));

        service.run().await?;

        outcomes.emit(RunServiceOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
