use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![Skill::new("pressure", include_str!("skills/pressure.md"))]
}
