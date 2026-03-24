use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![Skill::new("system-init", include_str!("skills/init.md"))]
}
