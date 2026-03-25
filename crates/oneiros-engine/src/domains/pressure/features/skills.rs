use crate::Skill;

pub struct PressureSkills;

impl PressureSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("pressure", include_str!("skills/pressure.md"))]
    }
}
