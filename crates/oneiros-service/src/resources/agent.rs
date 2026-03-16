use oneiros_model::*;

use crate::*;

pub struct AgentStore;

impl Dispatch<AgentRequests> for AgentStore {
    type Response = AgentResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, AgentRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            AgentRequests::CreateAgent(request) => {
                db.get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                if db.agent_name_exists(&request.name)? {
                    return Err(Conflicts::Agent(request.name).into());
                }

                let agent = Agent::init(
                    request.description,
                    request.prompt,
                    request.name,
                    request.persona,
                );

                let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
                context.scope.effects().emit(&event)?;

                Ok(AgentResponses::AgentCreated(agent))
            }
            AgentRequests::ListAgents(_) => Ok(AgentResponses::AgentsListed(db.list_agents()?)),
            AgentRequests::GetAgent(request) => {
                let agent = db
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name))?;
                Ok(AgentResponses::AgentFound(agent))
            }
            AgentRequests::UpdateAgent(request) => {
                let existing = db
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name.clone()))?;

                db.get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                let agent = Agent::construct(
                    existing.id,
                    request.description,
                    request.prompt,
                    existing.name.clone(),
                    request.persona,
                );

                let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));
                context.scope.effects().emit(&event)?;

                Ok(AgentResponses::AgentUpdated(agent))
            }
            AgentRequests::RemoveAgent(request) => {
                let event = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.name,
                }));
                context.scope.effects().emit(&event)?;

                Ok(AgentResponses::AgentRemoved)
            }
        }
    }
}
