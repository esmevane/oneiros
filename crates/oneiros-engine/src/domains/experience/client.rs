use crate::*;

pub(crate) struct ExperienceClient<'a> {
    client: &'a Client,
}

impl<'a> ExperienceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateExperience,
    ) -> Result<ExperienceResponse, ClientError> {
        self.client.post("/experiences", request).await
    }

    pub(crate) async fn list(&self, request: &ListExperiences) -> Result<ExperienceResponse, ClientError> {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(a) = &request.agent {
            params.push(("agent", a.to_string()));
        }

        params.push(("limit", request.filters.limit.to_string()));
        params.push(("offset", request.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        self.client.get(&format!("/experiences?{query}")).await
    }

    pub(crate) async fn get(&self, request: &GetExperience) -> Result<ExperienceResponse, ClientError> {
        self.client
            .get(&format!("/experiences/{}", request.id))
            .await
    }

    pub(crate) async fn update_description(
        &self,
        request: &UpdateExperienceDescription,
    ) -> Result<ExperienceResponse, ClientError> {
        self.client
            .put(
                &format!("/experiences/{}/description", request.id),
                &serde_json::json!({ "description": request.description }),
            )
            .await
    }

    pub(crate) async fn update_sensation(
        &self,
        request: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ClientError> {
        self.client
            .put(
                &format!("/experiences/{}/sensation", request.id),
                &serde_json::json!({ "sensation": request.sensation }),
            )
            .await
    }
}
