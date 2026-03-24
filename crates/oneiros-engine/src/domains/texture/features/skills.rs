use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![
        Skill::new("texture-set", include_str!("skills/set.md")),
        Skill::new("texture-show", include_str!("skills/show.md")),
        Skill::new("texture-list", include_str!("skills/list.md")),
        Skill::new("texture-remove", include_str!("skills/remove.md")),
    ]
}
