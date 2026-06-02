use crate::Skill;

pub(crate) struct SliceSkills;

impl SliceSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("slice-create", include_str!("skills/create.md"))]
    }
}
