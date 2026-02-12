use crate::Reportable;

/// A consumer of outcomes that presents them to the user.
///
/// Separating reporting from the outcome types allows different
/// presentation strategies (console, TUI, structured output, etc.)
/// without changing the outcome definitions.
pub trait Reporter {
    fn report(&self, outcome: &dyn Reportable);
}

/// Prints outcomes to stdout: message first, then prompt if present.
pub struct ConsoleReporter;

impl Reporter for ConsoleReporter {
    fn report(&self, outcome: &dyn Reportable) {
        println!("{}", outcome.message());
        if let Some(prompt) = outcome.prompt() {
            println!("{}", prompt);
        }
    }
}
