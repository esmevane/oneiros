use crate::*;

/// Prints message and prompt to stdout. The default reporter.
pub struct PromptReporter;

impl Reporter for PromptReporter {
    fn report(&self, outcome: &dyn Reportable) {
        println!("{}", outcome.message());
        if let Some(prompt) = outcome.prompt() {
            println!("{}", prompt);
        }
    }
}
