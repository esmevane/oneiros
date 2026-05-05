use crate::*;

pub struct SensationService;

impl SensationService {
    pub async fn set(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SetSensation,
    ) -> Result<SensationResponse, SensationError> {
        let SetSensation::V1(set) = request;
        let sensation = Sensation::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();
        let name = sensation.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Sensation(SensationEvents::SensationSet(
                SensationSet::builder_v1()
                    .sensation(sensation)
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let projected = SensationRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(SensationError::NotFound(name))?;

        Ok(SensationResponse::SensationSet(
            SensationSetResponse::builder_v1()
                .sensation(projected)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetSensation,
    ) -> Result<SensationResponse, SensationError> {
        let GetSensation::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let sensation = SensationRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(SensationError::NotFound(name))?;
        Ok(SensationResponse::SensationDetails(
            SensationDetailsResponse::builder_v1()
                .sensation(sensation)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        scope: &Scope<AtBookmark>,
        request: &ListSensations,
    ) -> Result<SensationResponse, SensationError> {
        let ListSensations::V1(listing) = request;
        let listed = SensationRepo::new(scope).list(&listing.filters).await?;
        if listed.total == 0 {
            Ok(SensationResponse::NoSensations)
        } else {
            Ok(SensationResponse::Sensations(
                SensationsResponse::builder_v1()
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
        request: &RemoveSensation,
    ) -> Result<SensationResponse, SensationError> {
        let RemoveSensation::V1(removal) = request;
        let name = removal.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Sensation(SensationEvents::SensationRemoved(
                SensationRemoved::builder_v1()
                    .name(name.clone())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { SensationRepo::new(scope).get(&name).await })
            .await?;

        Ok(SensationResponse::SensationRemoved(
            SensationRemovedResponse::builder_v1()
                .name(name)
                .build()
                .into(),
        ))
    }
}
