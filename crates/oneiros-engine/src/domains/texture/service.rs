use crate::*;

pub struct TextureService;

impl TextureService {
    pub async fn set(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let SetTexture::V1(set) = request;
        let texture = Texture::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();
        let name = texture.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Texture(TextureEvents::TextureSet(
                TextureSet::builder_v1().texture(texture).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let projected = TextureRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(TextureError::NotFound(name))?;

        Ok(TextureResponse::TextureSet(
            TextureSetResponse::builder_v1()
                .texture(projected)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetTexture,
    ) -> Result<TextureResponse, TextureError> {
        let GetTexture::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let texture = TextureRepo::new(scope)
            .fetch(&name)
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
        scope: &Scope<AtBookmark>,
        request: &ListTextures,
    ) -> Result<TextureResponse, TextureError> {
        let ListTextures::V1(listing) = request;
        let listed = TextureRepo::new(scope).list(&listing.filters).await?;
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
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveTexture,
    ) -> Result<TextureResponse, TextureError> {
        let RemoveTexture::V1(removal) = request;
        let name = removal.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Texture(TextureEvents::TextureRemoved(
                TextureRemoved::builder_v1()
                    .name(name.clone())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { TextureRepo::new(scope).get(&name).await })
            .await?;

        Ok(TextureResponse::TextureRemoved(
            TextureRemovedResponse::builder_v1()
                .name(name)
                .build()
                .into(),
        ))
    }
}
