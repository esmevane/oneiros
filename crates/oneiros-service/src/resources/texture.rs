use oneiros_model::*;

use crate::*;

pub struct TextureStore;

impl Dispatch<TextureRequests> for TextureStore {
    type Response = TextureResponses;
    type Error = Error;

    fn dispatch(
        &self,
        context: RequestContext<'_, TextureRequests>,
    ) -> Result<Self::Response, Self::Error> {
        let db = context.scope.db();

        match context.request {
            TextureRequests::SetTexture(texture) => {
                let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));
                context.scope.effects().emit(&event)?;
                Ok(TextureResponses::TextureSet(texture))
            }
            TextureRequests::ListTextures(_) => {
                Ok(TextureResponses::TexturesListed(db.list_textures()?))
            }
            TextureRequests::GetTexture(request) => {
                let texture = db
                    .get_texture(&request.name)?
                    .ok_or(NotFound::Texture(request.name))?;
                Ok(TextureResponses::TextureFound(texture))
            }
            TextureRequests::RemoveTexture(request) => {
                let event = Events::Texture(TextureEvents::TextureRemoved(SelectTextureByName {
                    name: request.name,
                }));
                context.scope.effects().emit(&event)?;
                Ok(TextureResponses::TextureRemoved)
            }
        }
    }
}
