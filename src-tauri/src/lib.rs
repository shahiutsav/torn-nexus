use keyring_core::Entry;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
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
async fn fetch_data(app: AppHandle, api_key: &str) -> Result<String, String> {
    keyring_core::set_default_store(windows_native_keyring_store::Store::new().unwrap());
    // Call the api to collect key info
    let url = "https://api.torn.com/v2/key/info";
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("ApiKey {}", api_key))
        .send()
        .await;

    // Check if the response was received
    match res {
        // If it was a success, unparse the json data to something readable
        Ok(response) => {
            let json = response.json::<APIResponse>().await;
            // Check if the json data was parsable
            match json {
                // If it was parsable, check what kind
                Ok(api_response) => match api_response {
                    // If there's usable data which is not an error
                    APIResponse::Success(key_info_root) => {
                        // Make sure the api key given has the minimum access required: Limited Access (3)
                        if key_info_root.info.access.level < 3 {
                            Err("The api key should have at least Limited Access Level".to_string())
                        } else {
                            // Every check has passed, thus save the key to the OS and other preference to the store for persistence
                            let entry = Entry::new(&app.config().identifier, "torn_api_key");
                            let user = key_info_root.info.user;
                            match entry {
                                Ok(value) => match value.set_password(api_key) {
                                    Ok(_) => {
                                        let store = app.store("store.json");
                                        match store {
                                            Ok(value) => {
                                                value.set(
                                                    "user_info",
                                                    serde_json::to_value(user).unwrap(),
                                                );
                                                Ok("User set and ready".to_string())
                                            }
                                            Err(err) => Err(format!(
                                                "Error when accessing the store\nMore Info:\n{}",
                                                err
                                            )),
                                        }
                                    }
                                    Err(err) => Err(format!(
                                        "Had problems storing the api key\nMore Details:\n{}",
                                        err
                                    )),
                                },
                                Err(err) => Err(format!("Could not create entry\n{}", err)),
                            }
                        }
                    }
                    // If there's an error return the error result
                    APIResponse::Error(err) => Err(format!(
                        "An error occurred from the torn's api\nMore Info\n{}",
                        err.error.error
                    )),
                },
                // If not return the error result
                Err(err) => Err(format!("Could not parse the json data\n{}", err)),
            }
        }
        // If it was a failure return the error result
        Err(err) => Err(format!(
            "An error occurred when making the request\n{}",
            err
        )),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("torn-nexus.json")?;
            store.close_resource();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
