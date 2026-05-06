use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct APIError {
    pub error: Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: u8,
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfoRoot {
    pub info: Info,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub access: Access,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Access {
    pub level: u8,
    #[serde(rename = "type")]
    pub type_: String,
    pub faction: bool,
    pub company: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub faction_id: u32,
    pub company_id: u32,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum APIResponse {
    Success(KeyInfoRoot),
    Error(APIError),
}

#[derive(Error, Debug)]
pub enum TornError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {0}")]
    TornApi(String),
}

pub struct TornClient {
    api_key: String,
    http_client: reqwest::Client,
}

impl TornClient {
    pub fn new(api_key: String) -> Self {
        TornClient {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_key_info(&self) -> Result<KeyInfoRoot, TornError> {
        let response = self
            .http_client
            .get("https://api.torn.com/v2/key/info")
            .header(AUTHORIZATION, format!("ApiKey {}", self.api_key))
            .send()
            .await?
            .json::<APIResponse>()
            .await?;

        match response {
            APIResponse::Success(key_info) => Ok(key_info),
            APIResponse::Error(err) => Err(TornError::TornApi(err.error.error)),
        }
    }
}
