mod error;
mod outcomes;
#[cfg(test)]
mod tests;

use clap::Args;
use oneiros_outcomes::Outcomes;

pub(crate) use error::InitSystemError;
pub(crate) use outcomes::InitSystemOutcomes;

use crate::*;

const UNKNOWN_TENANT: &str = "onerios user";

/// The identity-defining fields for a tenant. Serialized with postcard
/// and hashed with SHA-256 to produce a deterministic content-addressed ID.
#[derive(serde::Serialize)]
struct TenantContent<'a> {
    name: &'a TenantName,
}

/// The identity-defining fields for an actor. Serialized with postcard
/// and hashed with SHA-256 to produce a deterministic content-addressed ID.
#[derive(serde::Serialize)]
struct ActorContent<'a> {
    tenant_id: &'a TenantId,
    name: &'a ActorName,
}

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

        let tenant_bytes = postcard::to_allocvec(&TenantContent { name: &name })
            .expect("postcard serialization of tenant content");
        let tenant_id = TenantId(Id::from_content(&tenant_bytes));

        let create_tenant = Events::Tenant(TenantEvents::TenantCreated(Tenant {
            tenant_id,
            name: name.clone(),
        }));

        database.log_event(&create_tenant, projections::SYSTEM_PROJECTIONS)?;
        outcomes.emit(InitSystemOutcomes::TenantCreated);

        let actor_name = ActorName::new(name.as_str());
        let actor_bytes = postcard::to_allocvec(&ActorContent {
            tenant_id: &tenant_id,
            name: &actor_name,
        })
        .expect("postcard serialization of actor content");
        let actor_id = ActorId(Id::from_content(&actor_bytes));

        let create_actor = Events::Actor(ActorEvents::ActorCreated(Actor {
            tenant_id,
            actor_id,
            name: actor_name,
        }));

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
