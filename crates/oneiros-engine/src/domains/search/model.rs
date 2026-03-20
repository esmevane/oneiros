use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SearchResult {
    pub resource_ref: Ref,
    #[builder(into)]
    pub kind: Label,
    #[builder(into)]
    pub content: Content,
}
