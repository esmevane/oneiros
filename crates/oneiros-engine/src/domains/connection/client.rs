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
        from_entity: impl Into<String>,
        to_entity: impl Into<String>,
        nature: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<ConnectionResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            from_entity: String,
            to_entity: String,
            nature: String,
            description: String,
        }

        self.client
            .post(
                "/connections",
                &Body {
                    from_entity: from_entity.into(),
                    to_entity: to_entity.into(),
                    nature: nature.into(),
                    description: description.into(),
                },
            )
            .await
    }

    pub async fn list(&self, entity: Option<&str>) -> Result<ConnectionResponse, ClientError> {
        let path = match entity {
            Some(e) => format!("/connections?entity={e}"),
            None => "/connections".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, id: &str) -> Result<ConnectionResponse, ClientError> {
        self.client.get(&format!("/connections/{id}")).await
    }

    pub async fn remove(&self, id: &str) -> Result<ConnectionResponse, ClientError> {
        self.client.delete(&format!("/connections/{id}")).await
    }
}
