use crate::*;

#[allow(dead_code)]
pub(crate) enum CliOutcome {
    Doctor(Checkups),
    System(SystemOutcome),
}

impl From<Checkups> for CliOutcome {
    fn from(outcome: Checkups) -> Self {
        CliOutcome::Doctor(outcome)
    }
}

impl From<SystemOutcome> for CliOutcome {
    fn from(outcome: SystemOutcome) -> Self {
        CliOutcome::System(outcome)
    }
}
