use crate::*;

#[allow(dead_code)]
pub(crate) enum CliOutcome {
    Doctor(Checkups),
    Project(ProjectOutcome),
    Service(ServiceOutcome),
    System(SystemOutcome),
}

impl From<Checkups> for CliOutcome {
    fn from(outcome: Checkups) -> Self {
        CliOutcome::Doctor(outcome)
    }
}

impl From<ProjectOutcome> for CliOutcome {
    fn from(outcome: ProjectOutcome) -> Self {
        CliOutcome::Project(outcome)
    }
}

impl From<ServiceOutcome> for CliOutcome {
    fn from(outcome: ServiceOutcome) -> Self {
        CliOutcome::Service(outcome)
    }
}

impl From<SystemOutcome> for CliOutcome {
    fn from(outcome: SystemOutcome) -> Self {
        CliOutcome::System(outcome)
    }
}
