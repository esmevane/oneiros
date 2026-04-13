use crate::Skill;

pub(crate) struct StorageSkills;

impl StorageSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("storage-set", include_str!("skills/set.md")),
            Skill::new("storage-show", include_str!("skills/show.md")),
            Skill::new("storage-list", include_str!("skills/list.md")),
            Skill::new("storage-remove", include_str!("skills/remove.md")),
        ]
    }
}
