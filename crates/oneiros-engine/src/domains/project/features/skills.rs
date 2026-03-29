use crate::Skill;

pub struct ProjectSkills;

impl ProjectSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("project-init", include_str!("skills/init.md")),
            Skill::new("project-export", include_str!("skills/export.md")),
            Skill::new("project-import", include_str!("skills/import.md")),
            Skill::new("project-replay", include_str!("skills/replay.md")),
        ]
    }
}
