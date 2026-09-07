use noteworthy::Notation;

/// Pure resource metadata — the description of a resource, independent of any
/// consumer (aide routing, skills, docs). Attached via `#[annotation]`.
#[derive(Notation)]
pub(crate) struct ResourceMeta {
    pub path: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub content: &'static str,
    #[allow(dead_code)]
    pub status: u16,
}
