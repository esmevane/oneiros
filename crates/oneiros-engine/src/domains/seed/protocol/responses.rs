use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SeedResponse {
    SeedComplete,
}
