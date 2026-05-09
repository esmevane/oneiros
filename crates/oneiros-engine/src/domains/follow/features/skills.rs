use crate::Skill;

pub(crate) struct FollowSkills;

impl FollowSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("follow-get", include_str!("skills/get.md")),
            Skill::new("follow-list", include_str!("skills/list.md")),
        ]
    }
}
