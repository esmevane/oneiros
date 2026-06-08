use crate::*;

pub(crate) struct RemoteSkills;

impl RemoteSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("remote-add", include_str!("skills/add.md")),
            Skill::new("remote-share", include_str!("skills/share.md")),
            Skill::new("remote-list", include_str!("skills/list.md")),
            Skill::new("remote-remove", include_str!("skills/remove.md")),
            Skill::new("remote-bookmarks", include_str!("skills/bookmarks.md")),
        ]
    }
}
