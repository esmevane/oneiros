use oneiros_model::*;

use crate::*;

pub struct CognitionStore;

impl Dispatch<CognitionRequests> for CognitionStore {
    type Response = CognitionResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, CognitionRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            CognitionRequests::AddCognition(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                db.get_texture(&request.texture)?
                    .ok_or(NotFound::Texture(request.texture.clone()))?;

                let cognition = Cognition::create(agent.id, request.texture, request.content);

                let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));
                context.scope.effects().emit(&event)?;

                Ok(CognitionResponses::CognitionAdded(cognition))
            }
            CognitionRequests::GetCognition(request) => {
                let cognition = db
                    .get_cognition(request.id.to_string())?
                    .ok_or(NotFound::Cognition(request.id))?;
                Ok(CognitionResponses::CognitionFound(cognition))
            }
            CognitionRequests::ListCognitions(request) => {
                let cognitions = match (request.agent, request.texture) {
                    (Some(agent_name), Some(texture)) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.get_texture(&texture)?
                            .ok_or(NotFound::Texture(texture.clone()))?;

                        db.list_cognitions_by_agent_and_texture(agent.id.to_string(), &texture)?
                    }
                    (Some(agent_name), None) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.list_cognitions_by_agent(agent.id.to_string())?
                    }
                    (None, Some(texture)) => {
                        db.get_texture(&texture)?
                            .ok_or(NotFound::Texture(texture.clone()))?;

                        db.list_cognitions_by_texture(&texture)?
                    }
                    (None, None) => db.list_cognitions()?,
                };

                Ok(CognitionResponses::CognitionsListed(cognitions))
            }
        }
    }
}
