use serde::{Deserialize, Serialize};

use super::model::SearchResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SearchResponse {
    Results(Vec<SearchResult>),
}
