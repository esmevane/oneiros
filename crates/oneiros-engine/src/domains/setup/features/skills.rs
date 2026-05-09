use crate::Skill;

pub(crate) struct SetupSkills;

impl SetupSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("setup", include_str!("skills/setup.md"))]
    }
}
