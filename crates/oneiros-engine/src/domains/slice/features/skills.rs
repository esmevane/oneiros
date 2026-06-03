use crate::Skill;

pub(crate) struct SliceSkills;

impl SliceSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("slice-create", include_str!("skills/create.md")),
            Skill::new("slice-list", include_str!("skills/list.md")),
            Skill::new("slice-delete", include_str!("skills/delete.md")),
            Skill::new("slice-diff", include_str!("skills/diff.md")),
        ]
    }
}
