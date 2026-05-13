use crate::*;

pub(crate) struct TicketState;

impl TicketState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Ticket(ticket_event) = event
            && let Some(ticket) = ticket_event.maybe_ticket()
        {
            canon.tickets.set(&ticket);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}
