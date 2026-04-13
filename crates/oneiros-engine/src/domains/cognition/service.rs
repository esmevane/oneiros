use crate::*;

pub(crate) struct CognitionService;

impl CognitionService {
    pub(crate) async fn add(
        context: &ProjectContext,
        AddCognition {
            agent,
            texture,
            content,
        }: &AddCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_record = AgentRepo::new(context)
            .get(agent)
            .await?
            .ok_or_else(|| CognitionError::AgentNotFound(agent.clone()))?;

        let cognition = Cognition::builder()
            .agent_id(agent_record.id)
            .texture(texture.clone())
            .content(content.clone())
            .build();

        context
            .emit(CognitionEvents::CognitionAdded(cognition.clone()))
            .await?;
        let ref_token = RefToken::new(Ref::cognition(cognition.id));
        Ok(CognitionResponse::CognitionAdded(
            Response::new(cognition).with_ref_token(ref_token),
        ))
    }

    pub(crate) async fn get(
        context: &ProjectContext,
        selector: &GetCognition,
    ) -> Result<CognitionResponse, CognitionError> {
        let cognition = CognitionRepo::new(context)
            .get(&selector.id)
            .await?
            .ok_or_else(|| CognitionError::NotFound(selector.id))?;
        let ref_token = RefToken::new(Ref::cognition(cognition.id));
        Ok(CognitionResponse::CognitionDetails(
            Response::new(cognition).with_ref_token(ref_token),
        ))
    }

    pub(crate) async fn list(
        context: &ProjectContext,
        ListCognitions {
            agent,
            texture,
            filters,
        }: &ListCognitions,
    ) -> Result<CognitionResponse, CognitionError> {
        let agent_id = match agent {
            Some(name) => {
                let record = AgentRepo::new(context)
                    .get(name)
                    .await?
                    .ok_or_else(|| CognitionError::AgentNotFound(name.clone()))?;
                Some(record.id)
            }
            None => None,
        };

        let listed = CognitionRepo::new(context)
            .list(agent_id.as_ref(), texture.as_ref(), filters)
            .await?;
        Ok(if listed.total == 0 {
            CognitionResponse::NoCognitions
        } else {
            CognitionResponse::Cognitions(listed.map(|c| {
                let ref_token = RefToken::new(Ref::cognition(c.id));
                Response::new(c).with_ref_token(ref_token)
            }))
        })
    }
}
