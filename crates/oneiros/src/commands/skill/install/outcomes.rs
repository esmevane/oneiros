use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InstallSkillOutcomes {
    #[outcome(message("Wrote {0}"), level = "debug")]
    FileWritten(String),
    #[outcome(message("Created AGENTS.md"))]
    AgentsMdCreated,
    #[outcome(message("Updated AGENTS.md"))]
    AgentsMdUpdated,
    #[outcome(message("AGENTS.md already contains oneiros section"), level = "debug")]
    AgentsMdSkipped,
    #[outcome(message("Skill installed to {0}"))]
    Installed(String),
}
