use oneiros_usage::*;

use super::vocabulary::{self, VocabularyDomain};

const DOMAIN: VocabularyDomain = VocabularyDomain {
    command: "urge",
    set_type: "urge-set",
    details_type: "urge-details",
    list_type: "urges",
    empty_type: "no-urges",
    removed_type: "urge-removed",
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
