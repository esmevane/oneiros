use oneiros_model::*;

use crate::*;

pub struct NatureStore;

impl Dispatch<NatureRequests> for NatureStore {
    type Response = NatureResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, NatureRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            NatureRequests::SetNature(nature) => {
                let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));
                context.scope.effects().emit(&event)?;
                Ok(NatureResponses::NatureSet(nature))
            }
            NatureRequests::ListNatures(_) => {
                Ok(NatureResponses::NaturesListed(db.list_natures()?))
            }
            NatureRequests::GetNature(request) => {
                let nature = db
                    .get_nature(&request.name)?
                    .ok_or(NotFound::Nature(request.name))?;
                Ok(NatureResponses::NatureFound(nature))
            }
            NatureRequests::RemoveNature(request) => {
                let event = Events::Nature(NatureEvents::NatureRemoved(SelectNatureByName {
                    name: request.name,
                }));
                context.scope.effects().emit(&event)?;
                Ok(NatureResponses::NatureRemoved)
            }
        }
    }
}
