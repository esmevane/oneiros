use crate::*;

#[derive(Clone)]
pub enum SystemOutcomes {
    InitOutcome(InitSystemOutcomes),
}

impl From<InitSystemOutcomes> for SystemOutcomes {
    fn from(value: InitSystemOutcomes) -> Self {
        Self::InitOutcome(value)
    }
}
