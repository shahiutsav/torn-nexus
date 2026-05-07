use crate::torn_api::{APIResponse, TornError};

use super::TornClient;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub bars: Bars,
    pub cooldowns: Cooldowns,
    pub profile: Profile,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bars {
    pub energy: Bar,
    pub nerve: Bar,
    pub happy: Bar,
    pub life: Bar,
    pub chain: Chain,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bar {
    pub current: u32,
    pub maximum: u32,
    pub increment: u16,
    pub interval: u16,
    pub tick_time: u16,
    pub full_time: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chain {
    pub id: u32,
    pub current: u32,
    pub max: u32,
    pub timeout: u16,
    pub modifier: u8,
    pub cooldown: u32,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cooldowns {
    pub drug: u32,
    pub medical: u32,
    pub booster: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub level: u8,
    pub rank: String,
    pub title: String,
    pub age: u32,
    pub faction_id: u32,
    pub honor_id: u32,
    pub donator_status: String,
    pub image: String,
    pub gender: String,
    pub revivable: bool,
    pub role: String,
    pub status: Status,
    pub spouse: Spouse,
    pub awards: u16,
    pub friends: u32,
    pub enemies: u32,
    pub forum_posts: u32,
    pub karma: u32,
    pub last_action: LastAction,
    pub life: Life,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Status {
    pub description: String,
    pub details: Value,
    pub state: String,
    pub color: String,
    pub until: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Spouse {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub days_married: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LastAction {
    pub status: String,
    pub timestamp: u32,
    pub relative: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Life {
    pub current: u32,
    pub maximum: u32,
}

impl TornClient {
    pub async fn get_user_data(&self) -> Result<UserData, TornError> {
        let response = self
            .http_client
            .get("https://api.torn.com/v2/user?selections=bars,cooldowns,profile")
            .header(AUTHORIZATION, format!("ApiKey {}", self.api_key))
            .send()
            .await?
            .json::<APIResponse<UserData>>()
            .await?;

        match response {
            APIResponse::Success(user_data) => Ok(user_data),
            APIResponse::Error(err) => Err(TornError::TornApi(err.error.error)),
        }
    }
}
