use crate::Skill;

pub(crate) struct ExperienceSkills;

impl ExperienceSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("experience-create", include_str!("skills/create.md")),
            Skill::new("experience-show", include_str!("skills/show.md")),
            Skill::new("experience-list", include_str!("skills/list.md")),
            Skill::new("experience-update", include_str!("skills/update.md")),
        ]
    }
}
