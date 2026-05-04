use keyring_core::Entry;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn validate_and_save_key(app: AppHandle, api_key: &str) -> Result<String, String> {
    // Call the api to collect key info
    let url = "https://api.torn.com/v2/key/info";
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("ApiKey {}", api_key))
        .send()
        .await
        .map_err(|e| format!("An error occurred when making the request. {e}"))?;

    let json = res
        .json::<APIResponse>()
        .await
        .map_err(|e| format!("An error occurred when parsing the json data. {e}"))?;

    match json {
        APIResponse::Success(key_info_root) => {
            // Make sure the api key given has the minimum access required: Limited Access (3)
            if key_info_root.info.access.level < 3 {
                Err("The api key should have at least Limited Access Level".to_string())
            } else {
                // Every check has passed, thus save the key to the OS and other preference to the store for persistence
                let entry = Entry::new(&app.config().identifier, "torn_api_key")
                    .map_err(|e| format!("Error accessing OS secure storage. {e}"))?;
                let user = key_info_root.info.user;
                entry
                    .set_password(api_key)
                    .map_err(|e| format!("Error storing the api key. {e}"))?;
                let store = app
                    .store("store.json")
                    .map_err(|e| format!("Error accessing the store. {e}"))?;

                store.set("user_info", serde_json::to_value(user).unwrap());
                Ok("User set and ready".to_string())
            }
        }
        // If there's an error return the error result
        APIResponse::Error(err) => Err(format!(
            "An error occurred from the torn's api\nMore Info\n{}",
            err.error.error
        )),
    }
}

#[tauri::command]
async fn log_out(app: AppHandle) -> Result<String, String> {
    let entry = Entry::new(&app.config().identifier, "torn_api_key")
        .map_err(|e| format!("Error accessing OS secure storage. {e}"))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Error deleting credentials. {e}"))?;
    Ok("The credential has been deleted".to_string())
}

async fn authenticate_from_keyring(app: AppHandle) -> Result<String, String> {
    let entry = Entry::new(&app.config().identifier, "torn_api_key")
        .map_err(|e| format!("Error accessing OS secure storage. {e}"))?;
    let api_key = entry
        .get_password()
        .map_err(|e| format!("Error getting the api key from the storage. {e}"))?;
    validate_and_save_key(app, &api_key)
        .await
        .map_err(|e| format!("Error validating and updating data. {e}"))?;
    Ok("You are successfully authenticated".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            keyring_core::set_default_store(windows_native_keyring_store::Store::new().unwrap());
            let window = app.get_webview_window("main").unwrap();

            let store = app.store("store.json")?;

            store.close_resource();
            match tauri::async_runtime::block_on(authenticate_from_keyring(app.handle().clone())) {
                Ok(_) => {
                    window.eval("window.location.replace('/')").unwrap();
                }
                Err(_) => {
                    window.eval("window.location.replace('/setup')").unwrap();
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![validate_and_save_key, log_out])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
