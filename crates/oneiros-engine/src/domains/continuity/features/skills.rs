use crate::Skill;

pub(crate) struct ContinuitySkills;

impl ContinuitySkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("wake", include_str!("skills/wake.md")),
            Skill::new("dream", include_str!("skills/dream.md")),
            Skill::new("introspect", include_str!("skills/introspect.md")),
            Skill::new("reflect", include_str!("skills/reflect.md")),
            Skill::new("sense", include_str!("skills/sense.md")),
            Skill::new("sleep", include_str!("skills/sleep.md")),
            Skill::new("guidebook", include_str!("skills/guidebook.md")),
            Skill::new("emerge", include_str!("skills/emerge.md")),
            Skill::new("recede", include_str!("skills/recede.md")),
            Skill::new("status", include_str!("skills/status.md")),
        ]
    }
}
