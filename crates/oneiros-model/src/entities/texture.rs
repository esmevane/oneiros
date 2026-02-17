use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Texture {
    pub name: TextureName,
    pub description: Description,
    pub prompt: Prompt,
}

impl<A, B, C> From<(A, B, C)> for Texture
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
{
    fn from((name, description, prompt): (A, B, C)) -> Self {
        Texture {
            name: TextureName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

impl Texture {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

domain_name!(TextureName);
