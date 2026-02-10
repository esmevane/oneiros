use std::sync::Arc;

use clap::Args;
use oneiros_outcomes::Outcomes;
use oneiros_service::ServiceState;

use super::error::ServiceCommandError;
use super::service_outcomes::RunOutcomes;
use crate::*;

#[derive(Clone, Args)]
pub(crate) struct Run;

impl Run {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<RunOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        if !context.is_initialized() {
            return Err(ServiceCommandError::NotInitialized);
        }

        let database = context.database()?;
        let socket_path = context.socket_path();

        outcomes.emit(RunOutcomes::ServiceStarting(socket_path.clone()));

        let state = Arc::new(ServiceState::new(database, context.data_dir.clone()));

        oneiros_service::serve(state, &socket_path).await?;

        outcomes.emit(RunOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
