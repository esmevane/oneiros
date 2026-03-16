use oneiros_model::*;

use crate::*;

/// Level domain store. Knows how to dispatch level requests
/// against a scope's database and effects.
pub struct LevelStore;

impl Dispatch<LevelRequests> for LevelStore {
    type Response = LevelResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, LevelRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            LevelRequests::SetLevel(level) => {
                let event = Events::Level(LevelEvents::LevelSet(level.clone()));
                context.scope.effects().emit(&event)?;
                Ok(LevelResponses::LevelSet(level))
            }
            LevelRequests::ListLevels(_) => Ok(LevelResponses::LevelsListed(db.list_levels()?)),
            LevelRequests::GetLevel(request) => {
                let level = db
                    .get_level(&request.name)?
                    .ok_or(NotFound::Level(request.name))?;
                Ok(LevelResponses::LevelFound(level))
            }
            LevelRequests::RemoveLevel(request) => {
                let event = Events::Level(LevelEvents::LevelRemoved(SelectLevelByName {
                    name: request.name,
                }));
                context.scope.effects().emit(&event)?;
                Ok(LevelResponses::LevelRemoved)
            }
        }
    }
}
