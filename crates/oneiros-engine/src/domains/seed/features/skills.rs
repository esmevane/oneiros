use crate::Skill;

pub struct SeedSkills;

impl SeedSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("seed-core", include_str!("skills/core.md"))]
    }
}
