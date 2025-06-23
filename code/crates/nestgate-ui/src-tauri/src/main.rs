// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn execute_zfs_command(command: String) -> Result<String, String> {
    // This would integrate with the ZFS backend
    // For now, return a mock response
    Ok(format!("Executed ZFS command: {}", command))
}

#[tauri::command]
async fn get_system_info() -> Result<serde_json::Value, String> {
    // This would get real system information
    // For now, return mock data
    let info = serde_json::json!({
        "hostname": "nestgate-system",
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "version": "0.1.0"
    });
    Ok(info)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            execute_zfs_command,
            get_system_info
        ])
        .setup(|app| {
            // Additional setup can go here
            println!("NestGate Desktop Application Starting...");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 