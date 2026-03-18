use crate::*;

pub struct TextureService;

impl TextureService {
    pub fn set(ctx: &ProjectContext, texture: Texture) -> Result<TextureResponse, TextureError> {
        ctx.emit(TextureEvents::TextureSet(texture.clone()));
        Ok(TextureResponse::Set(texture))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<TextureResponse, TextureError> {
        let texture = ctx
            .with_db(|conn| TextureRepo::new(conn).get(name))
            .map_err(TextureError::Database)?
            .ok_or_else(|| TextureError::NotFound(name.to_string()))?;
        Ok(TextureResponse::Found(texture))
    }

    pub fn list(ctx: &ProjectContext) -> Result<TextureResponse, TextureError> {
        let textures = ctx
            .with_db(|conn| TextureRepo::new(conn).list())
            .map_err(TextureError::Database)?;
        Ok(TextureResponse::Listed(textures))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<TextureResponse, TextureError> {
        ctx.emit(TextureEvents::TextureRemoved(TextureRemoved {
            name: TextureName::new(name),
        }));
        Ok(TextureResponse::Removed)
    }
}
