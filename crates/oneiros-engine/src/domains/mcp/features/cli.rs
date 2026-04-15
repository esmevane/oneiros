use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum McpCommands {
    /// Write .mcp.json for Claude Code MCP integration.
    Init(InitMcp),
}

impl McpCommands {
    pub fn execute(&self, config: &Config) -> Result<Rendered<Responses>, McpConfigError> {
        let response = match self {
            McpCommands::Init(init) => {
                let result = McpConfigService::init(config, init)?;

                if let McpConfigResponse::McpConfigExists(ref path) = result {
                    if !init.yes {
                        let overwrite = inquire::Confirm::new(&format!(
                            "{} already exists. Overwrite?",
                            path.display()
                        ))
                        .with_default(false)
                        .prompt()
                        .unwrap_or(false);

                        if overwrite {
                            McpConfigService::write(config, init)?
                        } else {
                            result
                        }
                    } else {
                        result
                    }
                } else {
                    result
                }
            }
        };

        Ok(McpView::new(response).render().map(Into::into))
    }
}
