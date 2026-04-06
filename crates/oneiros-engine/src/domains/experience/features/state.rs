use crate::*;

pub struct ExperienceState;

impl ExperienceState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Experience(ExperienceEvents::ExperienceCreated(experience)) => {
                canon
                    .experiences
                    .insert(experience.id.to_string(), experience.clone());
            }
            Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(update)) => {
                if let Some(exp) = canon.experiences.get_mut(&update.id.to_string()) {
                    exp.description = update.description.clone();
                }
            }
            Events::Experience(ExperienceEvents::ExperienceSensationUpdated(update)) => {
                if let Some(exp) = canon.experiences.get_mut(&update.id.to_string()) {
                    exp.sensation = update.sensation.clone();
                }
            }
            _ => {}
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
        let experience = Experience::builder()
            .agent_id(AgentId::new())
            .sensation("distills")
            .description("Original description")
            .build();
        canon
            .experiences
            .insert(experience.id.to_string(), experience.clone());

        let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdate {
                id: experience.id,
                description: Description::new("Updated description"),
            },
        ));
        let next = ExperienceState::reduce(canon, &event);

        let updated = &next.experiences[&experience.id.to_string()];
        assert_eq!(updated.description, Description::new("Updated description"));
    }
}
