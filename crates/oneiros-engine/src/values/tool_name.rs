use core::fmt;

/// A typed tool name — derived from a request type's `Display` implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolName(String);

impl ToolName {
    pub fn new(name: impl fmt::Display) -> Self {
        Self(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ToolName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
