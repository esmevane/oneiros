use oneiros_model::*;

use crate::*;

pub struct LifecycleStore;

impl Dispatch<LifecycleRequests> for LifecycleStore {
    type Response = LifecycleResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, LifecycleRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();
        let effects = context.scope.effects();

        match context.request {
            LifecycleRequests::Emerge(request) => {
                db.get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                if db.agent_name_exists(&request.name)? {
                    return Err(Conflicts::Agent(request.name).into());
                }

                let agent_name = request.name.clone();

                let agent = Agent::init(
                    request.description,
                    request.prompt,
                    request.name,
                    request.persona,
                );

                let emerged = Events::Lifecycle(LifecycleEvents::Emerged(SelectAgentByName {
                    name: agent_name,
                }));
                effects.mark(&emerged)?;

                let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
                effects.emit(&created)?;

                Ok(LifecycleResponses::Emerged(agent))
            }
            LifecycleRequests::Wake(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent))?;

                let woke = Events::Lifecycle(LifecycleEvents::Woke(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                effects.mark(&woke)?;

                let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                effects.mark(&begun)?;

                let dream_context =
                    DreamCollector::new(db, DreamConfig::default()).collect(&agent)?;

                let complete =
                    Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
                        agent: dream_context.agent.clone(),
                    }));
                effects.mark(&complete)?;

                Ok(LifecycleResponses::Woke(Box::new(dream_context)))
            }
            LifecycleRequests::Sleep(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent))?;

                let slept = Events::Lifecycle(LifecycleEvents::Slept(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                effects.mark(&slept)?;

                let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                effects.mark(&begun)?;

                let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                effects.mark(&complete)?;

                Ok(LifecycleResponses::Slept(agent))
            }
            LifecycleRequests::Recede(request) => {
                db.get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let receded = Events::Lifecycle(LifecycleEvents::Receded(SelectAgentByName {
                    name: request.agent.clone(),
                }));
                effects.mark(&receded)?;

                let removed = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.agent,
                }));
                effects.emit(&removed)?;

                Ok(LifecycleResponses::Receded)
            }
        }
    }
}
