use oneiros_model::*;

use crate::*;

pub struct SenseStore;

impl Dispatch<SenseRequests> for SenseStore {
    type Response = SenseResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, SenseRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            SenseRequests::Sense(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let event = Events::Sense(SenseEvents::Sensed(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                context.scope.effects().mark(&event)?;

                Ok(SenseResponses::Sensed(agent))
            }
        }
    }
}
