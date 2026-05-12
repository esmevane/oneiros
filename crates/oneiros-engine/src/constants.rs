//! Shared string constants for URI schemes, prefixes, and protocol identifiers.
//!
//! Collects the prefix strings that were previously duplicated across
//! value types and domain code. Single source of truth — if a prefix
//! changes, it changes here.

/// Display/parse prefix for [`RefToken`] values: `ref:<base64url>`.
pub(crate) const REF_PREFIX: &str = "ref:";

/// Display/parse prefix for [`Link`] values: `link:<base64url>`.
pub(crate) const LINK_PREFIX: &str = "link:";

/// URI scheme for peer-to-peer transport via iroh: `oneiros://<host>/...`.
pub(crate) const ONEIROS_SCHEME: &str = "oneiros://";
