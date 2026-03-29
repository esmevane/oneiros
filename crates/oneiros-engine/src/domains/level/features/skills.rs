use crate::Skill;

pub struct LevelSkills;

impl LevelSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("level-set", include_str!("skills/set.md")),
            Skill::new("level-show", include_str!("skills/show.md")),
            Skill::new("level-list", include_str!("skills/list.md")),
            Skill::new("level-remove", include_str!("skills/remove.md")),
        ]
    }
}
