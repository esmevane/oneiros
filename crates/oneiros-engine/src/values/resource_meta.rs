use kinded::Kind;

/// Per-request-struct metadata — carried by each variant of a resource enum.
///
/// Each request struct (e.g. `CreateActor`) implements this trait, providing
/// the static routing and documentation data for its variant.
pub(crate) trait ResourceRequestMeta {
    /// The Kinded variant label (e.g. `ActorRequestType`).
    type Kind: Kind;

    const PATH: &'static str;
    const SUMMARY: &'static str;
    const DESCRIPTION: &'static str;

    /// Content for the skill document — replaces `include_str!("features/skills/...")`.
    fn content() -> &'static str;
}

/// Dispatch bridge — implemented by the owning enum (e.g. `ActorRequest`).
///
/// Maps a `Kind` to the `ResourceRequestMeta` impl of the corresponding variant.
pub(crate) trait ResourceDispatch {
    type Kind: Kind;

    fn path_for(kind: Self::Kind) -> &'static str;
    fn summary_for(kind: Self::Kind) -> &'static str;
    fn description_for(kind: Self::Kind) -> &'static str;
    fn content_for(kind: Self::Kind) -> &'static str;
}
