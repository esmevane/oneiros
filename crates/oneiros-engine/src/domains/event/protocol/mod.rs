mod errors;
mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "kebab-case")]
    pub enum EventResponse {}
}

pub use errors::EventError;
pub use responses::EventResponse;
