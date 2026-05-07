use crate::Skill;

pub(crate) struct DoctorSkills;

impl DoctorSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("doctor", include_str!("skills/doctor.md"))]
    }
}
