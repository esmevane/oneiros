use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetPersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetPersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<PersonaName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemovePersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: PersonaName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListPersonas {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

resource_requests! {
    SetPersona => |this, client| {
        let SetPersona::V1(body) = this;
        client
            .put(&format!("/personas/{name}", name = body.name), this)
            .await
    },
    GetPersona => |this, client| {
        let GetPersona::V1(lookup) = this;
        client.get(&format!("/personas/{}", lookup.key)).await
    },
    RemovePersona => |this, client| {
        let RemovePersona::V1(removal) = this;
        client.delete(&format!("/personas/{}", removal.name)).await
    },
    ListPersonas => |this, client| {
        let ListPersonas::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/personas?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PersonaRequestType, display = "kebab-case")]
pub(crate) enum PersonaRequest {
    SetPersona(SetPersona),
    GetPersona(GetPersona),
    ListPersonas(ListPersonas),
    RemovePersona(RemovePersona),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (PersonaRequestType::SetPersona, "set-persona"),
            (PersonaRequestType::GetPersona, "get-persona"),
            (PersonaRequestType::ListPersonas, "list-personas"),
            (PersonaRequestType::RemovePersona, "remove-persona"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
