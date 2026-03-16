use oneiros_model::*;

use crate::*;

pub struct SensationStore;

impl Dispatch<SensationRequests> for SensationStore {
    type Response = SensationResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, SensationRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            SensationRequests::SetSensation(sensation) => {
                let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));
                context.scope.effects().emit(&event)?;
                Ok(SensationResponses::SensationSet(sensation))
            }
            SensationRequests::ListSensations(_) => {
                Ok(SensationResponses::SensationsListed(db.list_sensations()?))
            }
            SensationRequests::GetSensation(request) => {
                let sensation = db
                    .get_sensation(&request.name)?
                    .ok_or(NotFound::Sensation(request.name))?;
                Ok(SensationResponses::SensationFound(sensation))
            }
            SensationRequests::RemoveSensation(request) => {
                let event =
                    Events::Sensation(SensationEvents::SensationRemoved(SelectSensationByName {
                        name: request.name,
                    }));
                context.scope.effects().emit(&event)?;
                Ok(SensationResponses::SensationRemoved)
            }
        }
    }
}
