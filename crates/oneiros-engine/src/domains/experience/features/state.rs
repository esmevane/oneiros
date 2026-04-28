use crate::*;

pub struct ExperienceState;

impl ExperienceState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Experience(experience_event) = event {
            match experience_event {
                ExperienceEvents::ExperienceCreated(created) => {
                    if let Ok(current) = created.current() {
                        canon.experiences.set(&current.experience);
                    }
                }
                ExperienceEvents::ExperienceDescriptionUpdated(updated) => {
                    if let Ok(current) = updated.current()
                        && let Some(experience) = canon.experiences.get_mut(current.id)
                    {
                        experience.description = current.description;
                    }
                }
                ExperienceEvents::ExperienceSensationUpdated(updated) => {
                    if let Ok(current) = updated.current()
                        && let Some(experience) = canon.experiences.get_mut(current.id)
                    {
                        experience.sensation = current.sensation;
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
        let experience = Experience::builder()
            .agent_id(AgentId::new())
            .sensation("distills")
            .description("Original description")
            .build();
        canon.experiences.set(&experience);

        let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdated::builder_v1()
                .id(experience.id)
                .description(Description::new("Updated description"))
                .build()
                .into(),
        ));
        let next = ExperienceState::reduce(canon, &event);

        let updated = next.experiences.get(experience.id).unwrap();
        assert_eq!(updated.description, Description::new("Updated description"));
    }
}
