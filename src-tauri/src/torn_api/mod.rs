use serde::{Deserialize, Serialize};
use thiserror::Error;

mod key;
mod user;

pub use user::UserData;

#[derive(Debug, Deserialize, Serialize)]
pub struct APIError {
    pub error: Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: u8,
    pub error: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum APIResponse<T> {
    Success(T),
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
}
