use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Sensation {
    #[builder(into)]
    pub(crate) name: SensationName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

impl Indexable<SensationName> for Sensation {
    fn id(&self) -> SensationName {
        self.name.clone()
    }
}

pub(crate) type Sensations = EntityIndex<SensationName, Sensation>;

resource_name!(SensationName);
