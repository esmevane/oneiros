use std::net::SocketAddr;
use std::sync::Arc;

use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_service::ServiceState;

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

        let database = context.database()?;
        let addr = context.config().service_addr();

        outcomes.emit(RunServiceOutcomes::ServiceStarting(addr));

        let state = Arc::new(ServiceState::new(
            database,
            context.data_dir().to_path_buf(),
        ));

        oneiros_service::serve(state, addr).await?;

        outcomes.emit(RunServiceOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
