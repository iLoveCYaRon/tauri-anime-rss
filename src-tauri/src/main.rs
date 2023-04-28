// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;
mod service;
use self::model::*;

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;
pub(crate) type InvokeResult<T> = std::result::Result<T, InvokeError>;
impl From<anyhow::Error> for InvokeError {
    fn from(e: anyhow::Error) -> Self {
        InvokeError { 
            code: None,
            msg: format!("{:#}", e)
        }
    }
}

/* Command Field Start */
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_channel_list() -> InvokeResult<GetChannelListResponse> {
    Ok(GetChannelListResponse {
        channel_list: service::rss_service::get_channel_list()?
    })
}

#[tauri::command]
fn get_channel_detail(channel_id: &str) -> InvokeResult<GetChannelDetailResponse> {
    Ok(GetChannelDetailResponse {
        item_list: service::rss_service::get_channel_detail(channel_id)?,
    })
}

#[tauri::command]
fn create_channel(link: &str) -> InvokeResult<CreateChannelResponse> {
    Ok(CreateChannelResponse {
        channel_id: "id".to_string(),
    })
}

#[tauri::command]
fn get_setting() -> InvokeResult<GetSettingResponse> {
    Ok(GetSettingResponse {
        setting: build_setting(),
    })
}

#[tauri::command]
fn update_setting() -> InvokeResult<UpdateSettingResponse> {
    Ok(UpdateSettingResponse {
        setting: build_setting(),
    })
}

/* Command Field End */

fn build_setting() -> Setting {
    Setting {
        network_proxy: NetworkProxy {
            proxy_type: ProxyType::Https,
            address: "127.0.0.1".to_string(),
            port: 8080,
            username: Some("username".to_string()),
            password: None,
        },
        qbit_config: QBitConfig {
            address: "127.0.0.1".to_string(),
            port: 8080,
            username: None,
            password: Some("password".to_string()),
        },
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_channel_list,
            get_channel_detail,
            create_channel,
            get_setting,
            update_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
