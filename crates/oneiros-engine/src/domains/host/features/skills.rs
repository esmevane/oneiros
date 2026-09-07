use crate::Skill;

pub(crate) struct HostServiceSkills;

impl HostServiceSkills {
    pub(crate) fn all() -> Vec<Skill> {
        vec![
            Skill::new("host-install", include_str!("skills/install.md")),
            Skill::new("host-uninstall", include_str!("skills/uninstall.md")),
            Skill::new("host-start", include_str!("skills/start.md")),
            Skill::new("host-stop", include_str!("skills/stop.md")),
            Skill::new("host-status", include_str!("skills/status.md")),
            Skill::new("host-run", include_str!("skills/run.md")),
        ]
    }
}
