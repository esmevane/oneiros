use oneiros_model::*;

use crate::*;

pub struct ReflectStore;

impl Dispatch<ReflectingRequests> for ReflectStore {
    type Response = ReflectingResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, ReflectingRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            ReflectingRequests::Reflect(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun =
                    Events::Reflecting(ReflectingEvents::ReflectionBegun(SelectAgentByName {
                        name: agent.name.clone(),
                    }));
                context.scope.effects().mark(&begun)?;

                let complete =
                    Events::Reflecting(ReflectingEvents::ReflectionComplete(SelectAgentByName {
                        name: agent.name.clone(),
                    }));
                context.scope.effects().mark(&complete)?;

                Ok(ReflectingResponses::ReflectionComplete(agent))
            }
        }
    }
}
