use tauri::AppHandle;
use tauri::{plugin::PermissionState, Window};
use anyhow::Result;
use tauri_plugin_blec::check_permissions;
use tauri_plugin_geolocation::{GeolocationExt, PermissionType, PositionOptions};
use tauri_plugin_shell::process::Command;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn connect_hk203(app: AppHandle) -> Result<(), String> {
    bluetooth_internal(app).await.map_err(|e| e.to_string() )
}

async fn bluetooth_internal(app: AppHandle) -> anyhow::Result<()> {
    check_permissions()?;

    let handler = tauri_plugin_blec::get_handler()?;
    handler.connect("7A:75:F0:F5:31:F3", (|| println!("disconnected")).into()).await?;

    let cmd = app.shell();
    cmd.command("input keyevent 126");
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_current_position(window: Window) -> Result<tauri_plugin_geolocation::Position, String> {
    get_position_internal(window).await.map_err(|e| e.to_string() )
}

async fn get_position_internal(window: Window) -> anyhow::Result<tauri_plugin_geolocation::Position> {
    let location = window.geolocation();
    let mut perm = location.check_permissions()?;
    loop {
        perm = match perm.location {
            PermissionState::Granted => break,
            PermissionState::Denied => continue,
            PermissionState::Prompt => location.request_permissions(Some(vec![PermissionType::Location]))?,
            PermissionState::PromptWithRationale => location.request_permissions(Some(vec![PermissionType::Location]))?
        };
    }
    Ok(location.get_current_position(Some(PositionOptions {
        enable_high_accuracy: true,
        timeout: 10000,
        maximum_age: 0
    }))?)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()                                                                                                                   
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_current_position, connect_hk203])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
