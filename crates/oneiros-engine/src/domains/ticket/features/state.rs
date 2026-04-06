use crate::*;

pub struct TicketState;

impl TicketState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Ticket(TicketEvents::TicketIssued(ticket)) = event {
            canon.tickets.insert(ticket.id.to_string(), ticket.clone());
        }

        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
