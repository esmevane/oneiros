use crate::*;

pub struct ProjectService;

impl ProjectService {
    pub fn init(
        ctx: &SystemContext,
        brain_name: String,
    ) -> Result<ProjectResponse, Box<dyn std::error::Error>> {
        if let Ok(BrainResponse::Found(_)) = BrainService::get(ctx, &brain_name) {
            return Ok(ProjectResponse::BrainAlreadyExists(BrainName::new(
                &brain_name,
            )));
        }

        BrainService::create(ctx, brain_name.clone())?;

        let actors = ctx
            .with_db(|conn| ActorRepo::new(conn).list())
            .map_err(|e| format!("database error: {e}"))?;

        if let Some(actor) = actors.first() {
            TicketService::create(ctx, actor.id.clone(), brain_name.clone())?;
        }

        Ok(ProjectResponse::BrainCreated(BrainName::new(&brain_name)))
    }
}
