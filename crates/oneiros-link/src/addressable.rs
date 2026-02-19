use crate::{Link, LinkError};

/// A type whose identity can be expressed as a content-addressable [`Link`].
///
/// Implementations choose which fields constitute identity (included in the
/// link) versus mutable content (excluded). The `address_label` discriminates
/// resource types so that identical field values in different domains produce
/// distinct links.
///
/// # Example
///
/// ```
/// use oneiros_link::{Addressable, Link, LinkError};
///
/// struct Agent {
///     name: String,
///     persona: String,
///     description: String, // mutable, not part of identity
/// }
///
/// impl Addressable for Agent {
///     fn address_label() -> &'static str { "agent" }
///
///     fn link(&self) -> Result<Link, LinkError> {
///         // Only name and persona are identity
///         Link::new(&(Self::address_label(), &self.name, &self.persona))
///     }
/// }
/// ```
pub trait Addressable {
    /// The resource type label included in the link for type discrimination.
    ///
    /// Two types with the same identity fields but different labels produce
    /// different links.
    fn address_label() -> &'static str;

    /// Compute the content-addressable link for this value.
    fn link(&self) -> Result<Link, LinkError>;
}
