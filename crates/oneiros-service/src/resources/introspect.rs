use oneiros_model::*;

use crate::*;

pub struct IntrospectStore;

impl Dispatch<IntrospectingRequests> for IntrospectStore {
    type Response = IntrospectingResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, IntrospectingRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            IntrospectingRequests::Introspect(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                context.scope.effects().mark(&begun)?;

                let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                context.scope.effects().mark(&complete)?;

                Ok(IntrospectingResponses::IntrospectionComplete(agent))
            }
        }
    }
}
