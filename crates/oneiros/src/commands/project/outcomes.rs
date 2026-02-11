use crate::*;

#[derive(Clone)]
pub enum ProjectOutcomes {
    Init(InitProjectOutcomes),
}

impl From<InitProjectOutcomes> for ProjectOutcomes {
    fn from(value: InitProjectOutcomes) -> Self {
        Self::Init(value)
    }
}
