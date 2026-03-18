use crate::*;

pub struct TextureService;

impl TextureService {
    pub fn set(ctx: &ProjectContext, texture: Texture) -> Result<TextureResponse, TextureError> {
        let name = texture.name.clone();
        ctx.emit(TextureEvents::TextureSet(texture));
        Ok(TextureResponse::TextureSet(name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<TextureResponse, TextureError> {
        let texture = ctx
            .with_db(|conn| TextureRepo::new(conn).get(name))
            .map_err(TextureError::Database)?
            .ok_or_else(|| TextureError::NotFound(name.to_string()))?;
        Ok(TextureResponse::TextureDetails(texture))
    }

    pub fn list(ctx: &ProjectContext) -> Result<TextureResponse, TextureError> {
        let textures = ctx
            .with_db(|conn| TextureRepo::new(conn).list())
            .map_err(TextureError::Database)?;
        if textures.is_empty() {
            Ok(TextureResponse::NoTextures)
        } else {
            Ok(TextureResponse::Textures(textures))
        }
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<TextureResponse, TextureError> {
        let texture_name = TextureName::new(name);
        ctx.emit(TextureEvents::TextureRemoved(TextureRemoved {
            name: texture_name.clone(),
        }));
        Ok(TextureResponse::TextureRemoved(texture_name))
    }
}
