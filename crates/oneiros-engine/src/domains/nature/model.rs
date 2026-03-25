use bon::Builder;
use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Args, Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Nature {
    #[builder(into)]
    pub name: NatureName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

resource_name!(NatureName);
