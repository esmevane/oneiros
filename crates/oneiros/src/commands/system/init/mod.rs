mod error;
mod outcomes;
#[cfg(test)]
mod tests;

use clap::Args;
use oneiros_outcomes::Outcomes;
use oneiros_protocol::*;

pub(crate) use error::InitSystemError;
pub(crate) use outcomes::InitSystemOutcomes;

use crate::*;

const UNKNOWN_TENANT: &str = "onerios user";

#[derive(Clone, Args)]
pub(crate) struct Init {
    /// Your preferred name for your oneiros host.
    #[arg(long, short)]
    name: Option<TenantName>,

    /// Accept defaults, no prompting.
    #[arg(short, long)]
    yes: bool,
}

impl Init {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<InitSystemOutcomes>, InitSystemError> {
        let mut outcomes = Outcomes::new();

        let file_ops = context.files();

        file_ops.ensure_dir(&context.data_dir)?;
        file_ops.ensure_dir(&context.config_dir)?;

        outcomes.emit(InitSystemOutcomes::EnsuredDirectories);

        let database = context.database()?;

        outcomes.emit(InitSystemOutcomes::DatabaseReady(context.db_path()));

        if database.tenant_exists()? {
            outcomes.emit(InitSystemOutcomes::HostAlreadyInitialized);

            return Ok(outcomes);
        }

        let name = match (self.yes, &self.name) {
            (_, Some(name)) => name.clone(),
            (true, _) => {
                outcomes.emit(InitSystemOutcomes::UnresolvedTenant);
                TenantName::new(UNKNOWN_TENANT)
            }
            _ => match context.terminal().get_name() {
                Some(got_it) => TenantName::new(got_it),
                None => {
                    outcomes.emit(InitSystemOutcomes::UnresolvedTenant);
                    TenantName::new(UNKNOWN_TENANT)
                }
            },
        };

        outcomes.emit(InitSystemOutcomes::ResolvedTenant(name.clone()));

        let tenant_id = TenantId::new();
        let create_tenant = Events::Tenant(TenantEvents::TenantCreated(Identity::new(
            tenant_id,
            Tenant { name: name.clone() },
        )));

        database.log_event(&create_tenant, projections::SYSTEM_PROJECTIONS)?;
        outcomes.emit(InitSystemOutcomes::TenantCreated);

        let actor_id = ActorId::new();

        let create_actor = Events::Actor(ActorEvents::ActorCreated(Identity::new(
            actor_id,
            Actor {
                tenant_id,
                name: ActorName::new(name.as_str()),
            },
        )));

        database.log_event(&create_actor, projections::SYSTEM_PROJECTIONS)?;
        outcomes.emit(InitSystemOutcomes::ActorCreated);

        let config_path = context.config_path();
        if !config_path.exists() {
            file_ops.write(&config_path, "")?;
            outcomes.emit(InitSystemOutcomes::ConfigurationEnsured(config_path));
        }

        outcomes.emit(InitSystemOutcomes::SystemInitialized(name));

        Ok(outcomes)
    }
}
