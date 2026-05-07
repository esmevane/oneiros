use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Persona {
    #[builder(into)]
    pub(crate) name: PersonaName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

impl Indexable<PersonaName> for Persona {
    fn id(&self) -> PersonaName {
        self.name.clone()
    }
}

pub(crate) type Personas = EntityIndex<PersonaName, Persona>;

resource_name!(PersonaName);
