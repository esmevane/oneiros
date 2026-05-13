use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Project {
    #[builder(default = ProjectId::new())]
    pub(crate) id: ProjectId,
    #[builder(into)]
    pub(crate) name: ProjectName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Indexable<ProjectId> for Project {
    fn id(&self) -> ProjectId {
        self.id
    }
}

pub(crate) type Projects = EntityIndex<ProjectId, Project>;

resource_name!(ProjectName);
resource_id!(ProjectId);
