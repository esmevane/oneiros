use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SetPersona {
    #[builder(into)]
    pub(crate) name: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetPersona {
    #[builder(into)]
    pub(crate) name: PersonaName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RemovePersona {
    #[builder(into)]
    pub(crate) name: PersonaName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListPersonas {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
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
