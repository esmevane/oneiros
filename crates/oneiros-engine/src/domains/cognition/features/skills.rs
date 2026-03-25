use crate::Skill;

pub struct CognitionSkills;

impl CognitionSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("cognition-add", include_str!("skills/add.md")),
            Skill::new("cognition-show", include_str!("skills/show.md")),
            Skill::new("cognition-list", include_str!("skills/list.md")),
        ]
    }
}
