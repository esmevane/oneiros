use oneiros_model::Ticket;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TicketEvents {
    TicketIssued(Ticket),
}
