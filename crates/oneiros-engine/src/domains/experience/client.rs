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
        creation: &CreateExperience,
    ) -> Result<ExperienceResponse, ClientError> {
        self.client.post("/experiences", creation).await
    }

    pub async fn list(&self, listing: &ListExperiences) -> Result<ExperienceResponse, ClientError> {
        let ListExperiences::V1(listing) = listing;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(sensation_name) = &listing.sensation {
            params.push(("sensation", sensation_name.to_string()));
        }

        if let Some(query) = &listing.query {
            params.push(("query", query.clone()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        self.client.get(&format!("/experiences?{query}")).await
    }

    pub async fn get(&self, lookup: &GetExperience) -> Result<ExperienceResponse, ClientError> {
        let GetExperience::V1(lookup) = lookup;
        self.client
            .get(&format!("/experiences/{}", lookup.key))
            .await
    }

    pub async fn update_description(
        &self,
        update: &UpdateExperienceDescription,
    ) -> Result<ExperienceResponse, ClientError> {
        let UpdateExperienceDescription::V1(update) = update;
        self.client
            .put(
                &format!("/experiences/{}/description", update.id),
                &serde_json::json!({ "description": update.description }),
            )
            .await
    }

    pub async fn update_sensation(
        &self,
        update: &UpdateExperienceSensation,
    ) -> Result<ExperienceResponse, ClientError> {
        let UpdateExperienceSensation::V1(update) = update;
        self.client
            .put(
                &format!("/experiences/{}/sensation", update.id),
                &serde_json::json!({ "sensation": update.sensation }),
            )
            .await
    }
}
