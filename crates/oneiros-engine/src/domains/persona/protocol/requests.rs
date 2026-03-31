use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SetPersona {
    #[builder(into)]
    pub name: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetPersona {
    #[builder(into)]
    pub name: PersonaName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemovePersona {
    #[builder(into)]
    pub name: PersonaName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PersonaRequestType, display = "kebab-case")]
pub enum PersonaRequest {
    SetPersona(SetPersona),
    GetPersona(GetPersona),
    ListPersonas,
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
