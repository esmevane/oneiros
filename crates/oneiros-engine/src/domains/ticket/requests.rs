use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TicketRequest {
    Create {
        actor_id: String,
        brain_name: String,
    },
    Get {
        id: String,
    },
    List,
    Validate {
        token: String,
    },
}
