/// A trait for types that can be reported as structured tracing events.
///
/// Implementors provide the tracing level and human-readable message.
/// The `Outcomes` collection handles the actual tracing emission and
/// caller location capture.
///
/// - `message()` is the user-facing display text.
/// - `log_message()` is the tracing output; defaults to `message()`.
/// - `prompt()` is optional actionable guidance text.
pub trait Reportable {
    fn level(&self) -> tracing::Level;
    fn message(&self) -> String;

    fn log_message(&self) -> String {
        self.message()
    }

    fn prompt(&self) -> Option<String> {
        None
    }
}
