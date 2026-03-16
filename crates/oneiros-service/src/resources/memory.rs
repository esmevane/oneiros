use oneiros_model::*;

use crate::*;

pub struct MemoryStore;

impl Dispatch<MemoryRequests> for MemoryStore {
    type Response = MemoryResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, MemoryRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            MemoryRequests::AddMemory(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                db.get_level(&request.level)?
                    .ok_or(NotFound::Level(request.level.clone()))?;

                let memory = Memory::create(agent.id, request.level, request.content);

                let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));
                context.scope.effects().emit(&event)?;

                Ok(MemoryResponses::MemoryAdded(memory))
            }
            MemoryRequests::GetMemory(request) => {
                let memory = db
                    .get_memory(request.id.to_string())?
                    .ok_or(NotFound::Memory(request.id))?;
                Ok(MemoryResponses::MemoryFound(memory))
            }
            MemoryRequests::ListMemories(request) => {
                let memories = match (request.agent, request.level) {
                    (Some(agent_name), Some(level)) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.get_level(&level)?
                            .ok_or(NotFound::Level(level.clone()))?;

                        db.list_memories_by_agent_and_level(agent.id.to_string(), &level)?
                    }
                    (Some(agent_name), None) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.list_memories_by_agent(agent.id.to_string())?
                    }
                    (None, Some(level)) => {
                        db.get_level(&level)?
                            .ok_or(NotFound::Level(level.clone()))?;

                        db.list_memories_by_level(&level)?
                    }
                    (None, None) => db.list_memories()?,
                };

                Ok(MemoryResponses::MemoriesListed(memories))
            }
        }
    }
}
