use crate::Skill;

pub(crate) struct SystemSkills;

impl SystemSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("system-init", include_str!("skills/init.md"))]
    }
}
