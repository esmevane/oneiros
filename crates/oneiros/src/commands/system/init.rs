use clap::Args;

use crate::*;

const UNKNOWN_TENANT: &str = "onerios user";

#[derive(Clone, Args)]
pub(crate) struct Init {
    /// Your preferred tenant name.
    #[arg(long, short)]
    name: Option<String>,

    /// Accept defaults, no prompting.
    #[arg(short, long)]
    yes: bool,
}

impl Init {
    pub(crate) async fn run(
        &self,
        context: Option<Context>,
    ) -> Result<Vec<InitOutcomes>, InitError> {
        let Some(context) = context else {
            return Ok(vec![InitOutcomes::NoSystemContext]);
        };

        let mut outcomes = Vec::new();
        let file_ops = context.files();

        file_ops.ensure_dir(&context.data_dir)?;
        file_ops.ensure_dir(&context.config_dir)?;

        outcomes.push(InitOutcomes::EnsuredDirectories);

        let database = context.database()?;

        outcomes.push(InitOutcomes::DatabaseReady(context.db_path()));

        if database.tenant_exists()? {
            outcomes.push(InitOutcomes::HostAlreadyInitialized);

            return Ok(outcomes);
        }

        let name = match (self.yes, &self.name) {
            (_, Some(name)) => Label::new(name),
            (true, _) => {
                outcomes.push(InitOutcomes::UnresolvedTenant);
                Label::new(UNKNOWN_TENANT)
            }
            _ => match context.terminal().get_name() {
                Some(got_it) => Label::new(got_it),
                None => {
                    outcomes.push(InitOutcomes::UnresolvedTenant);
                    Label::new(UNKNOWN_TENANT)
                }
            },
        };

        outcomes.push(InitOutcomes::ResolvedTenant(name.clone()));

        let tenant_id = Id::new();
        let create_tenant = Events::Tenant(TenantEvents::TenantCreated(Tenant {
            tenant_id,
            name: name.clone(),
        }));

        database.log_event(&create_tenant, projections::SYSTEM_PROJECTIONS)?;
        outcomes.push(InitOutcomes::TenantCreated);

        let actor_id = Id::new();

        let create_actor = Events::Actor(ActorEvents::ActorCreated(Actor {
            tenant_id,
            actor_id,
            name: name.clone(),
        }));

        database.log_event(&create_actor, projections::SYSTEM_PROJECTIONS)?;
        outcomes.push(InitOutcomes::ActorCreated);

        let config_path = context.config_path();
        if !config_path.exists() {
            file_ops.write(&config_path, "")?;
            outcomes.push(InitOutcomes::ConfigurationEnsured(config_path));
        }

        outcomes.push(InitOutcomes::SystemInitialized(name));

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
            name: Some("Test User".into()),
            yes: false,
        };

        let outcomes = init.run(Some(context)).await.unwrap();

        // Check outcomes
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitOutcomes::EnsuredDirectories))
        );
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitOutcomes::TenantCreated))
        );
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitOutcomes::ActorCreated))
        );
        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitOutcomes::SystemInitialized(_)))
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
            name: Some("Test User".into()),
            yes: false,
        };

        // First run
        let context = Context::with_paths(data_dir.clone(), config_dir.clone());
        let _ = init.run(Some(context)).await.unwrap();

        // Second run
        let context = Context::with_paths(data_dir.clone(), config_dir.clone());
        let outcomes = init.run(Some(context)).await.unwrap();

        assert!(
            outcomes
                .iter()
                .any(|o| matches!(o, InitOutcomes::HostAlreadyInitialized))
        );

        // Still only 2 events
        let db = Database::open(data_dir.join("oneiros.db")).unwrap();
        assert_eq!(db.event_count().unwrap(), 2);
    }
}
