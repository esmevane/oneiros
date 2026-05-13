use crate::*;

pub(crate) struct ProjectClient<'a> {
    client: &'a Client,
}

impl<'a> ProjectClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn create(
        &self,
        request: &CreateProject,
    ) -> Result<ProjectResponse, ClientError> {
        self.client.post("/projects", request).await
    }

    pub(crate) async fn get(&self, lookup: &GetProject) -> Result<ProjectResponse, ClientError> {
        let GetProject::V1(lookup) = lookup;
        self.client.get(&format!("/projects/{}", lookup.key)).await
    }

    pub(crate) async fn list(
        &self,
        listing: &ListProjects,
    ) -> Result<ProjectResponse, ClientError> {
        let ListProjects::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/projects?{query}")).await
    }
}
