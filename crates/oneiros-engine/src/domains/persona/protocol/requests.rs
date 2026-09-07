use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Set a persona",
        description: "Create or update an agent category.",
        content: include_str!("../features/skills/set.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Put,
        build: |docs| {
            ResourceMethod::Put.router(
                SetPersona::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<PersonaName>>()
                        .response::<200, Json<PersonaSetResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SetPersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl SetPersona {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<PersonaName>,
        Json(body): Json<SetPersona>,
    ) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
        let SetPersona::V1(mut setting) = body;
        setting.name = name;
        let request = SetPersona::V1(setting);
        Ok((
            StatusCode::OK,
            Json(PersonaService::set(&scope, &mailbox, &request).await?),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Get a persona",
        description: "Retrieve a single agent category by name.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetPersona::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<PersonaName>>()
                        .response::<200, Json<PersonaDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetPersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<PersonaName>,
        }
    }
}

impl GetPersona {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<PersonaName>>,
    ) -> Result<Json<PersonaResponse>, PersonaError> {
        Ok(Json(
            PersonaService::get(&scope, &GetPersona::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Remove a persona",
        description: "Delete an agent category from the project.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemovePersona::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<NamePathParam<PersonaName>>()
                        .response::<200, Json<PersonaRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemovePersona {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: PersonaName,
        }
    }
}

impl RemovePersona {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(name): Path<PersonaName>,
    ) -> Result<Json<PersonaResponse>, PersonaError> {
        Ok(Json(
            PersonaService::remove(
                &scope,
                &mailbox,
                &RemovePersona::builder_v1().name(name).build().into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List personas",
        description: "See all defined agent categories.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListPersonas::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<PersonasResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListPersonas {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListPersonas {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListPersonas>,
    ) -> Result<Json<PersonaResponse>, PersonaError> {
        Ok(Json(PersonaService::list(&scope, &params).await?))
    }
}

resource_requests! {
    SetPersona => |this, client| {
        let SetPersona::V1(body) = this;
        client
            .put(&format!("/personas/{name}", name = body.name), this)
            .await
    },
    GetPersona => |this, client| {
        let GetPersona::V1(lookup) = this;
        client.get(&format!("/personas/{}", lookup.key)).await
    },
    RemovePersona => |this, client| {
        let RemovePersona::V1(removal) = this;
        client.delete(&format!("/personas/{}", removal.name)).await
    },
    ListPersonas => |this, client| {
        let ListPersonas::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/personas?{query}")).await
    },
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

resource_root! {
    PersonaRequest => {
        label: "personas",
        purpose: "Define categories of agents",
        operations: [SetPersona, GetPersona, ListPersonas, RemovePersona],
    }
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
