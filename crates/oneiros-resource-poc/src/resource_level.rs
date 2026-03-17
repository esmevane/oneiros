use oneiros_db::Projection;
use oneiros_model::{LevelRequests, LevelResponses};
use oneiros_resource::Resource;

/// The Level resource declaration.
///
/// Level is project-scoped vocabulary — pure set/get/list/remove,
/// no FK validation, no conflict detection. The simplest resource shape.
pub struct Level;

impl Resource for Level {
    const NAME: &'static str = "level";

    type Request = LevelRequests;
    type Response = LevelResponses;
}

impl Level {
    /// Projections this resource needs to maintain its read model.
    pub fn projections() -> &'static [Projection] {
        crate::projections::LEVEL
    }
}
