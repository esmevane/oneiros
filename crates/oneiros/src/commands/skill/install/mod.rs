mod outcomes;

use clap::Args;
use directories::BaseDirs;
use oneiros_outcomes::Outcomes;
use oneiros_skill::artifacts;
use std::path::Path;

pub(crate) use outcomes::InstallSkillOutcomes;

use crate::*;

/// Install the oneiros skill for Claude Code.
#[derive(Clone, Args)]
pub(crate) struct InstallSkill {
    /// Install to ~/.claude/ instead of the current project.
    #[arg(long)]
    global: bool,
}

impl InstallSkill {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<InstallSkillOutcomes>, SkillCommandError> {
        let mut outcomes = Outcomes::new();
        let files = context.files();

        let base = if self.global {
            let dirs = BaseDirs::new().ok_or(SkillCommandError::NoHomeDir)?;
            dirs.home_dir().join(".claude")
        } else {
            let root = context.project_root().ok_or(SkillCommandError::NoProject)?;
            root.join(".claude")
        };

        for artifact in artifacts::all() {
            let dest = base.join(artifact.path);

            if let Some(parent) = dest.parent() {
                files.ensure_dir(parent)?;
            }

            files.write(&dest, &artifact.content)?;
            outcomes.emit(InstallSkillOutcomes::FileWritten(artifact.path.to_string()));
        }

        if !self.global
            && let Some(root) = context.project_root()
        {
            let agents_md = root.join("AGENTS.md");
            let section = artifacts::AGENTS_MD_SECTION;

            if agents_md.exists() {
                let existing = files.read_to_string(&agents_md)?;
                if existing.contains("## Oneiros") {
                    outcomes.emit(InstallSkillOutcomes::AgentsMdSkipped);
                } else {
                    let updated = format!("{}\n\n{}", existing.trim_end(), section);
                    files.write(&agents_md, updated)?;
                    outcomes.emit(InstallSkillOutcomes::AgentsMdUpdated);
                }
            } else {
                files.write(&agents_md, section)?;
                outcomes.emit(InstallSkillOutcomes::AgentsMdCreated);
            }
        }

        let display_base = display_path(&base);
        outcomes.emit(InstallSkillOutcomes::Installed(display_base));

        Ok(outcomes)
    }
}

fn display_path(path: &Path) -> String {
    if let Some(home) = BaseDirs::new().map(|d| d.home_dir().to_path_buf())
        && let Ok(relative) = path.strip_prefix(&home)
    {
        return format!("~/{}", relative.display());
    }
    path.display().to_string()
}
