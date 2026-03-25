use oneiros_db::Projection;
use oneiros_model::{LevelRequests, LevelResponses};
use oneiros_resource::{Feature, Projections, Resource};

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

impl Feature<Projections> for Level {
    type Surface = &'static [Projection];

    fn feature(&self) -> Self::Surface {
        crate::projections::LEVEL
    }
}

impl Level {
    pub fn projections() -> &'static [Projection] {
        <Level as Feature<Projections>>::feature(&Level)
    }
}
