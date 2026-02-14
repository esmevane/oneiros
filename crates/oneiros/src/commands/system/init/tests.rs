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
