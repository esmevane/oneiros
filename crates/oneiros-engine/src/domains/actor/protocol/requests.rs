use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateActor {
    #[arg(long)]
    #[builder(into)]
    pub tenant_id: TenantId,
    #[builder(into)]
    pub name: ActorName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetActor {
    #[builder(into)]
    pub id: ActorId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListActors {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ActorRequestType, display = "kebab-case")]
pub enum ActorRequest {
    CreateActor(CreateActor),
    GetActor(GetActor),
    ListActors(ListActors),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ActorRequestType::CreateActor, "create-actor"),
            (ActorRequestType::GetActor, "get-actor"),
            (ActorRequestType::ListActors, "list-actors"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
