use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use crate::torn_api::{APIResponse, TornError};

use super::TornClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfoRoot {
    pub info: Info,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub access: Access,
    pub user: KeyUser,
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
#[serde(rename = "user")]
pub struct KeyUser {
    pub id: u32,
    pub faction_id: u32,
    pub company_id: u32,
}

impl TornClient {
    pub async fn get_key_info(&self) -> Result<KeyInfoRoot, TornError> {
        let response = self
            .http_client
            .get("https://api.torn.com/v2/key/info")
            .header(AUTHORIZATION, format!("ApiKey {}", self.api_key))
            .send()
            .await?
            .json::<APIResponse<KeyInfoRoot>>()
            .await?;

        match response {
            APIResponse::Success(key_info) => Ok(key_info),
            APIResponse::Error(err) => Err(TornError::TornApi(err.error.error)),
        }
    }
}
