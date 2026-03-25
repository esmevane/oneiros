use crate::Skill;

pub struct TicketSkills;

impl TicketSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("ticket-issue", include_str!("skills/issue.md")),
            Skill::new("ticket-validate", include_str!("skills/validate.md")),
            Skill::new("ticket-list", include_str!("skills/list.md")),
        ]
    }
}
