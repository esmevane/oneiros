use crate::Skill;

pub(crate) struct PeerSkills;

impl PeerSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("peer-add", include_str!("skills/add.md")),
            Skill::new("peer-get", include_str!("skills/get.md")),
            Skill::new("peer-list", include_str!("skills/list.md")),
            Skill::new("peer-remove", include_str!("skills/remove.md")),
        ]
    }
}
