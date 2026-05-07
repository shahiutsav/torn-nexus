use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;
use tokio::time::{sleep_until, Duration, Instant};

use crate::{keyring::get_key, torn_api::TornClient};

mod keyring;
mod torn_api;

const STORE_NAME: &str = "store.json";
const TORN_USER: &str = "torn_api_key";

// TODO: look at how store can be managed neatly as well

async fn poll_and_emit(app: AppHandle) {
    loop {
        // Calculate ms until next 30s mark
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let secs = now.as_secs();
        let next_mark = ((secs / 30) + 1) * 30;

        let wait_secs = next_mark - secs;
        let target = Instant::now() + Duration::from_secs(wait_secs);

        sleep_until(target).await;

        let key = {
            let state = app.state::<Mutex<Option<String>>>();
            let guard = state.lock().unwrap();
            guard.clone()
        };

        match key {
            Some(key) => {
                let client = TornClient::new(key);
                match client.get_user_data().await {
                    Ok(data) => app.emit("data-updated", data).unwrap(),
                    Err(_) => continue,
                }
            }
            None => continue,
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn connect_torn(app: AppHandle, api_key: String) -> Result<(), String> {
    let client = TornClient::new(api_key.to_string());
    let result = client.get_key_info().await.map_err(|e| e.to_string())?;

    if result.info.access.level < 3 {
        Err("API access level not enough".to_string())
    } else {
        let user = result.info.user;
        keyring::set_key(TORN_USER, &api_key).map_err(|e| e.to_string())?;

        let state = app.state::<Mutex<Option<String>>>();
        *state.lock().unwrap() = Some(api_key);
        let store = app.store(STORE_NAME).map_err(|e| e.to_string())?;

        store.set("user_info", serde_json::to_value(&user).unwrap());

        Ok(())
    }
}

#[tauri::command]
fn log_out() -> Result<(), String> {
    keyring::delete_key(TORN_USER).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn verify_session(app: AppHandle) -> Result<bool, String> {
    let api_key = get_key(TORN_USER).map_err(|e| e.to_string())?;
    match api_key {
        Some(value) => {
            let client = TornClient::new(value.to_string());
            let result = client.get_key_info().await.map_err(|e| e.to_string())?;

            if result.info.access.level < 3 {
                Err("API access level not enough".to_string())
            } else {
                let user = result.info.user;

                let state = app.state::<Mutex<Option<String>>>();
                *state.lock().unwrap() = Some(value);
                let store = app.store(STORE_NAME).map_err(|e| e.to_string())?;

                store.set("user_info", serde_json::to_value(&user).unwrap());
                let store = app.store(STORE_NAME).map_err(|e| e.to_string())?;

                store.set("user_info", serde_json::to_value(&user).unwrap());
                Ok(true)
            }
        }
        None => Ok(false),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            keyring_core::set_default_store(windows_native_keyring_store::Store::new().unwrap());

            let initial_key = match keyring::get_key(TORN_USER) {
                Ok(Some(key)) => Some(key),
                _ => None,
            };
            app.manage(Mutex::new(initial_key));

            let store = app.store(STORE_NAME)?;

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                poll_and_emit(handle).await;
            });

            store.close_resource();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            connect_torn,
            verify_session,
            log_out
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
