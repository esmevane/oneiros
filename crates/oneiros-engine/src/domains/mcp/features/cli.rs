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
            McpCommands::Init(initialization) => {
                let InitMcp::V1(init) = initialization;
                let result = McpConfigService::init(config, initialization)?;

                if let McpConfigResponse::McpConfigExists(_) = &result {
                    if !init.yes {
                        let path = match &result {
                            McpConfigResponse::McpConfigExists(McpConfigExistsResponse::V1(d)) => {
                                d.path.clone()
                            }
                            _ => unreachable!(),
                        };
                        let overwrite = inquire::Confirm::new(&format!(
                            "{} already exists. Overwrite?",
                            path.display()
                        ))
                        .with_default(false)
                        .prompt()
                        .unwrap_or(false);

                        if overwrite {
                            McpConfigService::write(config, initialization)?
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
