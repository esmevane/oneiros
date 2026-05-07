use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Urge {
    #[builder(into)]
    pub(crate) name: UrgeName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

impl Indexable<UrgeName> for Urge {
    fn id(&self) -> UrgeName {
        self.name.clone()
    }
}

pub(crate) type Urges = EntityIndex<UrgeName, Urge>;

resource_name!(UrgeName);
