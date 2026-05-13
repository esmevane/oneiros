use crate::*;

pub(crate) struct CognitionState;

impl CognitionState {
    pub(crate) fn reduce(mut canon: ProjectCanon, event: &Events) -> ProjectCanon {
        if let Events::Cognition(CognitionEvents::CognitionAdded(added)) = event
            && let Ok(current) = added.current()
        {
            canon.cognitions.set(&current.cognition);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<ProjectCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_cognition() {
        let canon = ProjectCanon::default();
        let cognition = Cognition::builder()
            .agent_id(AgentId::new())
            .texture("observation")
            .content("Something noticed")
            .build();
        let added = CognitionAdded::builder_v1()
            .cognition(cognition)
            .build()
            .into();
        let event = Events::Cognition(CognitionEvents::CognitionAdded(added));

        let next = CognitionState::reduce(canon, &event);

        assert_eq!(next.cognitions.len(), 1);
    }
}
