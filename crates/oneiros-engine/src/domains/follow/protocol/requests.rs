use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Get a follow",
        description: "Retrieve a single follow record by its identifier.",
        content: include_str!("../features/skills/get.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetFollow::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.input::<IdPathParam<FollowId>>().response::<200, Json<FollowFoundResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetFollow {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<FollowId>,
        }
    }
}

impl GetFollow {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<FollowId>>,
    ) -> Result<Json<Response<FollowResponse>>, FollowError> {
        let id = key.resolve()?;
        let follow = FollowService::get(&scope, id).await?;

        let inner = FollowFoundResponse::builder_v1()
            .follow(follow)
            .build()
            .into();

        Ok(Json(Response::new(FollowResponse::Found(inner))))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List follows",
        description: "List all follow records on this host.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListFollows::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<FollowsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListFollows {
        #[derive(clap::Args)]
        V1 => {
            #[serde(flatten)]
            #[clap(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListFollows {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Query(params): Query<ListFollows>,
    ) -> Result<Json<Response<FollowResponse>>, FollowError> {
        let ListFollows::V1(listing) = &params;
        let listed = FollowService::list(&scope, &listing.filters).await?;

        let items: Vec<Response<FollowFoundResponse>> = listed
            .items
            .into_iter()
            .map(|follow| {
                Response::new(
                    FollowFoundResponse::builder_v1()
                        .follow(follow)
                        .build()
                        .into(),
                )
            })
            .collect();

        let listed = Listed::new(items, listed.total);
        let inner = FollowsResponse::builder_v1().follows(listed).build().into();

        Ok(Json(Response::new(FollowResponse::Listed(inner))))
    }
}

resource_requests! {
    GetFollow => |this, client| {
        let GetFollow::V1(lookup) = this;
        client.get(&format!("/follows/{}", lookup.key)).await
    },
    ListFollows => |this, client| {
        let ListFollows::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/follows?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = FollowRequestType, display = "kebab-case")]
pub(crate) enum FollowRequest {
    GetFollow(GetFollow),
    ListFollows(ListFollows),
}

resource_root! {
    FollowRequest => {
        label: "follows",
        purpose: "Inspect follow records — links between local bookmarks and the sources they track",
        operations: [GetFollow, ListFollows],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        assert_eq!(&FollowRequestType::GetFollow.to_string(), "get-follow");
        assert_eq!(&FollowRequestType::ListFollows.to_string(), "list-follows");
    }
}
