use crate::Skill;

pub(crate) struct BookmarkSkills;

impl BookmarkSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("bookmark-create", include_str!("skills/create.md")),
            Skill::new("bookmark-switch", include_str!("skills/switch.md")),
            Skill::new("bookmark-merge", include_str!("skills/merge.md")),
            Skill::new("bookmark-list", include_str!("skills/list.md")),
            Skill::new("bookmark-share", include_str!("skills/share.md")),
            Skill::new("bookmark-follow", include_str!("skills/follow.md")),
            Skill::new("bookmark-collect", include_str!("skills/collect.md")),
            Skill::new("bookmark-unfollow", include_str!("skills/unfollow.md")),
            Skill::new("bookmark-push", include_str!("skills/push.md")),
        ]
    }
}
