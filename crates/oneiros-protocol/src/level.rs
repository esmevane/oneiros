use oneiros_model::{Level, LevelName};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelEvents {
    LevelSet(Level),
    LevelRemoved { name: LevelName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelRequests {
    SetLevel(Level),
    RemoveLevel { name: LevelName },
}
