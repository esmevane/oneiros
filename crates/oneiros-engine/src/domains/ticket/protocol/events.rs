use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TicketEventsType, display = "kebab-case")]
pub enum TicketEvents {
    TicketIssued(Ticket),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&TicketEventsType::TicketIssued.to_string(), "ticket-issued");
    }
}
