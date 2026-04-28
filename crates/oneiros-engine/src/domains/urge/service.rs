use crate::*;

pub struct UrgeService;

impl UrgeService {
    pub async fn set(
        context: &ProjectContext,
        request: &SetUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let SetUrge::V1(set) = request;
        let urge = Urge::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        context
            .emit(UrgeEvents::UrgeSet(
                UrgeSet::builder_v1().urge(urge.clone()).build().into(),
            ))
            .await?;

        Ok(UrgeResponse::UrgeSet(
            UrgeSetResponse::builder_v1().urge(urge).build().into(),
        ))
    }

    pub async fn get(
        context: &ProjectContext,
        request: &GetUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let GetUrge::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let urge = UrgeRepo::new(context)
            .get(&name)
            .await?
            .ok_or(UrgeError::NotFound(name))?;
        Ok(UrgeResponse::UrgeDetails(
            UrgeDetailsResponse::builder_v1().urge(urge).build().into(),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        request: &ListUrges,
    ) -> Result<UrgeResponse, UrgeError> {
        let ListUrges::V1(listing) = request;
        let listed = UrgeRepo::new(context).list(&listing.filters).await?;
        if listed.total == 0 {
            Ok(UrgeResponse::NoUrges)
        } else {
            Ok(UrgeResponse::Urges(
                UrgesResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        request: &RemoveUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let RemoveUrge::V1(removal) = request;
        context
            .emit(UrgeEvents::UrgeRemoved(
                UrgeRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;
        Ok(UrgeResponse::UrgeRemoved(
            UrgeRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
