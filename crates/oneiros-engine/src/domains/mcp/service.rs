use std::path::PathBuf;

use crate::*;

pub(crate) struct McpConfigService;

impl McpConfigService {
    pub(crate) fn init(config: &Config, request: &InitMcp) -> Result<McpConfigResponse, McpConfigError> {
        let token = request
            .token
            .clone()
            .or_else(|| config.token())
            .ok_or(McpConfigError::NoToken)?;

        let address = request.address.unwrap_or(config.service_addr());

        let mcp_json = serde_json::json!({
            "mcpServers": {
                "oneiros-local": {
                    "type": "http",
                    "url": format!("http://{address}/mcp"),
                    "headers": {
                        "Authorization": format!("Bearer {token}")
                    }
                }
            }
        });

        let path = Self::mcp_json_path();

        if path.exists() && !request.yes {
            // In non-interactive contexts (like setup), the caller handles
            // the prompt. Here we just report the file exists.
            return Ok(McpConfigResponse::McpConfigExists(path));
        }

        let content = serde_json::to_string_pretty(&mcp_json)?;
        std::fs::write(&path, content)?;

        Ok(McpConfigResponse::McpConfigWritten(path))
    }

    /// Write the .mcp.json regardless of whether it exists.
    /// Used by setup after the user confirms.
    pub(crate) fn write(config: &Config, request: &InitMcp) -> Result<McpConfigResponse, McpConfigError> {
        let mut forced = request.clone();
        forced.yes = true;
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
