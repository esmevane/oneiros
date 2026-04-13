use crate::*;

pub(crate) struct TicketState;

impl TicketState {
    pub(crate) fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Ticket(TicketEvents::TicketIssued(ticket)) = event {
            canon.tickets.set(ticket);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
