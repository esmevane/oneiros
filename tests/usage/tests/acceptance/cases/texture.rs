use oneiros_engine::*;
use oneiros_usage::*;

use super::vocabulary::{self, VocabularyDomain};

const DOMAIN: VocabularyDomain = VocabularyDomain {
    command: "texture",
    is_set: |r| matches!(r, Responses::Texture(TextureResponse::TextureSet(_))),
    is_details: |r| matches!(r, Responses::Texture(TextureResponse::TextureDetails(_))),
    extract_details: |r| match r {
        Responses::Texture(TextureResponse::TextureDetails(t)) => Some((
            t.name.as_str().to_owned(),
            t.description.as_str().to_owned(),
            t.prompt.as_str().to_owned(),
        )),
        _ => None,
    },
    is_list: |r| matches!(r, Responses::Texture(TextureResponse::Textures(_))),
    extract_list_count: |r| match r {
        Responses::Texture(TextureResponse::Textures(list)) => Some(list.len()),
        _ => None,
    },
    is_empty: |r| matches!(r, Responses::Texture(TextureResponse::NoTextures)),
    is_removed: |r| matches!(r, Responses::Texture(TextureResponse::TextureRemoved(_))),
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
