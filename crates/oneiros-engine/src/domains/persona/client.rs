use crate::client::{Client, ClientError};
use crate::*;

pub struct PersonaClient<'a> {
    client: &'a Client,
}

impl<'a> PersonaClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, persona: &Persona) -> Result<PersonaResponse, ClientError> {
        self.client
            .put(&format!("/personas/{}", persona.name), persona)
            .await
    }

    pub async fn get(&self, name: &str) -> Result<PersonaResponse, ClientError> {
        self.client.get(&format!("/personas/{}", name)).await
    }

    pub async fn list(&self) -> Result<PersonaResponse, ClientError> {
        self.client.get("/personas").await
    }

    pub async fn remove(&self, name: &str) -> Result<PersonaResponse, ClientError> {
        self.client.delete(&format!("/personas/{}", name)).await
    }
}
