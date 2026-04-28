use crate::*;

pub struct TicketState;

impl TicketState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Ticket(ticket_event) = event
            && let Some(ticket) = ticket_event.maybe_ticket()
        {
            canon.tickets.set(&ticket);
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
