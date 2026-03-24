use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![Skill::new("doctor", include_str!("skills/doctor.md"))]
}
