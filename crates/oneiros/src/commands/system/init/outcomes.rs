use crate::*;

#[derive(Clone)]
pub enum InitSystemOutcomes {
    EnsuredDirectories,
    DatabaseReady(std::path::PathBuf),
    HostAlreadyInitialized,
    ResolvedTenant(TenantName),
    TenantCreated,
    ActorCreated,
    ConfigurationEnsured(std::path::PathBuf),
    SystemInitialized(TenantName),
    UnresolvedTenant,
}

impl oneiros_outcomes::Reportable for InitSystemOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::UnresolvedTenant => tracing::Level::WARN,
            Self::HostAlreadyInitialized | Self::SystemInitialized(_) => tracing::Level::INFO,
            Self::EnsuredDirectories
            | Self::DatabaseReady(_)
            | Self::ResolvedTenant(_)
            | Self::TenantCreated
            | Self::ActorCreated
            | Self::ConfigurationEnsured(_) => tracing::Level::DEBUG,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::UnresolvedTenant => "Could not resolve tenant name, using default.".into(),
            Self::EnsuredDirectories => "Ensured directories exist.".into(),
            Self::DatabaseReady(db_path) => format!("Database ready at {db_path:?}."),
            Self::HostAlreadyInitialized => "System already initialized.".into(),
            Self::ResolvedTenant(name) => format!("Resolved tenant name: {name}"),
            Self::TenantCreated => "Logged tenant_created event.".into(),
            Self::ActorCreated => "Logged actor_created event.".into(),
            Self::ConfigurationEnsured(config_path) => {
                format!("Created config file at {config_path:?}.")
            }
            Self::SystemInitialized(name) => format!("System initialized for '{name}'."),
        }
    }
}
