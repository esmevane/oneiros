use crate::Description;

/// A concrete MCP resource — fixed URI, no parameters.
///
/// Listed in `resources/list`. The agent reads it directly.
pub(crate) struct ResourceDef {
    pub(crate) uri: String,
    pub(crate) name: String,
    pub(crate) description: Description,
    pub(crate) mime_type: String,
}

impl ResourceDef {
    pub(crate) fn new(
        uri: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<Description>,
    ) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
            description: description.into(),
            mime_type: "text/markdown".to_string(),
        }
    }
}

/// A parameterized MCP resource template — URI with placeholders.
///
/// Listed in `resources/templates/list`. The agent fills in
/// parameters to form a concrete URI for `resources/read`.
pub(crate) struct ResourceTemplateDef {
    pub(crate) uri_template: String,
    pub(crate) name: String,
    pub(crate) description: Description,
    pub(crate) mime_type: String,
}

impl ResourceTemplateDef {
    pub(crate) fn new(
        uri_template: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<Description>,
    ) -> Self {
        Self {
            uri_template: uri_template.into(),
            name: name.into(),
            description: description.into(),
            mime_type: "text/markdown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_def_defaults_to_markdown() {
        let def = ResourceDef::new(
            "oneiros-mcp://agents",
            "agents",
            "All agents in the current project",
        );
        assert_eq!(def.mime_type, "text/markdown");
        assert_eq!(def.uri, "oneiros-mcp://agents");
    }

    #[test]
    fn resource_template_def_defaults_to_markdown() {
        let def = ResourceTemplateDef::new(
            "oneiros-mcp://agent/{name}/status",
            "agent-status",
            "Dashboard for a specific agent",
        );
        assert_eq!(def.mime_type, "text/markdown");
        assert!(def.uri_template.contains("{name}"));
    }
}
