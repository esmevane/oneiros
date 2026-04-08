use crate::Skill;

pub struct BookmarkSkills;

impl BookmarkSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("bookmark-create", include_str!("skills/create.md")),
            Skill::new("bookmark-switch", include_str!("skills/switch.md")),
            Skill::new("bookmark-merge", include_str!("skills/merge.md")),
            Skill::new("bookmark-list", include_str!("skills/list.md")),
        ]
    }
}
