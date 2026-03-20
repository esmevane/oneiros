mod errors;
mod responses {
    use serde::{Deserialize, Serialize};

    use crate::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "kebab-case")]
    pub enum EventResponse {
        Events(Vec<StoredEvent>),
    }
}

pub use errors::EventError;
pub use responses::EventResponse;
