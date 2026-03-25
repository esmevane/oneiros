use crate::Skill;

pub struct ServiceSkills;

impl ServiceSkills {
    pub fn all() -> Vec<Skill> {
        vec![
            Skill::new("service-install", include_str!("skills/service-install.md")),
            Skill::new(
                "service-uninstall",
                include_str!("skills/service-uninstall.md"),
            ),
            Skill::new("service-start", include_str!("skills/service-start.md")),
            Skill::new("service-stop", include_str!("skills/service-stop.md")),
            Skill::new("service-status", include_str!("skills/service-status.md")),
            Skill::new("service-run", include_str!("skills/service-run.md")),
        ]
    }
}
