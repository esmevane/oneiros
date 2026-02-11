mod outcomes;

use std::sync::Arc;

use clap::Args;
use oneiros_outcomes::Outcomes;
use oneiros_service::ServiceState;

pub(crate) use outcomes::RunServiceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RunService;

impl RunService {
    pub(crate) async fn run(
        &self,
        context: Context,
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
