mod outcomes;
mod reportable;
mod reporter;

#[cfg(test)]
mod tests;

pub use oneiros_outcomes_derive::Outcome;
pub use outcomes::Outcomes;
pub use reportable::Reportable;
pub use reporter::{ConsoleReporter, Reporter};
