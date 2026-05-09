use crate::Skill;

pub(crate) struct PressureSkills;

impl PressureSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("pressure", include_str!("skills/pressure.md"))]
    }
}
