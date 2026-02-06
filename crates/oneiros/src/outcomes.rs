use crate::*;

pub(crate) trait Reportable {
    fn report(&self) {}
}

pub(crate) enum Outcome {
    Doctor(Checkups),
    System(SystemOutcome),
}

impl Reportable for Outcome {
    fn report(&self) {
        match self {
            Self::Doctor(checkups) => checkups.report(),
            Self::System(system_outcome) => system_outcome.report(),
        }
    }
}

impl From<Checkups> for Outcome {
    fn from(outcome: Checkups) -> Self {
        Outcome::Doctor(outcome)
    }
}

impl From<SystemOutcome> for Outcome {
    fn from(outcome: SystemOutcome) -> Self {
        Outcome::System(outcome)
    }
}
