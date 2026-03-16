use oneiros_model::*;

use crate::*;

pub struct UrgeStore;

impl Dispatch<UrgeRequests> for UrgeStore {
    type Response = UrgeResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, UrgeRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            UrgeRequests::SetUrge(urge) => {
                let event = Events::Urge(UrgeEvents::UrgeSet(urge.clone()));
                context.scope.effects().emit(&event)?;
                Ok(UrgeResponses::UrgeSet(urge))
            }
            UrgeRequests::ListUrges(_) => Ok(UrgeResponses::UrgesListed(db.list_urges()?)),
            UrgeRequests::GetUrge(request) => {
                let urge = db
                    .get_urge(&request.name)?
                    .ok_or(NotFound::Urge(request.name))?;
                Ok(UrgeResponses::UrgeFound(urge))
            }
            UrgeRequests::RemoveUrge(request) => {
                let event = Events::Urge(UrgeEvents::UrgeRemoved(SelectUrgeByName {
                    name: request.name,
                }));
                context.scope.effects().emit(&event)?;
                Ok(UrgeResponses::UrgeRemoved)
            }
        }
    }
}
