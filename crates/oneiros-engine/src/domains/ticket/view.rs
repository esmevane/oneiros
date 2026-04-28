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
            TicketResponse::Created(TicketCreatedResponse::V1(created)) => {
                let prompt =
                    Confirmation::new("Ticket", created.ticket.brain_name.to_string(), "issued")
                        .to_string();
                Rendered::new(
                    TicketResponse::Created(TicketCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            TicketResponse::Found(TicketFoundResponse::V1(found)) => {
                let prompt = Detail::new(found.ticket.brain_name.to_string())
                    .field("actor_id:", found.ticket.actor_id.to_string())
                    .to_string();
                Rendered::new(
                    TicketResponse::Found(TicketFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            TicketResponse::Validated(TicketValidatedResponse::V1(validated)) => {
                let prompt = Confirmation::new(
                    "Ticket",
                    validated.ticket.brain_name.to_string(),
                    "validated",
                )
                .to_string();
                Rendered::new(
                    TicketResponse::Validated(TicketValidatedResponse::V1(validated)),
                    prompt,
                    String::new(),
                )
            }
            TicketResponse::Listed(TicketsResponse::V1(listed)) => {
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
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    TicketResponse::Listed(TicketsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
