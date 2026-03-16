use oneiros_model::*;

use crate::*;

pub struct DreamStore;

impl Dispatch<DreamingRequests> for DreamStore {
    type Response = DreamingResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, DreamingRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            DreamingRequests::Dream(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                context.scope.effects().mark(&begun)?;

                let dream_context = DreamCollector::new(db, request.config).collect(&agent)?;

                let complete =
                    Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
                        agent: dream_context.agent.clone(),
                    }));
                context.scope.effects().mark(&complete)?;

                Ok(DreamingResponses::DreamComplete(Box::new(dream_context)))
            }
        }
    }
}
