use oneiros_model::{Level, LevelName, Persona, PersonaName, Texture, TextureName, Token};
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

    pub async fn set_persona(&self, token: &Token, request: Persona) -> Result<Persona, Error> {
        let body = serde_json::to_vec(&request)?;
        let (status, response_body) = self
            .client
            .authenticated_request("PUT", "/personas", token, Some(body))
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

    pub async fn remove_persona(&self, token: &Token, name: &PersonaName) -> Result<(), Error> {
        let uri = format!("/personas/{name}");
        let (status, response_body) = self
            .client
            .authenticated_request("DELETE", &uri, token, None)
            .await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(())
    }

    pub async fn get_persona(&self, token: &Token, name: &PersonaName) -> Result<Persona, Error> {
        let uri = format!("/personas/{name}");
        let (status, response_body) = self
            .client
            .authenticated_request("GET", &uri, token, None)
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

    pub async fn list_personas(&self, token: &Token) -> Result<Vec<Persona>, Error> {
        let (status, response_body) = self
            .client
            .authenticated_request("GET", "/personas", token, None)
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

    pub async fn set_texture(&self, token: &Token, request: Texture) -> Result<Texture, Error> {
        let body = serde_json::to_vec(&request)?;
        let (status, response_body) = self
            .client
            .authenticated_request("PUT", "/textures", token, Some(body))
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

    pub async fn remove_texture(&self, token: &Token, name: &TextureName) -> Result<(), Error> {
        let uri = format!("/textures/{name}");
        let (status, response_body) = self
            .client
            .authenticated_request("DELETE", &uri, token, None)
            .await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(())
    }

    pub async fn get_texture(&self, token: &Token, name: &TextureName) -> Result<Texture, Error> {
        let uri = format!("/textures/{name}");
        let (status, response_body) = self
            .client
            .authenticated_request("GET", &uri, token, None)
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

    pub async fn list_textures(&self, token: &Token) -> Result<Vec<Texture>, Error> {
        let (status, response_body) = self
            .client
            .authenticated_request("GET", "/textures", token, None)
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

    pub async fn set_level(&self, token: &Token, request: Level) -> Result<Level, Error> {
        let body = serde_json::to_vec(&request)?;
        let (status, response_body) = self
            .client
            .authenticated_request("PUT", "/levels", token, Some(body))
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

    pub async fn remove_level(&self, token: &Token, name: &LevelName) -> Result<(), Error> {
        let uri = format!("/levels/{name}");
        let (status, response_body) = self
            .client
            .authenticated_request("DELETE", &uri, token, None)
            .await?;

        if status >= 400 {
            let body_str = String::from_utf8_lossy(&response_body).to_string();
            return Err(ResponseError {
                status,
                body: body_str,
            })?;
        }

        Ok(())
    }

    pub async fn get_level(&self, token: &Token, name: &LevelName) -> Result<Level, Error> {
        let uri = format!("/levels/{name}");
        let (status, response_body) = self
            .client
            .authenticated_request("GET", &uri, token, None)
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

    pub async fn list_levels(&self, token: &Token) -> Result<Vec<Level>, Error> {
        let (status, response_body) = self
            .client
            .authenticated_request("GET", "/levels", token, None)
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
