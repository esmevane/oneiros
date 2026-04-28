use crate::*;

pub struct CognitionState;

impl CognitionState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Cognition(CognitionEvents::CognitionAdded(added)) = event
            && let Ok(current) = added.current()
        {
            canon.cognitions.set(&current.cognition);
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
    fn adds_cognition() {
        let canon = BrainCanon::default();
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
