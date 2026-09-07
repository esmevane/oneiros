use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
    http::{StatusCode, header},
    response::IntoResponse,
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Upload a file",
        description: "Store a file as a blob in the project's archive.",
        content: include_str!("../features/skills/set.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                UploadStorage::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<201, Json<StorageSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum UploadStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: StorageKey,
            #[builder(default, into)] pub(crate) description: Description,
            pub(crate) data: Vec<u8>,
        }
    }
}

impl UploadStorage {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<UploadStorage>,
    ) -> Result<(StatusCode, Json<StorageResponse>), StorageError> {
        let response = StorageService::upload(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{ref_key}",
        summary: "Show a file",
        description: "Retrieve metadata and content for a specific archived blob.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetStorage::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<RefKeyPathParam<StorageKey>>()
                        .response::<200, Json<StorageDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<StorageKey>,
        }
    }
}

impl GetStorage {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(ref_key): Path<String>,
    ) -> Result<Json<StorageResponse>, StorageError> {
        let key = parse_storage_key(&ref_key)?;
        Ok(Json(
            StorageService::show(&scope, &GetStorage::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List files",
        description: "See all blobs currently stored in the project's archive.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListStorage::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<StorageEntriesResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListStorage {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListStorage {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListStorage>,
    ) -> Result<Json<StorageResponse>, StorageError> {
        Ok(Json(StorageService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{ref_key}",
        summary: "Remove a file",
        description: "Delete a blob from the project's archive.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveStorage::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<RefKeyPathParam<StorageKey>>()
                        .response::<200, Json<StorageRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveStorage {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: StorageKey,
        }
    }
}

impl RemoveStorage {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(ref_key): Path<String>,
    ) -> Result<Json<StorageResponse>, StorageError> {
        let storage_ref = StorageRef(ref_key);
        let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
        Ok(Json(
            StorageService::remove(
                &scope,
                &mailbox,
                &RemoveStorage::builder_v1().key(key).build().into(),
            )
            .await?,
        ))
    }
}

fn parse_storage_key(ref_key: &str) -> Result<ResourceKey<StorageKey>, StorageError> {
    if ref_key.starts_with(REF_PREFIX) {
        Ok(ResourceKey::Ref(
            ref_key.parse().map_err(|_| StorageError::InvalidRef)?,
        ))
    } else {
        let storage_ref = StorageRef(ref_key.to_string());
        Ok(ResourceKey::Key(
            storage_ref.decode().map_err(|_| StorageError::InvalidRef)?,
        ))
    }
}

/// Raw blob bytes — application/octet-stream, not JSON, so it lives
/// outside aide's typed routing.
pub(crate) async fn content(
    scope: Scope<AtBookmark>,
    Path(ref_key): Path<String>,
) -> Result<impl IntoResponse, StorageError> {
    let storage_ref = StorageRef(ref_key);
    let key = storage_ref.decode().map_err(|_| StorageError::InvalidRef)?;
    let bytes = StorageService::get_content(&scope, &key).await?;
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/octet-stream")],
        bytes,
    ))
}

resource_requests! {
    GetStorage => |this, client| {
        let GetStorage::V1(lookup) = this;
        let path = match &lookup.key {
            ResourceKey::Key(key) => StorageRef::encode(key).to_string(),
            ResourceKey::Ref(token) => token.to_string(),
        };
        client.get(&format!("/storage/{path}")).await
    },
    RemoveStorage => |this, client| {
        let RemoveStorage::V1(removal) = this;
        let ref_key = StorageRef::encode(&removal.key);
        client.delete(&format!("/storage/{ref_key}")).await
    },
    UploadStorage => |this, client| {
        client.post("/storage", this).await
    },
    ListStorage => |this, client| {
        let ListStorage::V1(listing) = this;
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset);
        client.get(&format!("/storage?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(
    kind = StorageRequestType,
    display = "kebab-case",
    attrs(
        expect(
            clippy::enum_variant_names,
            reason = "We use these for `type` notation in serde"
        )
    )
)]
#[expect(
    clippy::enum_variant_names,
    reason = "We use these for `type` notation in serde"
)]
pub(crate) enum StorageRequest {
    UploadStorage(UploadStorage),
    GetStorage(GetStorage),
    ListStorage(ListStorage),
    RemoveStorage(RemoveStorage),
}

resource_root! {
    StorageRequest => {
        label: "storage",
        purpose: "Archive and retrieve files",
        operations: [UploadStorage, GetStorage, ListStorage, RemoveStorage],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (StorageRequestType::UploadStorage, "upload-storage"),
            (StorageRequestType::GetStorage, "get-storage"),
            (StorageRequestType::ListStorage, "list-storage"),
            (StorageRequestType::RemoveStorage, "remove-storage"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
