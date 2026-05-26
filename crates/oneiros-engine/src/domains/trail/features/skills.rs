use crate::Skill;

pub(crate) struct TrailSkills;

impl TrailSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("trail-of", include_str!("skills/of.md")),
            Skill::new("trail-from", include_str!("skills/from.md")),
        ]
    }
}
