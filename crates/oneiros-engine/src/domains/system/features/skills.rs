use crate::Skill;

pub struct SystemSkills;

impl SystemSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("system-init", include_str!("skills/init.md"))]
    }
}
