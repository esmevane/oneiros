use crate::Skill;

pub struct TenantSkills;

impl TenantSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("tenant-create", include_str!("skills/create.md")),
            Skill::new("tenant-get", include_str!("skills/get.md")),
            Skill::new("tenant-list", include_str!("skills/list.md")),
        ]
    }
}
