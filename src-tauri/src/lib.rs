use keyring_core::Entry;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

const STORE_NAME: &str = "store.json";

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
pub enum APIResponse {
    Success(KeyInfoRoot),
    Error(APIError),
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring_core::Error),

    #[error("Store error: {0}")]
    Store(#[from] tauri_plugin_store::Error),

    #[error("Insufficient API key access level (need ≥ 3, got {0})")]
    InsufficientAccess(u8),

    #[error("Torn API error: {0}")]
    TornApi(String),
}

// Conversion at the FFI boundary only
impl From<AppError> for String {
    fn from(e: AppError) -> Self {
        e.to_string()
    }
}

fn torn_keyring(app: &AppHandle) -> Result<Entry, AppError> {
    Entry::new(&app.config().identifier, "torn_api_key").map_err(AppError::Keyring)
}

async fn validate_key(app: &AppHandle, api_key: &str) -> Result<User, AppError> {
    let json = app
        .state::<reqwest::Client>()
        .get("https://api.torn.com/v2/key/info")
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("ApiKey {}", api_key))
        .send()
        .await?
        .json::<APIResponse>()
        .await?;

    match json {
        APIResponse::Success(root) => {
            // Make sure the api key given has the minimum access required: Limited Access (3)
            if root.info.access.level < 3 {
                Err(AppError::InsufficientAccess(root.info.access.level))
            } else {
                Ok(root.info.user)
            }
        }
        // If there's an error return the error result
        APIResponse::Error(err) => Err(AppError::TornApi(err.error.error)),
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn validate_and_save_key(app: AppHandle, api_key: &str) -> Result<String, String> {
    let user = validate_key(&app, api_key).await?;
    let entry = torn_keyring(&app)?;
    entry.set_password(api_key).map_err(AppError::Keyring)?;
    let store = app.store(STORE_NAME).map_err(AppError::Store)?;

    store.set("user_info", serde_json::to_value(&user).unwrap());
    Ok(format!("User set: {}", user.id))
}

#[tauri::command]
fn log_out(app: AppHandle) -> Result<String, String> {
    let entry = torn_keyring(&app)?;
    match entry.delete_credential() {
        Ok(_) => Ok("The credential has been deleted".to_string()),
        Err(keyring_core::error::Error::NoEntry) => {
            Ok("No key found, already logged out".to_string())
        }
        Err(err) => Err(AppError::Keyring(err).to_string()),
    }
}

#[tauri::command]
async fn authenticate_from_keyring(app: AppHandle) -> Result<String, String> {
    let entry = torn_keyring(&app)?;
    let api_key = entry.get_password().map_err(AppError::Keyring)?;
    let user = validate_key(&app, &api_key).await?;
    let store = app.store(STORE_NAME).map_err(AppError::Store)?;

    store.set("user_info", serde_json::to_value(&user).unwrap());
    Ok("User validated and updated".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            app.manage(reqwest::Client::new());
            keyring_core::set_default_store(windows_native_keyring_store::Store::new().unwrap());

            let store = app.store(STORE_NAME)?;

            store.close_resource();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            validate_and_save_key,
            authenticate_from_keyring,
            log_out
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
