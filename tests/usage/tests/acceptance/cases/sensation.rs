use oneiros_engine::*;
use oneiros_usage::*;

use super::vocabulary::{self, VocabularyDomain};

const DOMAIN: VocabularyDomain = VocabularyDomain {
    command: "sensation",
    is_set: |r| matches!(r, Responses::Sensation(SensationResponse::SensationSet(_))),
    is_details: |r| {
        matches!(
            r,
            Responses::Sensation(SensationResponse::SensationDetails(_))
        )
    },
    extract_details: |r| match r {
        Responses::Sensation(SensationResponse::SensationDetails(s)) => Some((
            s.name.as_str().to_owned(),
            s.description.as_str().to_owned(),
            s.prompt.as_str().to_owned(),
        )),
        _ => None,
    },
    is_list: |r| matches!(r, Responses::Sensation(SensationResponse::Sensations(_))),
    extract_list_count: |r| match r {
        Responses::Sensation(SensationResponse::Sensations(list)) => Some(list.len()),
        _ => None,
    },
    is_empty: |r| matches!(r, Responses::Sensation(SensationResponse::NoSensations)),
    is_removed: |r| {
        matches!(
            r,
            Responses::Sensation(SensationResponse::SensationRemoved(_))
        )
    },
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

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    vocabulary::set_prompt_confirms_creation::<B>(&DOMAIN).await
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    vocabulary::show_prompt_contains_entry::<B>(&DOMAIN).await
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    vocabulary::list_prompt_contains_entries::<B>(&DOMAIN).await
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    vocabulary::remove_prompt_confirms_removal::<B>(&DOMAIN).await
}
