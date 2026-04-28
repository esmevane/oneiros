use crate::*;

pub struct SensationService;

impl SensationService {
    pub async fn set(
        context: &ProjectContext,
        request: &SetSensation,
    ) -> Result<SensationResponse, SensationError> {
        let SetSensation::V1(set) = request;
        let sensation = Sensation::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        context
            .emit(SensationEvents::SensationSet(
                SensationSet::builder_v1()
                    .sensation(sensation.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(SensationResponse::SensationSet(
            SensationSetResponse::builder_v1()
                .sensation(sensation)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        request: &GetSensation,
    ) -> Result<SensationResponse, SensationError> {
        let GetSensation::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let sensation = SensationRepo::new(context)
            .get(&name)
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
        context: &ProjectContext,
        request: &ListSensations,
    ) -> Result<SensationResponse, SensationError> {
        let ListSensations::V1(listing) = request;
        let listed = SensationRepo::new(context).list(&listing.filters).await?;
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
        context: &ProjectContext,
        request: &RemoveSensation,
    ) -> Result<SensationResponse, SensationError> {
        let RemoveSensation::V1(removal) = request;
        context
            .emit(SensationEvents::SensationRemoved(
                SensationRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;
        Ok(SensationResponse::SensationRemoved(
            SensationRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
