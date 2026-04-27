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
        let texture = Texture::Current(
            Texture::build_v1()
                .name(name.clone())
                .description(description.clone())
                .prompt(prompt.clone())
                .build(),
        );
        context.emit(TextureEvents::TextureSet(texture)).await?;
        Ok(TextureResponse::TextureSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let name = selector.key.resolve()?;
        let texture = TextureRepo::new(context)
            .get(&name)
            .await?
            .ok_or(TextureError::NotFound(name))?;
        let ref_token = RefToken::new(Ref::texture(texture.name().clone()));
        Ok(TextureResponse::TextureDetails(
            Response::new(texture).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListTextures { filters }: &ListTextures,
    ) -> Result<TextureResponse, TextureError> {
        let listed = TextureRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(TextureResponse::NoTextures)
        } else {
            Ok(TextureResponse::Textures(listed.map(|e| {
                let ref_token = RefToken::new(Ref::texture(e.name().clone()));
                Response::new(e).with_ref_token(ref_token)
            })))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveTexture,
    ) -> Result<TextureResponse, TextureError> {
        context
            .emit(TextureEvents::TextureRemoved(TextureRemoved::Current(
                TextureRemovedV1 {
                    name: selector.name.clone(),
                },
            )))
            .await?;
        Ok(TextureResponse::TextureRemoved(selector.name.clone()))
    }
}
