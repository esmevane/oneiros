use crate::*;

pub(crate) struct CognitionState;

impl CognitionState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Cognition(CognitionEvents::CognitionAdded(cognition)) = event {
            canon.cognitions.set(cognition);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_cognition() {
        let canon = BrainCanon::default();
        let cognition = Cognition::builder()
            .agent_id(AgentId::new())
            .texture("observation")
            .content("Something noticed")
            .build();
        let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));

        let next = CognitionState::reduce(canon, &event);

        assert_eq!(next.cognitions.len(), 1);
    }
}
