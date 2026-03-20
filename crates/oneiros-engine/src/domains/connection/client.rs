use crate::*;

pub struct ConnectionClient<'a> {
    client: &'a Client,
}

impl<'a> ConnectionClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        from_ref: Ref,
        to_ref: Ref,
        nature: NatureName,
        description: Description,
    ) -> Result<ConnectionResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            from_ref: Ref,
            to_ref: Ref,
            nature: NatureName,
            description: Description,
        }

        self.client
            .post(
                "/connections",
                &Body {
                    from_ref,
                    to_ref,
                    nature,
                    description,
                },
            )
            .await
    }

    pub async fn list(&self, entity: Option<&Ref>) -> Result<ConnectionResponse, ClientError> {
        let path = match entity {
            Some(e) => format!("/connections?entity={e}"),
            None => "/connections".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &ConnectionId) -> Result<ConnectionResponse, ClientError> {
        self.client.get(&format!("/connections/{id}")).await
    }

    pub async fn remove(&self, id: &ConnectionId) -> Result<ConnectionResponse, ClientError> {
        self.client.delete(&format!("/connections/{id}")).await
    }
}
