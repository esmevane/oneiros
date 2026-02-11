use crate::*;

#[derive(Clone)]
pub enum ServiceOutcomes {
    Run(RunServiceOutcomes),
    Status(ServiceStatusOutcomes),
}

impl From<RunServiceOutcomes> for ServiceOutcomes {
    fn from(value: RunServiceOutcomes) -> Self {
        Self::Run(value)
    }
}

impl From<ServiceStatusOutcomes> for ServiceOutcomes {
    fn from(value: ServiceStatusOutcomes) -> Self {
        Self::Status(value)
    }
}
