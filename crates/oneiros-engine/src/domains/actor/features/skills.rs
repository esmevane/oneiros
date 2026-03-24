use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![
        Skill::new("actor-create", include_str!("skills/create.md")),
        Skill::new("actor-get", include_str!("skills/get.md")),
        Skill::new("actor-list", include_str!("skills/list.md")),
    ]
}
