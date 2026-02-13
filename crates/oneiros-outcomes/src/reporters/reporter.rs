use crate::*;

/// A consumer of outcomes that presents them to the user.
///
/// Separating reporting from the outcome types allows different
/// presentation strategies (console, TUI, structured output, etc.)
/// without changing the outcome definitions.
pub trait Reporter {
    fn report(&self, outcome: &dyn Reportable);
}
