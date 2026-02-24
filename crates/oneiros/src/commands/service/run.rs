use std::sync::Arc;

use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_service::ServiceState;
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RunServiceOutcomes {
    #[outcome(message("Service starting on {}.", .0.display()))]
    ServiceStarting(PathBuf),
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
        let socket_path = context.socket_path();

        outcomes.emit(RunServiceOutcomes::ServiceStarting(socket_path.clone()));

        let state = Arc::new(ServiceState::new(database, context.data_dir.clone()));

        oneiros_service::serve(state, &socket_path).await?;

        outcomes.emit(RunServiceOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
