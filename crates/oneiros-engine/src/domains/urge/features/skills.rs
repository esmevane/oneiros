use crate::Skill;

pub(crate) struct UrgeSkills;

impl UrgeSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("urge-set", include_str!("skills/set.md")),
            Skill::new("urge-show", include_str!("skills/show.md")),
            Skill::new("urge-list", include_str!("skills/list.md")),
            Skill::new("urge-remove", include_str!("skills/remove.md")),
        ]
    }
}
