use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Nature {
    #[builder(into)]
    pub(crate) name: NatureName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

impl Indexable<NatureName> for Nature {
    fn id(&self) -> NatureName {
        self.name.clone()
    }
}

pub(crate) type Natures = EntityIndex<NatureName, Nature>;

resource_name!(NatureName);
