use oneiros_model::Actor;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ActorEvents {
    ActorCreated(Actor),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ActorRequests {
    CreateActor(Actor),
}
