use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SetPersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub prompt: Prompt,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetPersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<PersonaName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemovePersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: PersonaName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListPersonas {
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
#[kinded(kind = PersonaRequestType, display = "kebab-case")]
pub enum PersonaRequest {
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
