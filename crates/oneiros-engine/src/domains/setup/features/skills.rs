use crate::Skill;

pub struct SetupSkills;

impl SetupSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("setup", include_str!("skills/setup.md"))]
    }
}
