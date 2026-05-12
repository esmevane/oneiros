use std::path::PathBuf;

use crate::*;

pub(crate) struct McpConfigService;

impl McpConfigService {
    pub(crate) fn init(config: &Config, request: &InitMcp) -> Result<McpResponses, McpConfigError> {
        let details = request.current()?;
        let token = details
            .token
            .clone()
            .or_else(|| config.token())
            .ok_or(McpConfigError::NoToken)?;

        let address = details.address.unwrap_or(config.service_addr());
        let scheme = &config.service.scheme;

        let mcp_json = serde_json::json!({
            "mcpServers": {
                "oneiros-local": {
                    "type": "http",
                    "url": format!("{scheme}://{address}/mcp"),
                    "headers": {
                        "Authorization": format!("Bearer {token}")
                    }
                }
            }
        });

        let path = Self::mcp_json_path();

        if path.exists() && !details.yes {
            // In non-interactive contexts (like setup), the caller handles
            // the prompt. Here we just report the file exists.
            return Ok(McpResponses::McpConfigExists(
                McpConfigExistsResponse::builder_v1()
                    .path(path)
                    .build()
                    .into(),
            ));
        }

        let content = serde_json::to_string_pretty(&mcp_json)?;
        config.platform().write(&path, content)?;

        Ok(McpResponses::McpConfigWritten(
            McpConfigWrittenResponse::builder_v1()
                .path(path)
                .build()
                .into(),
        ))
    }

    /// Write the .mcp.json regardless of whether it exists.
    /// Used by setup after the user confirms.
    pub(crate) fn write(
        config: &Config,
        request: &InitMcp,
    ) -> Result<McpResponses, McpConfigError> {
        let details = request.current()?;
        let forced: InitMcp = InitMcp::builder_v1()
            .maybe_token(details.token.clone())
            .maybe_address(details.address)
            .yes(true)
            .build()
            .into();
        Self::init(config, &forced)
    }

    /// The path to .mcp.json in the current working directory.
    pub(crate) fn mcp_json_path() -> PathBuf {
        std::env::current_dir()
            .unwrap_or_default()
            .join(".mcp.json")
    }

    /// Check whether .mcp.json exists.
    pub(crate) fn is_configured() -> bool {
        Self::mcp_json_path().exists()
    }
}
