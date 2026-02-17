use oneiros_model::{
    Agent, AgentName, Cognition, CognitionId, DreamContext, Experience, ExperienceId, Level,
    LevelName, Memory, MemoryId, Persona, PersonaName, Sensation, SensationName, StorageEntry,
    StorageKey, StorageRef, Texture, TextureName, Token,
};
use std::path::Path;

use crate::*;

pub struct Client {
    client: SocketClient,
}

impl Client {
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            client: SocketClient::new(socket_path),
        }
    }

    async fn send(
        &self,
        method: &str,
        uri: &str,
        token: &Token,
        body: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, Error> {
        let (status, response_body) = self
            .client
            .authenticated_request(method, uri, token, body)
            .await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(response_body)
    }

    pub async fn create_brain(&self, request: CreateBrainRequest) -> Result<BrainInfo, Error> {
        let body = serde_json::to_vec(&request)?;
        let (status, response_body) = self.client.request("POST", "/brains", body).await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(serde_json::from_slice(&response_body)?)
    }

    pub async fn create_agent(
        &self,
        token: &Token,
        request: CreateAgentRequest,
    ) -> Result<Agent, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/agents", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn update_agent(
        &self,
        token: &Token,
        name: &AgentName,
        request: UpdateAgentRequest,
    ) -> Result<Agent, Error> {
        let uri = format!("/agents/{name}");
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", &uri, token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn remove_agent(&self, token: &Token, name: &AgentName) -> Result<(), Error> {
        let uri = format!("/agents/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_agent(&self, token: &Token, name: &AgentName) -> Result<Agent, Error> {
        let uri = format!("/agents/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_agents(&self, token: &Token) -> Result<Vec<Agent>, Error> {
        let bytes = self.send("GET", "/agents", token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn add_cognition(
        &self,
        token: &Token,
        request: AddCognitionRequest,
    ) -> Result<Cognition, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/cognitions", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get_cognition(&self, token: &Token, id: &CognitionId) -> Result<Cognition, Error> {
        let uri = format!("/cognitions/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_cognitions(
        &self,
        token: &Token,
        agent: Option<&AgentName>,
        texture: Option<&TextureName>,
    ) -> Result<Vec<Cognition>, Error> {
        let mut params = Vec::new();
        if let Some(agent) = agent {
            params.push(format!("agent={agent}"));
        }
        if let Some(texture) = texture {
            params.push(format!("texture={texture}"));
        }

        let uri = if params.is_empty() {
            "/cognitions".to_string()
        } else {
            format!("/cognitions?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn add_memory(
        &self,
        token: &Token,
        request: AddMemoryRequest,
    ) -> Result<Memory, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/memories", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get_memory(&self, token: &Token, id: &MemoryId) -> Result<Memory, Error> {
        let uri = format!("/memories/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_memories(
        &self,
        token: &Token,
        agent: Option<&AgentName>,
        level: Option<&LevelName>,
    ) -> Result<Vec<Memory>, Error> {
        let mut params = Vec::new();
        if let Some(agent) = agent {
            params.push(format!("agent={agent}"));
        }
        if let Some(level) = level {
            params.push(format!("level={level}"));
        }

        let uri = if params.is_empty() {
            "/memories".to_string()
        } else {
            format!("/memories?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn set_persona(&self, token: &Token, request: Persona) -> Result<Persona, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/personas", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn remove_persona(&self, token: &Token, name: &PersonaName) -> Result<(), Error> {
        let uri = format!("/personas/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_persona(&self, token: &Token, name: &PersonaName) -> Result<Persona, Error> {
        let uri = format!("/personas/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_personas(&self, token: &Token) -> Result<Vec<Persona>, Error> {
        let bytes = self.send("GET", "/personas", token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn set_texture(&self, token: &Token, request: Texture) -> Result<Texture, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/textures", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn remove_texture(&self, token: &Token, name: &TextureName) -> Result<(), Error> {
        let uri = format!("/textures/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_texture(&self, token: &Token, name: &TextureName) -> Result<Texture, Error> {
        let uri = format!("/textures/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_textures(&self, token: &Token) -> Result<Vec<Texture>, Error> {
        let bytes = self.send("GET", "/textures", token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn set_level(&self, token: &Token, request: Level) -> Result<Level, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/levels", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn remove_level(&self, token: &Token, name: &LevelName) -> Result<(), Error> {
        let uri = format!("/levels/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_level(&self, token: &Token, name: &LevelName) -> Result<Level, Error> {
        let uri = format!("/levels/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_levels(&self, token: &Token) -> Result<Vec<Level>, Error> {
        let bytes = self.send("GET", "/levels", token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn set_storage(
        &self,
        token: &Token,
        key: &StorageKey,
        data: Vec<u8>,
        description: &str,
    ) -> Result<StorageEntry, Error> {
        let storage_ref = StorageRef::encode(key);
        let uri = format!("/storage/{storage_ref}");
        let headers = vec![("x-storage-description", description)];
        let (status, response_body) = self
            .client
            .authenticated_binary_request("PUT", &uri, token, data, &headers)
            .await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(serde_json::from_slice(&response_body)?)
    }

    pub async fn get_storage(
        &self,
        token: &Token,
        key: &StorageKey,
    ) -> Result<StorageEntry, Error> {
        let storage_ref = StorageRef::encode(key);
        let uri = format!("/storage/{storage_ref}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get_storage_content(
        &self,
        token: &Token,
        key: &StorageKey,
    ) -> Result<Vec<u8>, Error> {
        let storage_ref = StorageRef::encode(key);
        let uri = format!("/storage/{storage_ref}/content");
        self.send("GET", &uri, token, None).await
    }

    pub async fn list_storage(&self, token: &Token) -> Result<Vec<StorageEntry>, Error> {
        let bytes = self.send("GET", "/storage", token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn remove_storage(&self, token: &Token, key: &StorageKey) -> Result<(), Error> {
        let storage_ref = StorageRef::encode(key);
        let uri = format!("/storage/{storage_ref}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn dream(
        &self,
        token: &Token,
        agent_name: &AgentName,
    ) -> Result<DreamContext, Error> {
        let uri = format!("/dream/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn introspect(&self, token: &Token, agent_name: &AgentName) -> Result<Agent, Error> {
        let uri = format!("/introspect/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    // -- Sensation methods --

    pub async fn set_sensation(
        &self,
        token: &Token,
        request: Sensation,
    ) -> Result<Sensation, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/sensations", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn remove_sensation(&self, token: &Token, name: &SensationName) -> Result<(), Error> {
        let uri = format!("/sensations/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_sensation(
        &self,
        token: &Token,
        name: &SensationName,
    ) -> Result<Sensation, Error> {
        let uri = format!("/sensations/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_sensations(&self, token: &Token) -> Result<Vec<Sensation>, Error> {
        let bytes = self.send("GET", "/sensations", token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    // -- Experience methods --

    pub async fn create_experience(
        &self,
        token: &Token,
        request: CreateExperienceRequest,
    ) -> Result<Experience, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/experiences", token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get_experience(
        &self,
        token: &Token,
        id: &ExperienceId,
    ) -> Result<Experience, Error> {
        let uri = format!("/experiences/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn list_experiences(
        &self,
        token: &Token,
        agent: Option<&AgentName>,
        sensation: Option<&SensationName>,
    ) -> Result<Vec<Experience>, Error> {
        let mut params = Vec::new();
        if let Some(agent) = agent {
            params.push(format!("agent={agent}"));
        }
        if let Some(sensation) = sensation {
            params.push(format!("sensation={sensation}"));
        }

        let uri = if params.is_empty() {
            "/experiences".to_string()
        } else {
            format!("/experiences?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn add_experience_ref(
        &self,
        token: &Token,
        experience_id: &ExperienceId,
        request: AddExperienceRefRequest,
    ) -> Result<Experience, Error> {
        let uri = format!("/experiences/{experience_id}/refs");
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", &uri, token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn update_experience_description(
        &self,
        token: &Token,
        experience_id: &ExperienceId,
        request: UpdateExperienceDescriptionRequest,
    ) -> Result<Experience, Error> {
        let uri = format!("/experiences/{experience_id}/description");
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", &uri, token, Some(body)).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn reflect(&self, token: &Token, agent_name: &AgentName) -> Result<Agent, Error> {
        let uri = format!("/reflect/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn health(&self) -> Result<(), Error> {
        let (status, response_body) = self.client.request("GET", "/health", vec![]).await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(())
    }
}
