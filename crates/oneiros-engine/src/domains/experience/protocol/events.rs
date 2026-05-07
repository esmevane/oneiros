use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ExperienceEventsType, display = "kebab-case")]
pub(crate) enum ExperienceEvents {
    ExperienceCreated(ExperienceCreated),
    ExperienceDescriptionUpdated(ExperienceDescriptionUpdated),
    ExperienceSensationUpdated(ExperienceSensationUpdated),
}

versioned! {
    pub(crate) enum ExperienceCreated {
        V1 => {
            #[serde(flatten)] pub(crate) experience: Experience,
        }
    }
}

versioned! {
    pub(crate) enum ExperienceDescriptionUpdated {
        V1 => {
            pub(crate) id: ExperienceId,
            #[builder(into)] pub(crate) description: Description,
        }
    }
}

versioned! {
    pub(crate) enum ExperienceSensationUpdated {
        V1 => {
            pub(crate) id: ExperienceId,
            #[builder(into)] pub(crate) sensation: SensationName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_experience() -> Experience {
        Experience::builder()
            .agent_id(AgentId::new())
            .sensation("echoes")
            .description("Resonance noticed")
            .build()
    }

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

    #[test]
    fn experience_created_wire_format_is_flat() {
        let event =
            ExperienceEvents::ExperienceCreated(ExperienceCreated::V1(ExperienceCreatedV1 {
                experience: sample_experience(),
            }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "experience-created");
        assert!(
            json["data"].get("experience").is_none(),
            "flatten must elide the experience envelope on the wire"
        );
        assert_eq!(json["data"]["sensation"], "echoes");
        assert_eq!(json["data"]["description"], "Resonance noticed");
        assert!(json["data"].get("id").is_some());
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
