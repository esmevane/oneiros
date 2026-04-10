//! Confirmation view — a success message for mutations.
//!
//! Used for create, update, remove, and other state-changing operations.

use crate::*;

/// A confirmation that a mutation succeeded.
///
/// Renders as: ✓ Entity 'name' verbed.
pub struct Confirmation {
    entity: String,
    name: String,
    verb: String,
}

impl Confirmation {
    pub fn new(
        entity: impl Into<String>,
        name: impl Into<String>,
        verb: impl Into<String>,
    ) -> Self {
        Self {
            entity: entity.into(),
            name: name.into(),
            verb: verb.into(),
        }
    }
}

impl std::fmt::Display for Confirmation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} '{}' {}.",
            "✓".success(),
            self.entity,
            self.name,
            self.verb
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirmation_renders_message() {
        let msg = Confirmation::new("Agent", "governor.process", "created");
        let output = msg.to_string();
        assert!(output.contains("Agent"));
        assert!(output.contains("governor.process"));
        assert!(output.contains("created"));
        assert!(output.contains("✓"));
    }
}
