use crate::*;

#[derive(Clone)]
pub(crate) enum InitOutcomes {
    NoSystemContext,
    EnsuredDirectories,
    DatabaseReady(std::path::PathBuf),
    HostAlreadyInitialized,
    ResolvedTenant(Label),
    TenantCreated,
    ActorCreated,
    ConfigurationEnsured(std::path::PathBuf),
    SystemInitialized(Label),
    UnresolvedTenant,
}

impl Reportable for InitOutcomes {
    fn report(&self) {
        match self {
            Self::UnresolvedTenant => {
                tracing::warn!("Could not resolve tenant name, using default.")
            }
            Self::NoSystemContext => tracing::error!("Failed to discover system context."),
            Self::EnsuredDirectories => tracing::debug!("Ensured directories exist."),
            Self::DatabaseReady(db_path) => tracing::debug!("Database ready at {:?}.", db_path),
            Self::HostAlreadyInitialized => tracing::info!("System already initialized."),
            Self::ResolvedTenant(name) => tracing::debug!("Resolved tenant name: {}", name),
            Self::TenantCreated => tracing::debug!("Logged tenant_created event."),
            Self::ActorCreated => tracing::debug!("Logged actor_created event."),
            Self::ConfigurationEnsured(config_path) => {
                tracing::debug!("Created config file at {:?}.", config_path)
            }
            Self::SystemInitialized(name) => {
                tracing::info!("System initialized for '{}'.", name)
            }
        }
    }
}
