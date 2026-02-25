use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum InitSystemError {
    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InitSystemOutcomes {
    #[outcome(message("Ensured directories exist."), level = "debug")]
    EnsuredDirectories,
    #[outcome(message("Database ready at {0:?}."), level = "debug")]
    DatabaseReady(std::path::PathBuf),
    #[outcome(message("System already initialized."))]
    HostAlreadyInitialized,
    #[outcome(message("Resolved tenant name: {0}"), level = "debug")]
    ResolvedTenant(TenantName),
    #[outcome(message("Logged tenant_created event."), level = "debug")]
    TenantCreated,
    #[outcome(message("Logged actor_created event."), level = "debug")]
    ActorCreated,
    #[outcome(message("Created config file at {0:?}."), level = "debug")]
    ConfigurationEnsured(std::path::PathBuf),
    #[outcome(message("System initialized for '{0}'."))]
    SystemInitialized(TenantName),
    #[outcome(
        message("Could not resolve tenant name, using default."),
        level = "warn"
    )]
    UnresolvedTenant,
}

const UNKNOWN_TENANT: &str = "onerios user";

#[derive(Clone, Args)]
pub struct Init {
    /// Your preferred name for your oneiros host.
    #[arg(long, short)]
    name: Option<TenantName>,

    /// Accept defaults, no prompting.
    #[arg(short, long)]
    yes: bool,
}

impl Init {
    pub async fn run(
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

        database.log_event(&create_tenant, projections::system::ALL)?;
        outcomes.emit(InitSystemOutcomes::TenantCreated);

        let create_actor = Events::Actor(ActorEvents::ActorCreated(Identity::new(
            ActorId::new(),
            Actor {
                tenant_id,
                name: ActorName::new(name.as_str()),
            },
        )));

        database.log_event(&create_actor, projections::system::ALL)?;
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

#[cfg(test)]
mod tests {
    use oneiros_db::Database;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use super::*;

    // We need to be able to construct a Context with custom paths
    impl Context {
        pub(crate) fn with_paths(data_dir: PathBuf, config_dir: PathBuf) -> Self {
            Self {
                project: None,
                config_dir,
                data_dir,
            }
        }
    }

    #[tokio::test]
    async fn init_creates_tenant_and_actor() {
        let temp = TempDir::new().unwrap();
        let data_dir = temp.path().join("data");
        let config_dir = temp.path().join("config");

        let context = Context::with_paths(data_dir.clone(), config_dir.clone());
        let init = Init {
            name: Some(TenantName::new("Test User")),
            yes: false,
        };

        let outcomes = init.run(&context).await.unwrap();

        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitSystemOutcomes::EnsuredDirectories))
        );
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitSystemOutcomes::TenantCreated))
        );
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitSystemOutcomes::ActorCreated))
        );
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitSystemOutcomes::SystemInitialized(_)))
        );

        // Verify database state
        let db = Database::open(data_dir.join("oneiros.db")).unwrap();
        assert!(db.tenant_exists().unwrap());
        assert_eq!(db.event_count().unwrap(), 2);
    }

    #[tokio::test]
    async fn init_is_idempotent() {
        let temp = TempDir::new().unwrap();
        let data_dir = temp.path().join("data");
        let config_dir = temp.path().join("config");

        let init = Init {
            name: Some(TenantName::new("Test User")),
            yes: false,
        };

        // First run
        let context = Context::with_paths(data_dir.clone(), config_dir.clone());
        let _ = init.run(&context).await.unwrap();

        // Second run
        let context = Context::with_paths(data_dir.clone(), config_dir.clone());
        let outcomes = init.run(&context).await.unwrap();

        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitSystemOutcomes::HostAlreadyInitialized))
        );

        // Still only 2 events
        let db = Database::open(data_dir.join("oneiros.db")).unwrap();
        assert_eq!(db.event_count().unwrap(), 2);
    }
}
