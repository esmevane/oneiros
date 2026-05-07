use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Level {
    #[builder(into)]
    pub(crate) name: LevelName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

impl Indexable<LevelName> for Level {
    fn id(&self) -> LevelName {
        self.name.clone()
    }
}

pub(crate) type Levels = EntityIndex<LevelName, Level>;

resource_name!(LevelName);
