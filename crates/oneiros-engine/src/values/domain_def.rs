use crate::*;

/// Domain-level identity — the label and purpose that group a domain's
/// resources under one tag. Consumer concerns (router, skills) are
/// hand-written per domain, reading from `ResourceMeta` impls.
pub(crate) trait DomainDef {
    const LABEL: &'static str;
    const PURPOSE: &'static str;

    fn tag() -> Tag {
        Tag::builder()
            .name(Self::LABEL)
            .description(Self::PURPOSE)
            .build()
    }
}
