//! Detail view — a heading with labeled fields.
//!
//! Used for single-entity display (agent show, memory show, etc.).

use crate::*;

/// A labeled field in a detail view.
pub struct Field {
    pub label: String,
    pub value: String,
}

impl Field {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// A single-entity detail view.
///
/// Renders as a styled heading followed by indented labeled fields.
pub struct Detail {
    heading: String,
    fields: Vec<Field>,
}

impl Detail {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            fields: Vec::new(),
        }
    }

    /// Add a field (builder style).
    pub fn field(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push(Field::new(label, value));
        self
    }

    /// Add a field (mutating).
    pub fn push_field(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.fields.push(Field::new(label, value));
    }
}

impl std::fmt::Display for Detail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.heading.heading())?;

        for field in &self.fields {
            writeln!(f, "  {} {}", field.label.label(), field.value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detail_renders_heading_and_fields() {
        let detail = Detail::new("governor.process")
            .field("persona:", "process")
            .field("description:", "Primary orchestration agent");

        let output = detail.to_string();
        assert!(output.contains("governor.process"));
        assert!(output.contains("persona:"));
        assert!(output.contains("process"));
        assert!(output.contains("description:"));
    }

    #[test]
    fn detail_with_no_fields_renders_heading_only() {
        let detail = Detail::new("empty");
        let output = detail.to_string();
        assert!(output.contains("empty"));
        // Just the heading line
        assert_eq!(output.lines().count(), 1);
    }
}
