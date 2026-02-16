use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sensation {
    pub name: SensationName,
    pub description: Description,
    pub prompt: Prompt,
}

domain_name!(SensationName);
