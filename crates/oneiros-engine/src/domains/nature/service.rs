use crate::*;

pub struct NatureService;

impl NatureService {
    pub async fn set(
        context: &ProjectContext,
        request: &SetNature,
    ) -> Result<NatureResponse, NatureError> {
        let SetNature::V1(set) = request;
        let nature = Nature::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        context
            .emit(NatureEvents::NatureSet(
                NatureSet::builder_v1()
                    .nature(nature.clone())
                    .build()
                    .into(),
            ))
            .await?;

        Ok(NatureResponse::NatureSet(
            NatureSetResponse::builder_v1()
                .nature(nature)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        request: &GetNature,
    ) -> Result<NatureResponse, NatureError> {
        let GetNature::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let nature = NatureRepo::new(context)
            .get(&name)
            .await?
            .ok_or(NatureError::NotFound(name))?;
        Ok(NatureResponse::NatureDetails(
            NatureDetailsResponse::builder_v1()
                .nature(nature)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        request: &ListNatures,
    ) -> Result<NatureResponse, NatureError> {
        let ListNatures::V1(listing) = request;
        let listed = NatureRepo::new(context).list(&listing.filters).await?;
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

    pub async fn remove(
        context: &ProjectContext,
        request: &RemoveNature,
    ) -> Result<NatureResponse, NatureError> {
        let RemoveNature::V1(removal) = request;
        context
            .emit(NatureEvents::NatureRemoved(
                NatureRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;
        Ok(NatureResponse::NatureRemoved(
            NatureRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
