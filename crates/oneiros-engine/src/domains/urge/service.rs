use crate::*;

pub struct UrgeService;

impl UrgeService {
    pub async fn set(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SetUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let SetUrge::V1(set) = request;
        let urge = Urge::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();
        let name = urge.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Urge(UrgeEvents::UrgeSet(
                UrgeSet::builder_v1().urge(urge).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let projected = UrgeRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(UrgeError::NotFound(name))?;

        Ok(UrgeResponse::UrgeSet(
            UrgeSetResponse::builder_v1().urge(projected).build().into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let GetUrge::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let urge = UrgeRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(UrgeError::NotFound(name))?;
        Ok(UrgeResponse::UrgeDetails(
            UrgeDetailsResponse::builder_v1().urge(urge).build().into(),
        ))
    }

    pub async fn list(
        scope: &Scope<AtBookmark>,
        request: &ListUrges,
    ) -> Result<UrgeResponse, UrgeError> {
        let ListUrges::V1(listing) = request;
        let listed = UrgeRepo::new(scope).list(&listing.filters).await?;
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
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let RemoveUrge::V1(removal) = request;
        let name = removal.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Urge(UrgeEvents::UrgeRemoved(
                UrgeRemoved::builder_v1().name(name.clone()).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { UrgeRepo::new(scope).get(&name).await })
            .await?;

        Ok(UrgeResponse::UrgeRemoved(
            UrgeRemovedResponse::builder_v1().name(name).build().into(),
        ))
    }
}
