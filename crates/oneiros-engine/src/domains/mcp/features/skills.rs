use crate::Skill;

pub(crate) struct McpConfigSkills;

impl McpConfigSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![Skill::new("mcp-init", include_str!("skills/mcp-init.md"))]
    }
}
