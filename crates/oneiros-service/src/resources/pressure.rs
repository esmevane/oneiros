use oneiros_model::*;

use crate::*;

pub struct PressureStore;

impl Dispatch<PressureRequests> for PressureStore {
    type Response = PressureResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, PressureRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            PressureRequests::GetPressure(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let pressures = db.list_pressures_for_agent(&agent.id.to_string())?;

                Ok(PressureResponses::PressureFound(pressures))
            }
            PressureRequests::ListPressures(_) => {
                let agents = db.list_agents()?;
                let mut pressures = Vec::new();

                for agent in agents {
                    let mut agent_pressures = db.list_pressures_for_agent(&agent.id.to_string())?;
                    pressures.append(&mut agent_pressures);
                }

                Ok(PressureResponses::PressuresListed(pressures))
            }
        }
    }
}
