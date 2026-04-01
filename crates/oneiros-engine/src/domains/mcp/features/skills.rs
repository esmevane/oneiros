use crate::Skill;

pub struct McpConfigSkills;

impl McpConfigSkills {
    pub fn all() -> Vec<Skill> {
        vec![Skill::new("mcp-init", include_str!("skills/mcp-init.md"))]
    }
}
