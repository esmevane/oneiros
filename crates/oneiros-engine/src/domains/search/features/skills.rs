use crate::Skill;

pub fn skills() -> Vec<Skill> {
    vec![Skill::new("search", include_str!("skills/search.md"))]
}
