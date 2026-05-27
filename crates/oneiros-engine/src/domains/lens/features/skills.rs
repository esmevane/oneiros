use crate::Skill;

pub(crate) struct LensSkills;

impl LensSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("lens-parse", include_str!("skills/parse.md")),
            Skill::new("lens-explain", include_str!("skills/explain.md")),
            Skill::new("lens-query", include_str!("skills/query.md")),
        ]
    }
}
