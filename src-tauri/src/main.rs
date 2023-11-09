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
            msg: format!("{:#}", e),
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
        channel_list: service::channel_service::get_channel_list()?,
    })
}

#[tauri::command]
fn get_feed_list(channel_id: &str) -> InvokeResult<GetFeedListResponse> {
    Ok(GetFeedListResponse {
        feed_list: service::feed_service::get_feed_list(channel_id)?,
    })
}

#[tauri::command]
fn create_channel(link: &str) -> InvokeResult<CreateChannelResponse> {
    Ok(CreateChannelResponse {
        channel_id: "id".to_string(),
    })
}

#[tauri::command]
fn refresh_all_channel() -> InvokeResult<()> {
    Ok(())
}

// #[tauri::command]
// fn get_setting() -> InvokeResult<GetSettingResponse> {
//     Ok(GetSettingResponse {
//         setting: build_setting(),
//     })
// }

// #[tauri::command]
// fn update_setting() -> InvokeResult<UpdateSettingResponse> {
//     Ok(UpdateSettingResponse {
//         setting: build_setting(),
//     })
// }

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
            get_feed_list,
            create_channel,
            refresh_all_channel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
