use oneiros_engine::*;
use oneiros_usage::*;

use super::vocabulary::{self, VocabularyDomain};

const DOMAIN: VocabularyDomain = VocabularyDomain {
    command: "persona",
    is_set: |r| matches!(r, Responses::Persona(PersonaResponse::PersonaSet(_))),
    is_details: |r| matches!(r, Responses::Persona(PersonaResponse::PersonaDetails(_))),
    extract_details: |r| match r {
        Responses::Persona(PersonaResponse::PersonaDetails(p)) => Some((
            p.name.as_str().to_owned(),
            p.description.as_str().to_owned(),
            p.prompt.as_str().to_owned(),
        )),
        _ => None,
    },
    is_list: |r| matches!(r, Responses::Persona(PersonaResponse::Personas(_))),
    extract_list_count: |r| match r {
        Responses::Persona(PersonaResponse::Personas(list)) => Some(list.len()),
        _ => None,
    },
    is_empty: |r| matches!(r, Responses::Persona(PersonaResponse::NoPersonas)),
    is_removed: |r| matches!(r, Responses::Persona(PersonaResponse::PersonaRemoved(_))),
};

pub(crate) async fn set_creates<B: Backend>() -> TestResult {
    vocabulary::set_creates_a_new_entry::<B>(&DOMAIN).await
}

pub(crate) async fn set_updates<B: Backend>() -> TestResult {
    vocabulary::set_updates_existing_entry::<B>(&DOMAIN).await
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    vocabulary::list_returns_empty_when_none_exist::<B>(&DOMAIN).await
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    vocabulary::list_returns_created_entries::<B>(&DOMAIN).await
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    vocabulary::remove_makes_it_unlisted::<B>(&DOMAIN).await
}
