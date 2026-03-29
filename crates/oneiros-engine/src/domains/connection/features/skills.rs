use crate::Skill;

pub struct ConnectionSkills;

impl ConnectionSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("connection-create", include_str!("skills/create.md")),
            Skill::new("connection-show", include_str!("skills/show.md")),
            Skill::new("connection-list", include_str!("skills/list.md")),
            Skill::new("connection-remove", include_str!("skills/remove.md")),
        ]
    }
}
