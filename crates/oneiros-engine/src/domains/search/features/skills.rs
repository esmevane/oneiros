use crate::Skill;

pub(crate) struct SearchSkills;

impl SearchSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("search", include_str!("skills/search.md"))]
    }
}
