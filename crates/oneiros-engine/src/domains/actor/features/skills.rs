use crate::{ActorRequestType, Skill};

pub(crate) struct ActorSkills;

impl ActorSkills {
    pub(crate) fn all() -> Vec<Skill> {
        ActorRequestType::all()
            .iter()
            .map(|kind| {
                let meta = kind.meta();
                Skill::builder()
                    .name(kind.to_string())
                    .content(meta.content)
                    .build()
            })
            .collect()
    }
}
