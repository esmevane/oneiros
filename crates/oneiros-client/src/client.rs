use oneiros_model::*;
use std::net::SocketAddr;

use crate::*;

pub struct Client {
    client: SocketClient,
}

impl Client {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            client: SocketClient::new(addr),
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

    /// Parse a JSON response as a full Response envelope.
    fn parse(&self, bytes: &[u8]) -> Result<Response, Error> {
        Ok(serde_json::from_slice(bytes)?)
    }

    pub async fn get_event(&self, token: &Token, name: &EventId) -> Result<Response, Error> {
        let uri = format!("/events/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_events(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/events", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn create_brain(&self, request: CreateBrainRequest) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let (status, response_body) = self.client.request("POST", "/brains", body).await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        self.parse(&response_body)
    }

    pub async fn export_brain(&self, token: &Token) -> Result<Response, Error> {
        self.list_events(token).await
    }

    pub async fn import_events(
        &self,
        token: &Token,
        events: Vec<ImportEvent>,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&events)?;
        let bytes = self
            .send("POST", "/events/import", token, Some(body))
            .await?;
        self.parse(&bytes)
    }

    pub async fn replay_brain(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("POST", "/events/replay", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn create_agent(
        &self,
        token: &Token,
        request: CreateAgentRequest,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/agents", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn update_agent(
        &self,
        token: &Token,
        name: &AgentName,
        request: UpdateAgentRequest,
    ) -> Result<Response, Error> {
        let uri = format!("/agents/{name}");
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", &uri, token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn remove_agent(&self, token: &Token, name: &AgentName) -> Result<(), Error> {
        let uri = format!("/agents/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_agent(&self, token: &Token, name: &AgentName) -> Result<Response, Error> {
        let uri = format!("/agents/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_agents(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/agents", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn add_cognition(
        &self,
        token: &Token,
        request: AddCognitionRequest,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/cognitions", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn get_cognition(&self, token: &Token, id: &CognitionId) -> Result<Response, Error> {
        let uri = format!("/cognitions/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_cognitions(
        &self,
        token: &Token,
        agent: Option<&AgentName>,
        texture: Option<&TextureName>,
    ) -> Result<Response, Error> {
        let mut params = Vec::new();
        if let Some(agent) = agent {
            params.push(format!("agent={}", urlencoding::encode(agent.as_str())));
        }
        if let Some(texture) = texture {
            params.push(format!("texture={}", urlencoding::encode(texture.as_str())));
        }

        let uri = if params.is_empty() {
            "/cognitions".to_string()
        } else {
            format!("/cognitions?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn add_memory(
        &self,
        token: &Token,
        request: AddMemoryRequest,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/memories", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn get_memory(&self, token: &Token, id: &MemoryId) -> Result<Response, Error> {
        let uri = format!("/memories/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_memories(
        &self,
        token: &Token,
        agent: Option<&AgentName>,
        level: Option<&LevelName>,
    ) -> Result<Response, Error> {
        let mut params = Vec::new();
        if let Some(agent) = agent {
            params.push(format!("agent={}", urlencoding::encode(agent.as_str())));
        }
        if let Some(level) = level {
            params.push(format!("level={}", urlencoding::encode(level.as_str())));
        }

        let uri = if params.is_empty() {
            "/memories".to_string()
        } else {
            format!("/memories?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn set_persona(&self, token: &Token, request: Persona) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/personas", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn remove_persona(&self, token: &Token, name: &PersonaName) -> Result<(), Error> {
        let uri = format!("/personas/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_persona(&self, token: &Token, name: &PersonaName) -> Result<Response, Error> {
        let uri = format!("/personas/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_personas(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/personas", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn set_texture(&self, token: &Token, request: Texture) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/textures", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn remove_texture(&self, token: &Token, name: &TextureName) -> Result<(), Error> {
        let uri = format!("/textures/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_texture(&self, token: &Token, name: &TextureName) -> Result<Response, Error> {
        let uri = format!("/textures/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_textures(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/textures", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn set_urge(&self, token: &Token, request: Urge) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/urges", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn remove_urge(&self, token: &Token, name: &UrgeName) -> Result<(), Error> {
        let uri = format!("/urges/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_urge(&self, token: &Token, name: &UrgeName) -> Result<Response, Error> {
        let uri = format!("/urges/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_urges(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/urges", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn get_pressure(&self, token: &Token, agent: &AgentName) -> Result<Response, Error> {
        self.parse(
            &self
                .send("GET", &format!("/pressures/{agent}"), token, None)
                .await?,
        )
    }

    pub async fn list_pressures(&self, token: &Token) -> Result<Response, Error> {
        self.parse(&self.send("GET", "/pressures", token, None).await?)
    }

    pub async fn set_level(&self, token: &Token, request: Level) -> Result<Response, Error> {
        self.parse(
            &self
                .send("PUT", "/levels", token, Some(serde_json::to_vec(&request)?))
                .await?,
        )
    }

    pub async fn remove_level(&self, token: &Token, name: &LevelName) -> Result<(), Error> {
        let uri = format!("/levels/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_level(&self, token: &Token, name: &LevelName) -> Result<Response, Error> {
        let uri = format!("/levels/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_levels(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/levels", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn set_storage(
        &self,
        token: &Token,
        key: &StorageKey,
        data: Vec<u8>,
        description: &str,
    ) -> Result<Response, Error> {
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

        self.parse(&response_body)
    }

    pub async fn get_storage(&self, token: &Token, key: &StorageKey) -> Result<Response, Error> {
        let storage_ref = StorageRef::encode(key);
        let uri = format!("/storage/{storage_ref}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
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

    pub async fn list_storage(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/storage", token, None).await?;
        self.parse(&bytes)
    }

    pub async fn remove_storage(&self, token: &Token, key: &StorageKey) -> Result<(), Error> {
        let storage_ref = StorageRef::encode(key);
        let uri = format!("/storage/{storage_ref}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn dream(&self, token: &Token, agent_name: &AgentName) -> Result<Response, Error> {
        let uri = format!("/dream/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn introspect(
        &self,
        token: &Token,
        agent_name: &AgentName,
    ) -> Result<Response, Error> {
        let uri = format!("/introspect/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        self.parse(&bytes)
    }

    // -- Sensation methods --

    pub async fn set_sensation(
        &self,
        token: &Token,
        request: Sensation,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/sensations", token, Some(body)).await?;
        self.parse(&bytes)
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
    ) -> Result<Response, Error> {
        let uri = format!("/sensations/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_sensations(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/sensations", token, None).await?;
        self.parse(&bytes)
    }

    // -- Nature methods --

    pub async fn set_nature(&self, token: &Token, request: Nature) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", "/natures", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn remove_nature(&self, token: &Token, name: &NatureName) -> Result<(), Error> {
        let uri = format!("/natures/{name}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    pub async fn get_nature(&self, token: &Token, name: &NatureName) -> Result<Response, Error> {
        let uri = format!("/natures/{name}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_natures(&self, token: &Token) -> Result<Response, Error> {
        let bytes = self.send("GET", "/natures", token, None).await?;
        self.parse(&bytes)
    }

    // -- Connection methods --

    pub async fn create_connection(
        &self,
        token: &Token,
        request: CreateConnectionRequest,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/connections", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn get_connection(
        &self,
        token: &Token,
        id: &ConnectionId,
    ) -> Result<Response, Error> {
        let uri = format!("/connections/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_connections(
        &self,
        token: &Token,
        nature: Option<&NatureName>,
        entity_ref: Option<&RefToken>,
    ) -> Result<Response, Error> {
        let mut params = Vec::new();
        if let Some(nature) = nature {
            params.push(format!("nature={}", urlencoding::encode(nature.as_str())));
        }
        if let Some(entity_ref) = entity_ref {
            params.push(format!(
                "entity_ref={}",
                urlencoding::encode(&entity_ref.to_string())
            ));
        }

        let uri = if params.is_empty() {
            "/connections".to_string()
        } else {
            format!("/connections?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn remove_connection(&self, token: &Token, id: &ConnectionId) -> Result<(), Error> {
        let uri = format!("/connections/{id}");
        self.send("DELETE", &uri, token, None).await?;
        Ok(())
    }

    // -- Experience methods --

    pub async fn create_experience(
        &self,
        token: &Token,
        request: CreateExperienceRequest,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("POST", "/experiences", token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn get_experience(
        &self,
        token: &Token,
        id: &ExperienceId,
    ) -> Result<Response, Error> {
        let uri = format!("/experiences/{id}");
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn list_experiences(
        &self,
        token: &Token,
        agent: Option<&AgentName>,
        sensation: Option<&SensationName>,
    ) -> Result<Response, Error> {
        let mut params = Vec::new();
        if let Some(agent) = agent {
            params.push(format!("agent={}", urlencoding::encode(agent.as_str())));
        }
        if let Some(sensation) = sensation {
            params.push(format!(
                "sensation={}",
                urlencoding::encode(sensation.as_str())
            ));
        }

        let uri = if params.is_empty() {
            "/experiences".to_string()
        } else {
            format!("/experiences?{}", params.join("&"))
        };

        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn update_experience_description(
        &self,
        token: &Token,
        experience_id: &ExperienceId,
        request: UpdateExperienceDescriptionRequest,
    ) -> Result<Response, Error> {
        let uri = format!("/experiences/{experience_id}/description");
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", &uri, token, Some(body)).await?;
        self.parse(&bytes)
    }

    pub async fn update_experience_sensation(
        &self,
        token: &Token,
        experience_id: &ExperienceId,
        request: UpdateExperienceSensationRequest,
    ) -> Result<Response, Error> {
        let uri = format!("/experiences/{experience_id}/sensation");
        let body = serde_json::to_vec(&request)?;
        let bytes = self.send("PUT", &uri, token, Some(body)).await?;
        self.parse(&bytes)
    }

    // -- Lifecycle methods --

    pub async fn wake(&self, token: &Token, agent_name: &AgentName) -> Result<Response, Error> {
        let uri = format!("/lifecycle/wake/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn sleep(&self, token: &Token, agent_name: &AgentName) -> Result<Response, Error> {
        let uri = format!("/lifecycle/sleep/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn emerge(
        &self,
        token: &Token,
        request: CreateAgentRequest,
    ) -> Result<Response, Error> {
        let body = serde_json::to_vec(&request)?;
        let bytes = self
            .send("POST", "/lifecycle/emerge", token, Some(body))
            .await?;
        self.parse(&bytes)
    }

    pub async fn recede(&self, token: &Token, agent_name: &AgentName) -> Result<(), Error> {
        let uri = format!("/lifecycle/recede/{agent_name}");
        self.send("POST", &uri, token, None).await?;
        Ok(())
    }

    pub async fn reflect(&self, token: &Token, agent_name: &AgentName) -> Result<Response, Error> {
        let uri = format!("/reflect/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn sense(&self, token: &Token, agent_name: &AgentName) -> Result<Response, Error> {
        let uri = format!("/sense/{agent_name}");
        let bytes = self.send("POST", &uri, token, None).await?;
        self.parse(&bytes)
    }

    pub async fn search(
        &self,
        token: &Token,
        query: &str,
        agent: Option<&AgentName>,
    ) -> Result<Response, Error> {
        let mut uri = format!("/search?query={}", urlencoding::encode(query));
        if let Some(agent) = agent {
            uri.push_str(&format!("&agent={}", urlencoding::encode(agent.as_str())));
        }
        let bytes = self.send("GET", &uri, token, None).await?;
        self.parse(&bytes)
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
