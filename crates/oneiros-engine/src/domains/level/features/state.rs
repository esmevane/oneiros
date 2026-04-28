use crate::*;

pub struct LevelState;

impl LevelState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Level(level_event) = event {
            match level_event {
                LevelEvents::LevelSet(setting) => {
                    if let Ok(current) = setting.current() {
                        canon.levels.set(&current.level);
                    }
                }
                LevelEvents::LevelRemoved(removal) => {
                    if let Ok(current) = removal.current() {
                        canon.levels.remove(&current.name);
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
    fn sets_and_removes_level() {
        let canon = BrainCanon::default();
        let name = LevelName::new("working");
        let level = Level::builder()
            .name(name.clone())
            .description("Short-term")
            .prompt("")
            .build();
        let event = Events::Level(LevelEvents::LevelSet(
            LevelSet::builder_v1().level(level).build().into(),
        ));

        let next = LevelState::reduce(canon, &event);
        assert_eq!(next.levels.len(), 1);

        let event = Events::Level(LevelEvents::LevelRemoved(
            LevelRemoved::builder_v1().name(name).build().into(),
        ));
        let next = LevelState::reduce(next, &event);
        assert_eq!(next.levels.len(), 0);
    }
}
