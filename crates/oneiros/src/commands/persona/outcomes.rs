use crate::*;

#[derive(Clone)]
pub enum PersonaOutcomes {
    Set(SetPersonaOutcomes),
    Remove(RemovePersonaOutcomes),
    List(ListPersonasOutcomes),
    Show(ShowPersonaOutcomes),
}

impl From<SetPersonaOutcomes> for PersonaOutcomes {
    fn from(value: SetPersonaOutcomes) -> Self {
        Self::Set(value)
    }
}

impl From<RemovePersonaOutcomes> for PersonaOutcomes {
    fn from(value: RemovePersonaOutcomes) -> Self {
        Self::Remove(value)
    }
}

impl From<ListPersonasOutcomes> for PersonaOutcomes {
    fn from(value: ListPersonasOutcomes) -> Self {
        Self::List(value)
    }
}

impl From<ShowPersonaOutcomes> for PersonaOutcomes {
    fn from(value: ShowPersonaOutcomes) -> Self {
        Self::Show(value)
    }
}
