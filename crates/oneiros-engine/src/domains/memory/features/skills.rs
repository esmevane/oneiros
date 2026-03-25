use crate::Skill;

pub struct MemorySkills;

impl MemorySkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("memory-add", include_str!("skills/add.md")),
            Skill::new("memory-show", include_str!("skills/show.md")),
            Skill::new("memory-list", include_str!("skills/list.md")),
        ]
    }
}
