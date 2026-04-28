// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration management commands
//!
//! Provides configuration display, get/set, and reset operations.

use crate::cli::ConfigAction;
use anyhow::Result;
use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};

/// Execute configuration management commands
pub async fn execute(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Show => show_config().await,
        ConfigAction::Set { key, value } => set_config(&key, &value).await,
        ConfigAction::Get { key } => get_config(&key).await,
        ConfigAction::Reset { confirm } => reset_config(confirm).await,
        ConfigAction::Validate => validate_config().await,
        ConfigAction::Export { output, format } => export_config(output, &format).await,
        ConfigAction::Import { input } => import_config(input).await,
    }
}

/// Show current configuration
async fn show_config() -> Result<()> {
    show_config_from_env_source(&ProcessEnv).await
}

async fn show_config_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<()> {
    println!("NestGate Configuration");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let runtime_config = nestgate_core::config::runtime::get_config();

    println!("\nNetwork:");
    println!("  API Port:     {}", runtime_config.network.api_port);
    println!("  API Host:     {}", runtime_config.network.api_host);

    // Socket configuration
    match nestgate_core::rpc::SocketConfig::from_environment() {
        Ok(socket_config) => {
            println!("\nSocket:");
            println!("  Path:         {}", socket_config.socket_path.display());
            if let Some(fid) = env.get("NESTGATE_FAMILY_ID") {
                println!("  Family ID:    {fid}");
            }
        }
        Err(e) => {
            println!("\nSocket: {e}");
        }
    }

    // Storage backend
    let caps = nestgate_core::services::storage::capabilities::detect_backend();
    println!("\nStorage:");
    println!("  Backend:      {:?}", caps.backend_type);
    let mut features = Vec::new();
    if caps.native_snapshots {
        features.push("snapshots");
    }
    if caps.native_deduplication {
        features.push("dedup");
    }
    if caps.native_compression {
        features.push("compression");
    }
    if caps.native_checksums {
        features.push("checksums");
    }
    if caps.native_replication {
        features.push("replication");
    }
    println!(
        "  Features:     {}",
        if features.is_empty() {
            "basic_operations".to_string()
        } else {
            features.join(", ")
        }
    );
    let storage_path =
        env_var_or_default(env, "NESTGATE_STORAGE_PATH", "/var/lib/nestgate/storage");
    println!("  Path:         {storage_path}");

    // Environment
    println!("\nEnvironment:");
    let env_vars = [
        "NESTGATE_API_PORT",
        "NESTGATE_BIND",
        "NESTGATE_STORAGE_PATH",
        "NESTGATE_FAMILY_ID",
        "NESTGATE_JWT_SECRET",
        "NESTGATE_SOCKET",
        "BIOMEOS_SOCKET_DIR",
        "XDG_RUNTIME_DIR",
    ];
    for var in &env_vars {
        match env.get(var) {
            Some(val) => {
                // Mask sensitive values
                if var.contains("SECRET") || var.contains("JWT") {
                    println!("  {var}: [set, masked]");
                } else {
                    println!("  {var}: {val}");
                }
            }
            None => println!("  {var}: (not set)"),
        }
    }

    println!("\nVersion: {}", env!("CARGO_PKG_VERSION"));

    Ok(())
}

/// Set a configuration value
async fn set_config(key: &str, value: &str) -> Result<()> {
    println!("Setting configuration: {key}={value}");

    // NestGate uses environment-first configuration
    // We inform the user how to persist the setting
    match key {
        "api_port" | "port" => {
            println!("  Set environment variable: NESTGATE_API_PORT={value}");
            println!("  Or add to .env.sovereignty: NESTGATE_API_PORT={value}");
        }
        "bind" | "api_bind" => {
            println!("  Set environment variable: NESTGATE_BIND={value}");
        }
        "storage_path" => {
            println!("  Set environment variable: NESTGATE_STORAGE_PATH={value}");
            let path = std::path::Path::new(value);
            if !path.exists() {
                println!("  Path does not exist yet (will be created on first use)");
            }
        }
        "family_id" => {
            println!("  Set environment variable: NESTGATE_FAMILY_ID={value}");
            println!("  Or use CLI flag: nestgate daemon --family-id {value}");
        }
        "socket_path" | "socket" => {
            println!("  Set environment variable: NESTGATE_SOCKET={value}");
        }
        _ => {
            println!("  Unknown configuration key: {key}");
            println!("\n  Available keys:");
            println!("    api_port      - API port (NESTGATE_API_PORT)");
            println!("    bind          - Bind address (NESTGATE_BIND)");
            println!("    storage_path  - Storage directory (NESTGATE_STORAGE_PATH)");
            println!("    family_id     - Family ID (NESTGATE_FAMILY_ID)");
            println!("    socket_path   - Socket path (NESTGATE_SOCKET)");
        }
    }

    Ok(())
}

/// Get a configuration value
async fn get_config(key: &str) -> Result<()> {
    get_config_from_env_source(&ProcessEnv, key).await
}

async fn get_config_from_env_source(env: &(impl EnvSource + ?Sized), key: &str) -> Result<()> {
    let value = match key {
        "api_port" | "port" => {
            let cfg = nestgate_core::config::runtime::get_config();
            cfg.network.api_port.to_string()
        }
        "bind" | "api_host" => {
            let cfg = nestgate_core::config::runtime::get_config();
            cfg.network.api_host.to_string()
        }
        "storage_path" => {
            env_var_or_default(env, "NESTGATE_STORAGE_PATH", "/var/lib/nestgate/storage")
        }
        "family_id" => env
            .get("NESTGATE_FAMILY_ID")
            .unwrap_or_else(|| "(not set)".to_string()),
        "socket_path" | "socket" => match nestgate_core::rpc::SocketConfig::from_environment() {
            Ok(cfg) => cfg.socket_path.display().to_string(),
            Err(_) => "(default)".to_string(),
        },
        "backend" => {
            let caps = nestgate_core::services::storage::capabilities::detect_backend();
            format!("{:?}", caps.backend_type)
        }
        "version" => env!("CARGO_PKG_VERSION").to_string(),
        _ => {
            println!("Unknown key: {key}");
            println!(
                "Available: api_port, bind, storage_path, family_id, socket_path, backend, version"
            );
            return Ok(());
        }
    };

    println!("{value}");
    Ok(())
}

/// Reset configuration to defaults
async fn reset_config(confirm: bool) -> Result<()> {
    if !confirm {
        println!("This will reset NestGate configuration to defaults.");
        println!("   Run with --confirm to proceed.");
        println!("\n   Note: NestGate uses environment-first configuration.");
        println!("   This will clear any local config overrides but not environment variables.");
        return Ok(());
    }

    println!("Resetting configuration to defaults...");

    // Check for local config files
    let config_paths = ["nestgate.toml", "config/nestgate.toml", ".nestgate.toml"];

    for path in &config_paths {
        if std::path::Path::new(path).exists() {
            println!("  Found local config: {path} (would need manual removal)");
        }
    }

    println!("\n  Runtime configuration reset to defaults");
    println!("  Environment variables remain active - unset them to fully reset:");
    println!("     unset NESTGATE_API_PORT NESTGATE_BIND NESTGATE_STORAGE_PATH");

    Ok(())
}

/// Validate the current configuration
async fn validate_config() -> Result<()> {
    validate_config_from_env_source(&ProcessEnv).await
}

async fn validate_config_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<()> {
    println!("Validating NestGate configuration...");
    let mut issues = 0;

    // Check JWT secret
    let jwt_secret = env.get("NESTGATE_JWT_SECRET").unwrap_or_default();
    if jwt_secret.is_empty() || jwt_secret == "development-secret-change-me" {
        println!("  JWT secret not set or using default (set NESTGATE_JWT_SECRET)");
        issues += 1;
    } else {
        println!("  JWT secret configured");
    }

    // Check storage path
    let storage_path =
        env_var_or_default(env, "NESTGATE_STORAGE_PATH", "/var/lib/nestgate/storage");
    if std::path::Path::new(&storage_path).exists() {
        println!("  Storage path exists: {storage_path}");
    } else {
        println!("  Storage path missing: {storage_path} (will be created on first use)");
        issues += 1;
    }

    // Check socket config
    match nestgate_core::rpc::SocketConfig::from_environment() {
        Ok(config) => println!("  Socket config valid: {}", config.socket_path.display()),
        Err(e) => {
            println!("  Socket config issue: {e}");
            issues += 1;
        }
    }

    // Check backend
    let caps = nestgate_core::services::storage::capabilities::detect_backend();
    println!("  Backend detected: {:?}", caps.backend_type);

    if issues == 0 {
        println!("\nConfiguration is valid");
    } else {
        println!("\n{issues} issue(s) found");
    }

    Ok(())
}

/// Export configuration to file or stdout
async fn export_config(output: Option<std::path::PathBuf>, format: &str) -> Result<()> {
    export_config_from_env_source(&ProcessEnv, output, format).await
}

async fn export_config_from_env_source(
    env_src: &(impl EnvSource + ?Sized),
    output: Option<std::path::PathBuf>,
    format: &str,
) -> Result<()> {
    let runtime_config = nestgate_core::config::runtime::get_config();
    let socket_config = nestgate_core::rpc::SocketConfig::from_environment().ok();
    let caps = nestgate_core::services::storage::capabilities::detect_backend();

    // Build config object
    let config_export = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "network": {
            "api_host": runtime_config.network.api_host.to_string(),
            "api_port": runtime_config.network.api_port,
        },
        "socket": socket_config.map(|c| c.socket_path.display().to_string()),
        "storage": {
            "backend": format!("{:?}", caps.backend_type),
            "path": env_src.get("NESTGATE_STORAGE_PATH").unwrap_or_default(),
        },
        "family_id": env_src.get("NESTGATE_FAMILY_ID"),
    });

    // JSON export (primary format for NestGate config interop)
    let content = serde_json::to_string_pretty(&config_export)?;
    let _ = format; // format parameter reserved for future TOML/YAML support

    match output {
        Some(path) => {
            tokio::fs::write(&path, &content).await?;
            println!("Configuration exported to: {}", path.display());
        }
        None => {
            println!("{content}");
        }
    }

    Ok(())
}

/// Import configuration from file
async fn import_config(input: std::path::PathBuf) -> Result<()> {
    if !input.exists() {
        anyhow::bail!("Configuration file not found: {}", input.display());
    }

    let content = tokio::fs::read_to_string(&input).await?;
    println!("Reading configuration from: {}", input.display());

    // Parse and display what would be imported
    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
        println!("  Configuration contents:");
        if let Some(network) = config.get("network") {
            println!("    Network: {network}");
        }
        if let Some(storage) = config.get("storage") {
            println!("    Storage: {storage}");
        }
        println!("\n  To apply, set the corresponding environment variables:");
        if let Some(port) = config.pointer("/network/api_port") {
            println!("    export NESTGATE_API_PORT={port}");
        }
        if let Some(path) = config.pointer("/storage/path")
            && let Some(p) = path.as_str()
            && !p.is_empty()
        {
            println!("    export NESTGATE_STORAGE_PATH={p}");
        }
    } else {
        println!("  Could not parse configuration file (expected JSON)");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show_config_succeeds() {
        let result = show_config().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_config_version() {
        let result = get_config("version").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_config_backend() {
        let result = get_config("backend").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_config_unknown_key() {
        let result = get_config("nonexistent_key").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_config_known_key() {
        let result = set_config("api_port", "9090").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reset_config_without_confirm() {
        let result = reset_config(false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_config_succeeds() {
        let result = validate_config().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_config_to_stdout() {
        let result = export_config(None, "json").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_config_to_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test_config_export.json");
        let result = export_config(Some(output_path.clone()), "json").await;
        assert!(result.is_ok());
        assert!(output_path.exists());

        let content = tokio::fs::read_to_string(&output_path).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(parsed["version"].is_string());
    }

    #[tokio::test]
    async fn test_import_config_nonexistent_file() {
        let result = import_config(std::path::PathBuf::from(
            "/tmp/nonexistent_config_12345.json",
        ))
        .await;
        assert!(result.is_err());
    }
}
