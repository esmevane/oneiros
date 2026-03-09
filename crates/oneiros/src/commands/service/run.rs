use std::net::SocketAddr;
use std::sync::Arc;

use clap::Args;
use oneiros_model::*;
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

        let tenant_id: TenantId = database
            .get_tenant_id()?
            .ok_or(ServiceCommandError::MissingId)?
            .parse()?;

        let actor_id: ActorId = database
            .get_actor_id(tenant_id.to_string())?
            .ok_or(ServiceCommandError::MissingId)?
            .parse()?;

        let source = Source {
            actor_id,
            tenant_id,
        };

        outcomes.emit(RunServiceOutcomes::ServiceStarting(addr));

        let state = Arc::new(ServiceState::new(
            database,
            context.data_dir().to_path_buf(),
            source,
        ));

        let grace_period = context.config().service.grace_period();
        oneiros_http::serve(state, addr, grace_period).await?;

        outcomes.emit(RunServiceOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
