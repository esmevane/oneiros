use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use oneiros_templates::McpTemplate;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InitProjectOutcomes {
    #[outcome(message("Brain '{0}' created."))]
    BrainCreated(BrainName),
    #[outcome(message("Brain '{0}' already exists."))]
    BrainAlreadyExists(BrainName),
    #[outcome(message("MCP config written to {}", .0.display()))]
    McpConfigWritten(std::path::PathBuf),
    #[outcome(message("MCP config already exists at {}, skipping.", .0.display()))]
    McpConfigExists(std::path::PathBuf),
    #[outcome(message("Added .mcp.json to .gitignore."))]
    GitignoreUpdated,
}

#[derive(Clone, Args, bon::Builder)]
pub struct InitProject {
    /// Accept defaults, no prompting.
    #[arg(short, long)]
    #[builder(default)]
    yes: bool,
}

impl InitProject {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<InitProjectOutcomes>, ProjectCommandError> {
        let mut outcomes = Outcomes::new();

        let project_name = BrainName::new(
            context
                .project_name()
                .ok_or(ProjectCommandError::NoProject)?,
        );

        let client = context.client();

        let request = CreateBrainRequest {
            name: project_name.clone(),
        };

        let token = match client.create_brain(request).await {
            Ok(response) => {
                let info: BrainInfo = response.data()?;
                context.store_ticket(project_name.as_str(), info.token.as_str())?;
                outcomes.emit(InitProjectOutcomes::BrainCreated(project_name.clone()));
                info.token
            }
            Err(oneiros_client::Error::ServiceResponse(ref e)) if e.status == 409 => {
                outcomes.emit(InitProjectOutcomes::BrainAlreadyExists(
                    project_name.clone(),
                ));
                context.ticket_token()?
            }
            Err(error) => return Err(error.into()),
        };

        if let Some(project_root) = context.project_root() {
            let mcp_path = project_root.join(".mcp.json");

            if mcp_path.exists() {
                outcomes.emit(InitProjectOutcomes::McpConfigExists(mcp_path));
            } else if self.yes
                || context
                    .terminal()
                    .confirm("Generate .mcp.json for AI agent integration?", true)
            {
                let addr = context.config().service_addr();
                let mcp_json = McpTemplate::new(&addr, token.as_str()).to_string();
                context.files().write(&mcp_path, mcp_json)?;
                outcomes.emit(InitProjectOutcomes::McpConfigWritten(mcp_path));

                if self.yes
                    || context
                        .terminal()
                        .confirm("Add .mcp.json to .gitignore? (contains auth token)", true)
                {
                    let gitignore = project_root.join(".gitignore");
                    let files = context.files();

                    if !files.contains(&gitignore, ".mcp.json")? {
                        let needs_newline = files
                            .read_to_string(&gitignore)
                            .is_ok_and(|c| !c.is_empty() && !c.ends_with('\n'));

                        let entry = if needs_newline {
                            "\n.mcp.json\n"
                        } else {
                            ".mcp.json\n"
                        };

                        files.append(&gitignore, entry)?;
                    }

                    outcomes.emit(InitProjectOutcomes::GitignoreUpdated);
                }
            }
        }

        Ok(outcomes)
    }
}
