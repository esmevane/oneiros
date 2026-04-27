use crate::*;

pub struct ExperienceState;

impl ExperienceState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Experience(experience_event) = event {
            match experience_event {
                ExperienceEvents::ExperienceCreated(experience) => {
                    canon.experiences.set(experience);
                }
                ExperienceEvents::ExperienceDescriptionUpdated(update) => {
                    if let Some(exp) = canon.experiences.get_mut(update.id()) {
                        exp.set_description(update.description().clone());
                    }
                }
                ExperienceEvents::ExperienceSensationUpdated(update) => {
                    if let Some(exp) = canon.experiences.get_mut(update.id()) {
                        exp.set_sensation(update.sensation().clone());
                    }
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_experience_description() {
        let mut canon = BrainCanon::default();
        let experience = Experience::Current(
            Experience::build_v1()
                .agent_id(AgentId::new())
                .sensation("distills")
                .description("Original description")
                .build(),
        );
        canon.experiences.set(&experience);

        let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdate::Current(ExperienceDescriptionUpdateV1 {
                id: experience.id(),
                description: Description::new("Updated description"),
            }),
        ));
        let next = ExperienceState::reduce(canon, &event);

        let updated = next.experiences.get(experience.id()).unwrap();
        assert_eq!(
            updated.description(),
            &Description::new("Updated description")
        );
    }
}
