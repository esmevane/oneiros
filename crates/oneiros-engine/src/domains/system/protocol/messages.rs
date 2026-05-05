use bon::Builder;

use crate::*;

/// Append a fresh event to the system event log, then notify projections.
#[derive(Builder, Clone)]
pub struct AppendSystemLog {
    pub scope: Scope<AtHost>,
    #[builder(into)]
    pub event: Box<NewEvent>,
}

/// Wipe the system event log. (No-op stub for now — durable record.)
#[derive(Builder, Clone)]
pub struct ResetSystemLog {
    pub scope: Scope<AtHost>,
}

/// Apply a stored event to the system projections.
#[derive(Builder, Clone)]
pub struct ApplySystemProjection {
    pub scope: Scope<AtHost>,
    #[builder(into)]
    pub stored: Box<StoredEvent>,
}

/// Run system-tier projection migrations.
#[derive(Builder, Clone)]
pub struct MigrateSystemProjection {
    pub scope: Scope<AtHost>,
}

/// Clear and rebuild system-tier projection state.
#[derive(Builder, Clone)]
pub struct ResetSystemProjection {
    pub scope: Scope<AtHost>,
}

/// All system-tier messages, flat. Each variant addresses a specific
/// actor + action; the router dispatches by variant. Actors handle
/// the variants they own and treat the rest as no-ops.
#[derive(Clone)]
pub enum SystemMessage {
    LogAppend(AppendSystemLog),
    LogReset(ResetSystemLog),
    ProjectionApply(ApplySystemProjection),
    ProjectionMigrate(MigrateSystemProjection),
    ProjectionReset(ResetSystemProjection),
}

impl SystemMessage {
    pub fn scope(&self) -> &Scope<AtHost> {
        match self {
            Self::LogAppend(message) => &message.scope,
            Self::LogReset(message) => &message.scope,
            Self::ProjectionApply(message) => &message.scope,
            Self::ProjectionMigrate(message) => &message.scope,
            Self::ProjectionReset(message) => &message.scope,
        }
    }
}

collects_enum!(
    SystemMessage::LogAppend => AppendSystemLog,
    SystemMessage::LogReset => ResetSystemLog,
    SystemMessage::ProjectionApply => ApplySystemProjection,
    SystemMessage::ProjectionMigrate => MigrateSystemProjection,
    SystemMessage::ProjectionReset => ResetSystemProjection,
);
