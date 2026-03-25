use crate::Skill;

pub struct DoctorSkills;

impl DoctorSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("doctor", include_str!("skills/doctor.md"))]
    }
}
