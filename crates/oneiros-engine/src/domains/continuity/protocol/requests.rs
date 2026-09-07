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
        path: "/",
        summary: "Emerge an agent",
        description: "Bring an agent online for the first time, establishing its initial cognitive presence.",
        content: include_str!("../features/skills/emerge.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                EmergeAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<201, Json<EmergedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum EmergeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: AgentName,
            #[builder(into)] pub(crate) persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)] pub(crate) description: Description,
        }
    }
}

impl EmergeAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<EmergeAgent>,
    ) -> Result<(StatusCode, Json<ContinuityResponse>), ContinuityError> {
        Ok((
            StatusCode::CREATED,
            Json(
                ContinuityService::emerge(&scope, &mailbox, &body, &DreamOverrides::default())
                    .await?,
            ),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Get agent status",
        description: "Report the current lifecycle state and cognitive activity level of an agent.",
        content: include_str!("../features/skills/status.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                StatusAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<StatusResponse>>()
                },
            )
        },
    })]
    pub(crate) enum StatusAgent {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl StatusAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<StatusAgent>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(ContinuityService::status(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}",
        summary: "Recede an agent",
        description: "Gracefully withdraw an agent from active service, preserving its accumulated context.",
        content: include_str!("../features/skills/recede.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RecedeAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<RecededResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RecedeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl RecedeAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::recede(
                &scope,
                &mailbox,
                &RecedeAgent::builder_v1().agent(agent).build().into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/wake",
        summary: "Wake an agent",
        description: "Restore a dormant agent to an active state, making it ready to receive work.",
        content: include_str!("../features/skills/wake.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                WakeAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<WakingResponse>>()
                },
            )
        },
    })]
    pub(crate) enum WakeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl WakeAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
        Query(overrides): Query<DreamOverrides>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::wake(
                &scope,
                &mailbox,
                &WakeAgent::builder_v1().agent(agent).build().into(),
                &overrides,
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/dream",
        summary: "Dream an agent",
        description: "Generate a cognitive context document from the agent's accumulated thoughts and memories.",
        content: include_str!("../features/skills/dream.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                DreamAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<DreamingResponse>>()
                },
            )
        },
    })]
    pub(crate) enum DreamAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl DreamAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
        Query(overrides): Query<DreamOverrides>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::dream(
                &scope,
                &mailbox,
                &DreamAgent::builder_v1().agent(agent).build().into(),
                &overrides,
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/introspect",
        summary: "Introspect an agent",
        description: "Summarize the agent's current session into consolidated memories before context compaction.",
        content: include_str!("../features/skills/introspect.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                IntrospectAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<IntrospectingResponse>>()
                },
            )
        },
    })]
    pub(crate) enum IntrospectAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl IntrospectAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
        Query(overrides): Query<DreamOverrides>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::introspect(
                &scope,
                &mailbox,
                &IntrospectAgent::builder_v1().agent(agent).build().into(),
                &overrides,
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/reflect",
        summary: "Reflect an agent",
        description: "Capture a snapshot of the agent's present reasoning as a durable reflection.",
        content: include_str!("../features/skills/reflect.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                ReflectAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<ReflectingResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ReflectAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl ReflectAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
        Query(overrides): Query<DreamOverrides>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::reflect(
                &scope,
                &mailbox,
                &ReflectAgent::builder_v1().agent(agent).build().into(),
                &overrides,
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/sense",
        summary: "Sense an agent",
        description: "Sample the agent's current cognitive pressure and activity without modifying its state.",
        content: include_str!("../features/skills/sense.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                SenseContent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<SleepingResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SenseContent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) content: Content,
        }
    }
}

impl SenseContent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
        Json(body): Json<SenseContent>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        let SenseContent::V1(sensing) = body;
        let request: SenseContent = SenseContent::builder_v1()
            .agent(agent)
            .content(sensing.content)
            .build()
            .into();
        Ok(Json(
            ContinuityService::sense(&scope, &mailbox, &request, &DreamOverrides::default())
                .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/sleep",
        summary: "Sleep an agent",
        description: "Suspend an agent into a dormant state, pausing cognitive activity while preserving context.",
        content: include_str!("../features/skills/sleep.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                SleepAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<SleepingResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SleepAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl SleepAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(agent): Path<AgentName>,
        Query(overrides): Query<DreamOverrides>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::sleep(
                &scope,
                &mailbox,
                &SleepAgent::builder_v1().agent(agent).build().into(),
                &overrides,
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}/guidebook",
        summary: "Get agent guidebook",
        description: "Retrieve the accumulated guidance document that shapes the agent's cognitive style and constraints.",
        content: include_str!("../features/skills/guidebook.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GuidebookAgent::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<AgentPathParam<AgentName>>().response::<200, Json<GuidebookResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GuidebookAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

impl GuidebookAgent {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(agent): Path<AgentName>,
        Query(overrides): Query<DreamOverrides>,
    ) -> Result<Json<ContinuityResponse>, ContinuityError> {
        Ok(Json(
            ContinuityService::guidebook(
                &scope,
                &GuidebookAgent::builder_v1().agent(agent).build().into(),
                &overrides,
            )
            .await?,
        ))
    }
}

fn encode_dream_overrides(overrides: &DreamOverrides) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(value) = overrides.recent_window {
        parts.push(format!("recent_window={value}"));
    }
    if let Some(value) = overrides.dream_depth {
        parts.push(format!("dream_depth={value}"));
    }
    if let Some(value) = overrides.cognition_size {
        parts.push(format!("cognition_size={value}"));
    }
    if let Some(value) = &overrides.recollection_level {
        parts.push(format!("recollection_level={value}"));
    }
    if let Some(value) = overrides.recollection_size {
        parts.push(format!("recollection_size={value}"));
    }
    if let Some(value) = overrides.experience_size {
        parts.push(format!("experience_size={value}"));
    }
    parts.join("&")
}

resource_requests! {
    WakeAgent => |this, client| {
        let WakeAgent::V1(wake) = this;
        client
            .post(
                &format!("/continuity/{agent}/wake", agent = wake.agent),
                &serde_json::Value::Null,
            )
            .await
    },
    DreamAgent => |this, client| {
        let DreamAgent::V1(dream) = this;
        let query = encode_dream_overrides(&DreamOverrides::default());
        let path = if query.is_empty() {
            format!("/continuity/{agent}/dream", agent = dream.agent)
        } else {
            format!("/continuity/{agent}/dream?{query}", agent = dream.agent)
        };
        client.post(&path, &serde_json::Value::Null).await
    },
    IntrospectAgent => |this, client| {
        let IntrospectAgent::V1(introspecting) = this;
        client
            .post(
                &format!(
                    "/continuity/{agent}/introspect",
                    agent = introspecting.agent
                ),
                &serde_json::Value::Null,
            )
            .await
    },
    ReflectAgent => |this, client| {
        let ReflectAgent::V1(reflecting) = this;
        client
            .post(
                &format!("/continuity/{agent}/reflect", agent = reflecting.agent),
                &serde_json::Value::Null,
            )
            .await
    },
    SenseContent => |this, client| {
        let SenseContent::V1(sense) = this;
        client
            .post(
                &format!("/continuity/{agent}/sense", agent = sense.agent),
                this,
            )
            .await
    },
    SleepAgent => |this, client| {
        let SleepAgent::V1(sleeping) = this;
        client
            .post(
                &format!("/continuity/{agent}/sleep", agent = sleeping.agent),
                &serde_json::Value::Null,
            )
            .await
    },
    GuidebookAgent => |this, client| {
        let GuidebookAgent::V1(lookup) = this;
        client
            .get(&format!(
                "/continuity/{agent}/guidebook",
                agent = lookup.agent
            ))
            .await
    },
    EmergeAgent => |this, client| {
        client.post("/continuity", this).await
    },
    RecedeAgent => |this, client| {
        let RecedeAgent::V1(receding) = this;
        client
            .delete(&format!("/continuity/{agent}", agent = receding.agent))
            .await
    },
}

resource_requests! {
    StatusAgent => |client| { client.get("/continuity").await },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ContinuityRequestType, display = "kebab-case")]
pub(crate) enum ContinuityRequest {
    WakeAgent(WakeAgent),
    DreamAgent(DreamAgent),
    IntrospectAgent(IntrospectAgent),
    ReflectAgent(ReflectAgent),
    SenseContent(SenseContent),
    SleepAgent(SleepAgent),
    GuidebookAgent(GuidebookAgent),
    EmergeAgent(EmergeAgent),
    RecedeAgent(RecedeAgent),
    StatusAgent(StatusAgent),
}

resource_root! {
    ContinuityRequest => {
        label: "continuity",
        purpose: "Agent lifecycle and cognitive operations",
        operations: [EmergeAgent, StatusAgent, RecedeAgent, WakeAgent, DreamAgent, IntrospectAgent, ReflectAgent, SenseContent, SleepAgent, GuidebookAgent],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ContinuityRequestType::WakeAgent, "wake-agent"),
            (ContinuityRequestType::DreamAgent, "dream-agent"),
            (ContinuityRequestType::IntrospectAgent, "introspect-agent"),
            (ContinuityRequestType::ReflectAgent, "reflect-agent"),
            (ContinuityRequestType::SenseContent, "sense-content"),
            (ContinuityRequestType::SleepAgent, "sleep-agent"),
            (ContinuityRequestType::GuidebookAgent, "guidebook-agent"),
            (ContinuityRequestType::EmergeAgent, "emerge-agent"),
            (ContinuityRequestType::RecedeAgent, "recede-agent"),
            (ContinuityRequestType::StatusAgent, "status-agent"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
