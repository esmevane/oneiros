use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ExperienceEventsType, display = "kebab-case")]
pub(crate) enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceDescriptionUpdated(ExperienceDescriptionUpdate),
    ExperienceSensationUpdated(ExperienceSensationUpdate),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (
                ExperienceEventsType::ExperienceCreated,
                "experience-created",
            ),
            (
                ExperienceEventsType::ExperienceDescriptionUpdated,
                "experience-description-updated",
            ),
            (
                ExperienceEventsType::ExperienceSensationUpdated,
                "experience-sensation-updated",
            ),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExperienceDescriptionUpdate {
    pub(crate) id: ExperienceId,
    pub(crate) description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExperienceSensationUpdate {
    pub(crate) id: ExperienceId,
    pub(crate) sensation: SensationName,
}
