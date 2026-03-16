use oneiros_model::*;

use crate::*;

pub struct ExperienceStore;

impl Dispatch<ExperienceRequests> for ExperienceStore {
    type Response = ExperienceResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, ExperienceRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            ExperienceRequests::CreateExperience(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                db.get_sensation(&request.sensation)?
                    .ok_or(NotFound::Sensation(request.sensation.clone()))?;

                let experience =
                    Experience::create(agent.id, request.sensation, request.description);

                let event =
                    Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));
                context.scope.effects().emit(&event)?;

                Ok(ExperienceResponses::ExperienceCreated(experience))
            }
            ExperienceRequests::GetExperience(request) => {
                let experience = db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceFound(experience))
            }
            ExperienceRequests::ListExperiences(request) => {
                let experiences = match (request.agent, request.sensation) {
                    (Some(agent_name), Some(sensation)) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.get_sensation(&sensation)?
                            .ok_or(NotFound::Sensation(sensation.clone()))?;

                        db.list_experiences_by_agent(agent.id.to_string())?
                            .into_iter()
                            .filter(|exp| exp.sensation == sensation)
                            .collect()
                    }
                    (Some(agent_name), None) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.list_experiences_by_agent(agent.id.to_string())?
                    }
                    (None, Some(sensation)) => {
                        db.get_sensation(&sensation)?
                            .ok_or(NotFound::Sensation(sensation.clone()))?;

                        db.list_experiences_by_sensation(&sensation)?
                    }
                    (None, None) => db.list_experiences()?,
                };

                Ok(ExperienceResponses::ExperiencesListed(experiences))
            }
            ExperienceRequests::UpdateExperienceDescription(request) => {
                db.get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;

                let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
                    ExperienceDescriptionUpdate {
                        experience_id: request.id,
                        description: request.description,
                    },
                ));

                context.scope.effects().emit(&event)?;

                let experience = db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceUpdated(experience))
            }
            ExperienceRequests::UpdateExperienceSensation(request) => {
                db.get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;

                let event = Events::Experience(ExperienceEvents::ExperienceSensationUpdated(
                    ExperienceSensationUpdate {
                        experience_id: request.id,
                        sensation: request.sensation,
                    },
                ));

                context.scope.effects().emit(&event)?;

                let experience = db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceUpdated(experience))
            }
        }
    }
}
