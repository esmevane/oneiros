/// A trait for types that can be reported as structured tracing events.
///
/// Implementors provide the tracing level and human-readable message.
/// The `Outcomes` collection handles the actual tracing emission and
/// caller location capture.
pub trait Reportable {
    fn level(&self) -> tracing::Level;
    fn message(&self) -> String;
}
