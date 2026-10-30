use crate::*;

pub(crate) struct NatureService;

impl NatureService {
    pub(crate) async fn set(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SetNature,
    ) -> Result<NatureResponse, NatureError> {
        let SetNature::V1(set) = request;
        let nature = Nature::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Nature(NatureEvents::NatureSet(
                NatureSet::builder_v1()
                    .nature(nature.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(NatureResponse::NatureSet(
            NatureSetResponse::builder_v1()
                .nature(nature)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetNature,
    ) -> Result<NatureResponse, NatureError> {
        let GetNature::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let nature = NatureRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(NatureError::NotFound(name))?;
        Ok(NatureResponse::NatureDetails(
            NatureDetailsResponse::builder_v1()
                .nature(nature)
                .build()
                .into(),
        ))
    }

    pub(crate) async fn list(
        scope: &Scope<AtBookmark>,
        request: &ListNatures,
    ) -> Result<NatureResponse, NatureError> {
        let ListNatures::V1(listing) = request;
        let listed = NatureRepo::new(scope).list(&listing.filters).await?;
        if listed.total == 0 {
            Ok(NatureResponse::NoNatures)
        } else {
            Ok(NatureResponse::Natures(
                NaturesResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub(crate) async fn remove(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveNature,
    ) -> Result<NatureResponse, NatureError> {
        let RemoveNature::V1(removal) = request;
        let name = removal.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Nature(NatureEvents::NatureRemoved(
                NatureRemoved::builder_v1()
                    .name(name.clone())
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(scope.append(new_event.clone()));
        scope.ensure_persisted(&new_event).await?;

        Ok(NatureResponse::NatureRemoved(
            NatureRemovedResponse::builder_v1()
                .name(name)
                .build()
                .into(),
        ))
    }
}
