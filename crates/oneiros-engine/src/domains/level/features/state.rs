use crate::*;

pub(crate) struct LevelState;

impl LevelState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Level(level_event) = event {
            match level_event {
                LevelEvents::LevelSet(level) => {
                    canon.levels.set(level);
                }
                LevelEvents::LevelRemoved(removed) => {
                    canon.levels.remove(&removed.name);
                }
            };
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
    fn sets_and_removes_level() {
        let canon = BrainCanon::default();
        let level = Level::builder()
            .name("working")
            .description("Short-term")
            .prompt("")
            .build();
        let event = Events::Level(LevelEvents::LevelSet(level.clone()));

        let next = LevelState::reduce(canon, &event);
        assert_eq!(next.levels.len(), 1);

        let event = Events::Level(LevelEvents::LevelRemoved(LevelRemoved {
            name: level.name.clone(),
        }));
        let next = LevelState::reduce(next, &event);
        assert_eq!(next.levels.len(), 0);
    }
}
