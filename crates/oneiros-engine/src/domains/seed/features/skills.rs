use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![Skill::new("seed-core", include_str!("skills/core.md"))]
}
