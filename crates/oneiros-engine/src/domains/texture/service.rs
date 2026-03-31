use crate::*;

pub struct TextureService;

impl TextureService {
    pub async fn set(
        context: &ProjectContext,
        SetTexture {
            name,
            description,
            prompt,
        }: &SetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let texture = Texture::builder()
            .name(name.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();
        context.emit(TextureEvents::TextureSet(texture)).await?;
        Ok(TextureResponse::TextureSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let texture = TextureRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| TextureError::NotFound(selector.name.clone()))?;
        Ok(TextureResponse::TextureDetails(texture))
    }

    pub async fn list(context: &ProjectContext) -> Result<TextureResponse, TextureError> {
        let textures = TextureRepo::new(context).list().await?;
        if textures.is_empty() {
            Ok(TextureResponse::NoTextures)
        } else {
            Ok(TextureResponse::Textures(textures))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveTexture,
    ) -> Result<TextureResponse, TextureError> {
        context
            .emit(TextureEvents::TextureRemoved(TextureRemoved {
                name: selector.name.clone(),
            }))
            .await?;
        Ok(TextureResponse::TextureRemoved(selector.name.clone()))
    }
}
