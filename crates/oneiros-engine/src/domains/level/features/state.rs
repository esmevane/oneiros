use crate::*;

pub struct LevelState;

impl LevelState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Level(LevelEvents::LevelSet(level)) => {
                canon.levels.insert(level.name.to_string(), level.clone());
            }
            Events::Level(LevelEvents::LevelRemoved(removed)) => {
                canon.levels.remove(&removed.name.to_string());
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
        assert!(next.levels.is_empty());
    }
}
