use bon::Builder;

use crate::*;

/// Append a fresh event to the host event log, then notify projections.
#[derive(Builder, Clone)]
pub(crate) struct AppendHostLog {
    pub(crate) scope: Scope<AtHost>,
    #[builder(into)]
    pub(crate) event: Box<NewEvent>,
}

/// Apply a stored event to the host projections.
#[derive(Builder, Clone)]
pub(crate) struct ApplyHostProjection {
    pub(crate) scope: Scope<AtHost>,
    #[builder(into)]
    pub(crate) stored: Box<StoredEvent>,
}

/// Run host-tier projection migrations.
#[derive(Builder, Clone)]
pub(crate) struct MigrateHostProjection {
    pub(crate) scope: Scope<AtHost>,
}

/// Clear and rebuild host-tier projection state.
#[derive(Builder, Clone)]
pub(crate) struct ResetHostProjection {
    pub(crate) scope: Scope<AtHost>,
}

/// All host-tier messages, flat. Each variant addresses a specific
/// actor + action; the router dispatches by variant. Actors handle
/// the variants they own and treat the rest as no-ops.
#[derive(Clone)]
pub(crate) enum HostMessage {
    LogAppend(AppendHostLog),
    ProjectionApply(ApplyHostProjection),
    ProjectionMigrate(MigrateHostProjection),
    ProjectionReset(ResetHostProjection),
}

collects_enum!(
    HostMessage::LogAppend => AppendHostLog,
    HostMessage::ProjectionApply => ApplyHostProjection,
    HostMessage::ProjectionMigrate => MigrateHostProjection,
    HostMessage::ProjectionReset => ResetHostProjection,
);
