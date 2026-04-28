use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = SearchResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SearchResponse {
    Results(ResultsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ResultsResponse {
        V1 => {
            pub query: QueryText,
            pub results: Vec<Expression>,
        }
    }
}
