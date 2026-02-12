use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Level {
    pub name: LevelName,
    pub description: Description,
    pub prompt: Prompt,
}

domain_name!(LevelName);
