// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn execute_zfs_command(command: String) -> Result<String, String> {
    // Parse and validate the ZFS command for security
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() || (parts[0] != "zfs" && parts[0] != "zpool") {
        return Err("Only ZFS commands are allowed".to_string());
    }

    // Execute the actual ZFS command
    match Command::new(parts[0])
        .args(&parts[1..])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute command: {}", e))
    }
}

#[tauri::command]
async fn get_system_info() -> Result<serde_json::Value, String> {
    // Get real system information
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    
    // Get system uptime (Linux/Unix)
    let uptime = match std::fs::read_to_string("/proc/uptime") {
        Ok(content) => {
            content.split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
                .map(|s| s as u64)
                .unwrap_or(0)
        }
        Err(_) => 0
    };
    
    // Get memory information
    let (total_memory, free_memory) = match std::fs::read_to_string("/proc/meminfo") {
        Ok(content) => {
            let mut total = 0u64;
            let mut free = 0u64;
            
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        total = kb_str.parse::<u64>().unwrap_or(0) * 1024;
                    }
                } else if line.starts_with("MemAvailable:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        free = kb_str.parse::<u64>().unwrap_or(0) * 1024;
                    }
                }
            }
            (total, free)
        }
        Err(_) => (0, 0)
    };
    
    let info = serde_json::json!({
        "hostname": hostname,
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "version": "0.2.0",
        "uptime_seconds": uptime,
        "memory": {
            "total_bytes": total_memory,
            "available_bytes": free_memory,
            "used_bytes": total_memory.saturating_sub(free_memory)
        }
    });
    Ok(info)
}

#[tauri::command]
async fn get_zfs_pools() -> Result<serde_json::Value, String> {
    // Get real ZFS pool information
    match Command::new("zpool")
        .args(&["list", "-H", "-p"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut pools = Vec::new();
                
                for line in stdout.lines() {
                    let fields: Vec<&str> = line.split('\t').collect();
                    if fields.len() >= 6 {
                        pools.push(serde_json::json!({
                            "name": fields[0],
                            "size": fields[1].parse::<u64>().unwrap_or(0),
                            "allocated": fields[2].parse::<u64>().unwrap_or(0),
                            "free": fields[3].parse::<u64>().unwrap_or(0),
                            "capacity_percent": fields[4].trim_end_matches('%').parse::<f64>().unwrap_or(0.0),
                            "health": fields[5]
                        }));
                    }
                }
                
                Ok(serde_json::json!({
                    "pools": pools,
                    "count": pools.len()
                }))
            } else {
                Err("Failed to list ZFS pools".to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute zpool command: {}", e))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            execute_zfs_command,
            get_system_info,
            get_zfs_pools
        ])
        .setup(|app| {
            println!("NestGate Desktop Application Starting...");
            println!("System: {} {}", std::env::consts::OS, std::env::consts::ARCH);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 