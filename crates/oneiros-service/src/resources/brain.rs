use std::path::PathBuf;

use oneiros_db::Database;
use oneiros_model::*;

use crate::*;

pub struct BrainStore {
    pub data_dir: PathBuf,
}

impl Dispatch<BrainRequests> for BrainStore {
    type Response = BrainResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, BrainRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();
        let source = context.scope.source();

        match context.request {
            BrainRequests::CreateBrain(request) => {
                let tenant_id = source.tenant_id;
                let actor_id = source.actor_id;

                if db
                    .brain_exists(tenant_id.to_string(), request.name.as_str())
                    .is_ok()
                {
                    Err(Conflicts::Brain(request.name.clone()))?;
                }

                let brains_dir = self.data_dir.join("brains");
                oneiros_fs::FileOps.ensure_dir(&brains_dir)?;

                let path = brains_dir.join(format!("{}.db", request.name));

                Database::create_brain_db(&path)?;

                let brain = Brain::init(tenant_id, request.name, path);
                let brain_id = brain.id;

                let event = Events::Brain(BrainEvents::BrainCreated(brain));
                context.scope.effects().emit(&event)?;

                let token = Token::issue(TokenClaims {
                    brain_id,
                    tenant_id,
                    actor_id,
                });

                let ticket = Ticket::init(token.clone(), actor_id);

                let ticket_event = Events::Ticket(TicketEvents::TicketIssued(ticket));
                context.scope.effects().emit(&ticket_event)?;

                Ok(BrainResponses::BrainCreated(BrainInfo {
                    entity: brain_id,
                    token,
                }))
            }
            BrainRequests::GetBrain(request) => {
                let tenant_id = source.tenant_id;
                let brain = db
                    .get_brain_by_name(tenant_id.to_string(), &request.name)?
                    .ok_or(NotFound::BrainByName(request.name))?;
                Ok(BrainResponses::BrainFound(brain))
            }
            BrainRequests::ListBrains(_) => Ok(BrainResponses::BrainsListed(db.list_brains()?)),
        }
    }
}
