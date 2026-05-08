use bon::Builder;

use crate::*;

/// Append a fresh event to the system event log, then notify projections.
#[derive(Builder, Clone)]
pub(crate) struct AppendSystemLog {
    pub(crate) scope: Scope<AtHost>,
    #[builder(into)]
    pub(crate) event: Box<NewEvent>,
}

/// Apply a stored event to the system projections.
#[derive(Builder, Clone)]
pub(crate) struct ApplySystemProjection {
    pub(crate) scope: Scope<AtHost>,
    #[builder(into)]
    pub(crate) stored: Box<StoredEvent>,
}

/// Run system-tier projection migrations.
#[derive(Builder, Clone)]
pub(crate) struct MigrateSystemProjection {
    pub(crate) scope: Scope<AtHost>,
}

/// Clear and rebuild system-tier projection state.
#[derive(Builder, Clone)]
pub(crate) struct ResetSystemProjection {
    pub(crate) scope: Scope<AtHost>,
}

/// All system-tier messages, flat. Each variant addresses a specific
/// actor + action; the router dispatches by variant. Actors handle
/// the variants they own and treat the rest as no-ops.
#[derive(Clone)]
pub(crate) enum SystemMessage {
    LogAppend(AppendSystemLog),
    ProjectionApply(ApplySystemProjection),
    ProjectionMigrate(MigrateSystemProjection),
    ProjectionReset(ResetSystemProjection),
}

collects_enum!(
    SystemMessage::LogAppend => AppendSystemLog,
    SystemMessage::ProjectionApply => ApplySystemProjection,
    SystemMessage::ProjectionMigrate => MigrateSystemProjection,
    SystemMessage::ProjectionReset => ResetSystemProjection,
);
