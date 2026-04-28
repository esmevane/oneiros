use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ContinuityResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ContinuityResponse {
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
    pub enum EmergedResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum WakingResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum DreamingResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum IntrospectingResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ReflectingResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SleepingResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GuidebookResponse {
        V1 => {
            pub context: DreamContext,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RecededResponse {
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum StatusResponse {
        V1 => {
            pub table: AgentActivityTable,
        }
    }
}
