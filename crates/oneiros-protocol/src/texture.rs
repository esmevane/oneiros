use oneiros_model::{Texture, TextureName};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureEvents {
    TextureSet(Texture),
    TextureRemoved { name: TextureName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureRequests {
    SetTexture(Texture),
    RemoveTexture { name: TextureName },
}
