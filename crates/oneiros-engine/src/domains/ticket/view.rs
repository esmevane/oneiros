use crate::*;

pub struct TicketView {
    response: TicketResponse,
}

impl TicketView {
    pub fn new(response: TicketResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<TicketResponse> {
        match self.response {
            TicketResponse::Created(ticket) => {
                let prompt = Confirmation::new("Ticket", ticket.brain_name.to_string(), "issued")
                    .to_string();
                Rendered::new(TicketResponse::Created(ticket), prompt, String::new())
            }
            TicketResponse::Found(ticket) => {
                let prompt = Detail::new(ticket.brain_name.to_string())
                    .field("actor_id:", ticket.actor_id.to_string())
                    .to_string();
                Rendered::new(TicketResponse::Found(ticket), prompt, String::new())
            }
            TicketResponse::Validated(ticket) => {
                let prompt =
                    Confirmation::new("Ticket", ticket.brain_name.to_string(), "validated")
                        .to_string();
                Rendered::new(TicketResponse::Validated(ticket), prompt, String::new())
            }
            TicketResponse::Listed(listed) => {
                let mut table = Table::new(vec![
                    Column::key("brain_name", "Brain"),
                    Column::key("actor_id", "Actor"),
                ]);
                for ticket in &listed.items {
                    table.push_row(vec![
                        ticket.brain_name.to_string(),
                        ticket.actor_id.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(TicketResponse::Listed(listed), prompt, String::new())
            }
        }
    }
}
