use crate::Skill;

pub(crate) struct TicketSkills;

impl TicketSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("ticket-issue", include_str!("skills/issue.md")),
            Skill::new("ticket-validate", include_str!("skills/validate.md")),
            Skill::new("ticket-list", include_str!("skills/list.md")),
        ]
    }
}
