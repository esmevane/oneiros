use oneiros_model::*;

use crate::*;

pub struct ActorStore;

impl Dispatch<ActorRequests> for ActorStore {
    type Response = ActorResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, ActorRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            ActorRequests::GetActor(request) => {
                let actor = db
                    .get_actor_by_name(&request.name)?
                    .ok_or(NotFound::Actor(request.name))?;
                Ok(ActorResponses::ActorFound(actor))
            }
            ActorRequests::ListActors(_) => Ok(ActorResponses::ActorsListed(db.list_actors()?)),
        }
    }
}
