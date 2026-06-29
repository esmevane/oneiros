use crate::*;

/// Pure resource metadata — the description of a resource, independent of any
/// consumer (aide routing, skills, docs). All value-level data, returned by
/// const fn. No aide, no associated types.
pub(crate) struct ResourceMeta {
    pub path: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub content: &'static str,
    pub status: u16,
}
