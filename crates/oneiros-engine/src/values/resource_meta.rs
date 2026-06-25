use schemars::JsonSchema;

/// Pure resource metadata — the description of a resource, independent of any
/// consumer (aide routing, skills, docs). Carries value-level data (consts)
/// and type-valued metadata (Response) but calls no consumer APIs.
pub(crate) trait ResourceMeta {
    type Response: JsonSchema;

    const PATH: &'static str;
    const SUMMARY: &'static str;
    const DESCRIPTION: &'static str;
    const STATUS: u16 = 200;

    fn content() -> &'static str;
}
