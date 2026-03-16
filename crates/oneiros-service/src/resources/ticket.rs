use oneiros_model::*;

use crate::*;

pub struct TicketStore;

impl Dispatch<TicketRequests> for TicketStore {
    type Response = TicketResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, TicketRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
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
