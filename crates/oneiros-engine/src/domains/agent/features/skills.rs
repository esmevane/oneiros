use crate::Skill;

pub struct AgentSkills;

impl AgentSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("agent-create", include_str!("skills/create.md")),
            Skill::new("agent-show", include_str!("skills/show.md")),
            Skill::new("agent-list", include_str!("skills/list.md")),
            Skill::new("agent-update", include_str!("skills/update.md")),
            Skill::new("agent-remove", include_str!("skills/remove.md")),
        ]
    }
}
