use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ContinuityResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ContinuityResponse {
    Emerged(EmergedResponse),
    Waking(WakingResponse),
    Dreaming(DreamingResponse),
    Introspecting(IntrospectingResponse),
    Reflecting(ReflectingResponse),
    Sleeping(SleepingResponse),
    Receded(RecededResponse),
    Status(StatusResponse),
    Guidebook(GuidebookResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum EmergedResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum WakingResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum DreamingResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum IntrospectingResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ReflectingResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SleepingResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GuidebookResponse {
        V1 => {
            pub(crate) context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RecededResponse {
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum StatusResponse {
        V1 => {
            pub(crate) table: AgentActivityTable,
        }
    }
}
