use crate::Skill;

pub(crate) struct NatureSkills;

impl NatureSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("nature-set", include_str!("skills/set.md")),
            Skill::new("nature-show", include_str!("skills/show.md")),
            Skill::new("nature-list", include_str!("skills/list.md")),
            Skill::new("nature-remove", include_str!("skills/remove.md")),
        ]
    }
}
