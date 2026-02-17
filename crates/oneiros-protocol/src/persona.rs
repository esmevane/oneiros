use oneiros_model::{Persona, PersonaName};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(Persona),
    PersonaRemoved { name: PersonaName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaRequests {
    SetPersona(Persona),
    RemovePersona { name: PersonaName },
}
