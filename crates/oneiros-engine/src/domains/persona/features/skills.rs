use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![
        Skill::new("persona-set", include_str!("skills/set.md")),
        Skill::new("persona-show", include_str!("skills/show.md")),
        Skill::new("persona-list", include_str!("skills/list.md")),
        Skill::new("persona-remove", include_str!("skills/remove.md")),
    ]
}
