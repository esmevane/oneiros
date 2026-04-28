use crate::*;

pub struct TextureService;

impl TextureService {
    pub async fn set(
        context: &ProjectContext,
        request: &SetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let SetTexture::V1(set) = request;
        let texture = Texture::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        context
            .emit(TextureEvents::TextureSet(
                TextureSet::builder_v1()
                    .texture(texture.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(TextureResponse::TextureSet(
            TextureSetResponse::builder_v1()
                .texture(texture)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        request: &GetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let GetTexture::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let texture = TextureRepo::new(context)
            .get(&name)
            .await?
            .ok_or(TextureError::NotFound(name))?;
        Ok(TextureResponse::TextureDetails(
            TextureDetailsResponse::builder_v1()
                .texture(texture)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        request: &ListTextures,
    ) -> Result<TextureResponse, TextureError> {
        let ListTextures::V1(listing) = request;
        let listed = TextureRepo::new(context).list(&listing.filters).await?;
        if listed.total == 0 {
            Ok(TextureResponse::NoTextures)
        } else {
            Ok(TextureResponse::Textures(
                TexturesResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        request: &RemoveTexture,
    ) -> Result<TextureResponse, TextureError> {
        let RemoveTexture::V1(removal) = request;
        context
            .emit(TextureEvents::TextureRemoved(
                TextureRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;
        Ok(TextureResponse::TextureRemoved(
            TextureRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
