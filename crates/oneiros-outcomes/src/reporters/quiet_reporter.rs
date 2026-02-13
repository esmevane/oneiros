use crate::*;

/// Prints message only, no prompts.
pub struct QuietReporter;

impl Reporter for QuietReporter {
    fn report(&self, outcome: &dyn Reportable) {
        println!("{}", outcome.message());
    }
}
