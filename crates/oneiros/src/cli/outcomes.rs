use crate::*;

pub enum CliOutcomes {
    Doctor(DoctorOutcomes),
    Persona(PersonaOutcomes),
    Project(ProjectOutcomes),
    Service(ServiceOutcomes),
    System(SystemOutcomes),
}

impl From<DoctorOutcomes> for CliOutcomes {
    fn from(outcome: DoctorOutcomes) -> Self {
        CliOutcomes::Doctor(outcome)
    }
}

impl From<PersonaOutcomes> for CliOutcomes {
    fn from(outcome: PersonaOutcomes) -> Self {
        CliOutcomes::Persona(outcome)
    }
}

impl From<ProjectOutcomes> for CliOutcomes {
    fn from(outcome: ProjectOutcomes) -> Self {
        CliOutcomes::Project(outcome)
    }
}

impl From<ServiceOutcomes> for CliOutcomes {
    fn from(outcome: ServiceOutcomes) -> Self {
        CliOutcomes::Service(outcome)
    }
}

impl From<SystemOutcomes> for CliOutcomes {
    fn from(outcome: SystemOutcomes) -> Self {
        CliOutcomes::System(outcome)
    }
}
