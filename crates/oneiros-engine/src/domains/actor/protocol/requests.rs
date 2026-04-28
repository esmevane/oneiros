use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateActor {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)] pub tenant_id: TenantId,
            #[builder(into)] pub name: ActorName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetActor {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<ActorId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListActors {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
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
