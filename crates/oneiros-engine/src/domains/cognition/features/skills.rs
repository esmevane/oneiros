use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![
        Skill::new("cognition-add", include_str!("skills/add.md")),
        Skill::new("cognition-show", include_str!("skills/show.md")),
        Skill::new("cognition-list", include_str!("skills/list.md")),
    ]
}
