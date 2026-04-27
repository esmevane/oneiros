use crate::*;

pub struct NatureService;

impl NatureService {
    pub async fn set(
        context: &ProjectContext,
        SetNature {
            name,
            description,
            prompt,
        }: &SetNature,
    ) -> Result<NatureResponse, NatureError> {
        let nature = Nature::Current(
            Nature::build_v1()
                .name(name.clone())
                .description(description.clone())
                .prompt(prompt.clone())
                .build(),
        );
        context.emit(NatureEvents::NatureSet(nature)).await?;
        Ok(NatureResponse::NatureSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetNature,
    ) -> Result<NatureResponse, NatureError> {
        let name = selector.key.resolve()?;
        let nature = NatureRepo::new(context)
            .get(&name)
            .await?
            .ok_or(NatureError::NotFound(name))?;
        let ref_token = RefToken::new(Ref::nature(nature.name().clone()));
        Ok(NatureResponse::NatureDetails(
            Response::new(nature).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListNatures { filters }: &ListNatures,
    ) -> Result<NatureResponse, NatureError> {
        let listed = NatureRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(NatureResponse::NoNatures)
        } else {
            Ok(NatureResponse::Natures(listed.map(|e| {
                let ref_token = RefToken::new(Ref::nature(e.name().clone()));
                Response::new(e).with_ref_token(ref_token)
            })))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveNature,
    ) -> Result<NatureResponse, NatureError> {
        context
            .emit(NatureEvents::NatureRemoved(NatureRemoved::Current(
                NatureRemovedV1 {
                    name: selector.name.clone(),
                },
            )))
            .await?;
        Ok(NatureResponse::NatureRemoved(selector.name.clone()))
    }
}
