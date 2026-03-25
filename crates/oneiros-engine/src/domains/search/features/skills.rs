use crate::Skill;

pub struct SearchSkills;

impl SearchSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("search", include_str!("skills/search.md"))]
    }
}
