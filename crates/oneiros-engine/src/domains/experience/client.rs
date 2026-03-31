use crate::*;

pub struct ExperienceClient<'a> {
    client: &'a Client,
}

impl<'a> ExperienceClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &CreateExperience,
    ) -> Result<ExperienceResponse, ClientError> {
        self.client.post("/experiences", request).await
    }

    pub async fn list(&self, request: &ListExperiences) -> Result<ExperienceResponse, ClientError> {
        let path = match &request.agent {
            Some(a) => format!("/experiences?agent={a}"),
            None => "/experiences".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, request: &GetExperience) -> Result<ExperienceResponse, ClientError> {
        self.client
            .get(&format!("/experiences/{}", request.id))
            .await
    }

    pub async fn update_description(
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

    pub async fn update_sensation(
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
