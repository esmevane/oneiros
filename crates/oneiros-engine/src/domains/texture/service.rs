use crate::*;

pub struct TextureService;

impl TextureService {
    pub async fn set(
        context: &ProjectContext,
        texture: Texture,
    ) -> Result<TextureResponse, TextureError> {
        let name = texture.name.clone();
        context.emit(TextureEvents::TextureSet(texture)).await?;
        Ok(TextureResponse::TextureSet(name))
    }

    pub fn get(
        context: &ProjectContext,
        name: &TextureName,
    ) -> Result<TextureResponse, TextureError> {
        let texture = context
            .with_db(|conn| TextureRepo::new(conn).get(name))?
            .ok_or_else(|| TextureError::NotFound(name.clone()))?;
        Ok(TextureResponse::TextureDetails(texture))
    }

    pub fn list(context: &ProjectContext) -> Result<TextureResponse, TextureError> {
        let textures = context
            .with_db(|conn| TextureRepo::new(conn).list())
            .map_err(TextureError::Database)?;
        if textures.is_empty() {
            Ok(TextureResponse::NoTextures)
        } else {
            Ok(TextureResponse::Textures(textures))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        name: &TextureName,
    ) -> Result<TextureResponse, TextureError> {
        context
            .emit(TextureEvents::TextureRemoved(TextureRemoved {
                name: name.clone(),
            }))
            .await?;
        Ok(TextureResponse::TextureRemoved(name.clone()))
    }
}
