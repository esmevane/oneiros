use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SetTexture {
    #[builder(into)]
    pub(crate) name: TextureName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetTexture {
    #[builder(into)]
    pub(crate) name: TextureName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RemoveTexture {
    #[builder(into)]
    pub(crate) name: TextureName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListTextures {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TextureRequestType, display = "kebab-case")]
pub(crate) enum TextureRequest {
    SetTexture(SetTexture),
    GetTexture(GetTexture),
    ListTextures(ListTextures),
    RemoveTexture(RemoveTexture),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (TextureRequestType::SetTexture, "set-texture"),
            (TextureRequestType::GetTexture, "get-texture"),
            (TextureRequestType::ListTextures, "list-textures"),
            (TextureRequestType::RemoveTexture, "remove-texture"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
