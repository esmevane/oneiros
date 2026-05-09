use crate::Skill;

pub(crate) struct SensationSkills;

impl SensationSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("sensation-set", include_str!("skills/set.md")),
            Skill::new("sensation-show", include_str!("skills/show.md")),
            Skill::new("sensation-list", include_str!("skills/list.md")),
            Skill::new("sensation-remove", include_str!("skills/remove.md")),
        ]
    }
}
