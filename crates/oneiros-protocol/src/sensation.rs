use oneiros_model::{Sensation, SensationName};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(Sensation),
    SensationRemoved { name: SensationName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationRequests {
    SetSensation(Sensation),
    RemoveSensation { name: SensationName },
}
