use oneiros_db::Database;
use oneiros_model::*;

use crate::*;

impl OneirosService {
    // ── Brain operations ──────────────────────────────────────────────

    pub fn dispatch_brain(&self, request: BrainRequests) -> Result<BrainResponses, Error> {
        let db = self.lock_system()?;
        match request {
            BrainRequests::CreateBrain(request) => {
                let tenant_id = self.source().tenant_id;
                let actor_id = self.source().actor_id;

                if db
                    .brain_exists(tenant_id.to_string(), request.name.as_str())
                    .is_ok()
                {
                    Err(Conflicts::Brain(request.name.clone()))?;
                }

                let brains_dir = self.state().data_dir().join("brains");
                oneiros_fs::FileOps.ensure_dir(&brains_dir)?;

                let path = brains_dir.join(format!("{}.db", request.name));

                Database::create_brain_db(&path)?;

                let brain = Brain::init(tenant_id, request.name, path);
                let brain_id = brain.id;

                let event = Events::Brain(BrainEvents::BrainCreated(brain));
                self.log_and_broadcast(&db, &event, projections::SYSTEM)?;

                let token = Token::issue(TokenClaims {
                    brain_id,
                    tenant_id,
                    actor_id,
                });

                let ticket = Ticket::init(token.clone(), actor_id);

                let ticket_event = Events::Ticket(TicketEvents::TicketIssued(ticket));
                self.log_and_broadcast(&db, &ticket_event, projections::SYSTEM)?;

                Ok(BrainResponses::BrainCreated(BrainInfo {
                    entity: brain_id,
                    token,
                }))
            }
            BrainRequests::GetBrain(request) => {
                let tenant_id = self.source().tenant_id;
                let brain = db
                    .get_brain_by_name(tenant_id.to_string(), &request.name)?
                    .ok_or(NotFound::BrainByName(request.name))?;
                Ok(BrainResponses::BrainFound(brain))
            }
            BrainRequests::ListBrains(_) => Ok(BrainResponses::BrainsListed(db.list_brains()?)),
        }
    }

    // ── Tenant operations ─────────────────────────────────────────────

    pub fn dispatch_tenant(&self, request: TenantRequests) -> Result<TenantResponses, Error> {
        let db = self.lock_system()?;
        match request {
            TenantRequests::GetTenant(request) => {
                let tenant = db
                    .get_tenant_by_name(&request.name)?
                    .ok_or(NotFound::Tenant(request.name))?;
                Ok(TenantResponses::TenantFound(tenant))
            }
            TenantRequests::ListTenants(_) => {
                Ok(TenantResponses::TenantsListed(db.list_tenants()?))
            }
        }
    }

    // ── Actor operations ──────────────────────────────────────────────

    pub fn dispatch_actor(&self, request: ActorRequests) -> Result<ActorResponses, Error> {
        let db = self.lock_system()?;
        match request {
            ActorRequests::GetActor(request) => {
                let actor = db
                    .get_actor_by_name(&request.name)?
                    .ok_or(NotFound::Actor(request.name))?;
                Ok(ActorResponses::ActorFound(actor))
            }
            ActorRequests::ListActors(_) => Ok(ActorResponses::ActorsListed(db.list_actors()?)),
        }
    }

    // ── Ticket operations ─────────────────────────────────────────────

    pub fn dispatch_ticket(&self, request: TicketRequests) -> Result<TicketResponses, Error> {
        let db = self.lock_system()?;
        match request {
            TicketRequests::ValidateTicket(request) => {
                let valid = db.validate_ticket(request.token.as_str())?;
                Ok(TicketResponses::TicketValid(valid))
            }
            TicketRequests::ListTickets(_) => {
                Ok(TicketResponses::TicketsListed(db.list_tickets()?))
            }
        }
    }
}
