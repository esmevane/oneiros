use crate::Skill;

pub(crate) struct BrainSkills;

impl BrainSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("brain-create", include_str!("skills/create.md")),
            Skill::new("brain-get", include_str!("skills/get.md")),
            Skill::new("brain-list", include_str!("skills/list.md")),
        ]
    }
}
