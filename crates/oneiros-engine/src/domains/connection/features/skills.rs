use crate::Skill;

pub(crate) struct ConnectionSkills;

impl ConnectionSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("connection-create", include_str!("skills/create.md")),
            Skill::new("connection-show", include_str!("skills/show.md")),
            Skill::new("connection-list", include_str!("skills/list.md")),
            Skill::new("connection-remove", include_str!("skills/remove.md")),
        ]
    }
}
