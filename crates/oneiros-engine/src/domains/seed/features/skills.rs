use crate::Skill;

pub(crate) struct SeedSkills;

impl SeedSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("seed-core", include_str!("skills/core.md")),
            Skill::new("seed-agents", include_str!("skills/agents.md")),
        ]
    }
}
