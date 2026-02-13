use crate::*;
use oneiros_outcomes::Outcome;

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
