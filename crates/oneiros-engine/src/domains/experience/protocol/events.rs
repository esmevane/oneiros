use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ExperienceEventsType, display = "kebab-case")]
pub enum ExperienceEvents {
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
#[serde(untagged)]
pub enum ExperienceDescriptionUpdate {
    Current(ExperienceDescriptionUpdateV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ExperienceDescriptionUpdateV1 {
    pub id: ExperienceId,
    pub description: Description,
}

impl ExperienceDescriptionUpdate {
    pub fn build_v1() -> ExperienceDescriptionUpdateV1Builder {
        ExperienceDescriptionUpdateV1::builder()
    }

    pub fn id(&self) -> ExperienceId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn description(&self) -> &Description {
        match self {
            Self::Current(v) => &v.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExperienceSensationUpdate {
    Current(ExperienceSensationUpdateV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ExperienceSensationUpdateV1 {
    pub id: ExperienceId,
    pub sensation: SensationName,
}

impl ExperienceSensationUpdate {
    pub fn build_v1() -> ExperienceSensationUpdateV1Builder {
        ExperienceSensationUpdateV1::builder()
    }

    pub fn id(&self) -> ExperienceId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn sensation(&self) -> &SensationName {
        match self {
            Self::Current(v) => &v.sensation,
        }
    }
}
