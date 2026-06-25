use crate::Skill;

pub(crate) struct ActorSkills;

impl ActorSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("create-actor", include_str!("skills/create.md")),
            Skill::new("get-actor", include_str!("skills/get.md")),
            Skill::new("list-actors", include_str!("skills/list.md")),
        ]
    }
}
