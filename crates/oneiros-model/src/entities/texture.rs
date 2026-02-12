use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Texture {
    pub name: TextureName,
    pub description: Description,
    pub prompt: Prompt,
}

domain_name!(TextureName);
