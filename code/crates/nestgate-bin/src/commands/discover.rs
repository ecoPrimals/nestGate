// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery command implementations.
//!
//! Primal self-knowledge via runtime discovery.

use crate::cli::DiscoverTarget;
use crate::error::BinResult;
use nestgate_core::services::storage::capabilities;
use nestgate_types::{EnvSource, ProcessEnv};

/// Execute discovery commands
pub async fn execute(target: DiscoverTarget) -> BinResult<()> {
    match target {
        DiscoverTarget::Primals => discover_primals_from_env_source(&ProcessEnv).await,
        DiscoverTarget::Services => discover_services().await,
        DiscoverTarget::Capabilities => discover_capabilities().await,
    }
}

/// Discover primals in the local ecosystem.
async fn discover_primals_from_env_source(env: &(impl EnvSource + ?Sized)) -> BinResult<()> {
    println!("Primal Discovery");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Self-knowledge: NestGate always knows itself
    println!("Local Primals:");
    println!(
        "   nestgate (self) - Universal Data Orchestrator v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Discover via socket directory
    let socket_dir = discover_socket_dir_from_env_source(env);
    println!();
    if let Some(dir) = &socket_dir {
        println!("Socket Directory: {}", dir.display());

        match tokio::fs::read_dir(dir).await {
            Ok(mut entries) => {
                let mut found = 0;
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.ends_with(".sock") {
                        let primal_name = name_str.trim_end_matches(".sock");
                        // Check if socket is alive
                        let status = if tokio::net::UnixStream::connect(entry.path()).await.is_ok()
                        {
                            "ALIVE"
                        } else {
                            "STALE"
                        };
                        println!("   {} {} ({})", status, primal_name, entry.path().display());
                        found += 1;
                    }
                }
                if found == 0 {
                    println!("   (no primal sockets found)");
                }
            }
            Err(e) => {
                println!("   Cannot read socket directory: {e}");
            }
        }
    } else {
        println!();
        println!("   No socket directory configured");
        println!("   Set NESTGATE_SOCKET or BIOMEOS_SOCKET_DIR to enable discovery");
    }

    println!();
    Ok(())
}

/// Discover available services
async fn discover_services() -> BinResult<()> {
    println!("Service Discovery");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // NestGate's own services
    println!("NestGate Services:");
    println!("   storage      - Persistent key-value storage");
    println!("   blob_storage - Binary large object storage");
    println!("   model_cache  - Model registration and discovery");
    println!("   templates    - Collaborative template management");
    println!("   audit        - Execution audit trail");

    // Backend detection
    let caps = capabilities::detect_backend();
    println!();
    println!("Storage Backend:");
    match caps.backend_type {
        capabilities::BackendType::Zfs => {
            println!("   Type: ZFS (full feature set)");
            println!("   Native snapshots");
            println!("   Native deduplication");
            println!("   Native compression");
            println!("   Native checksums");
            println!("   Native replication");
        }
        capabilities::BackendType::Filesystem => {
            println!("   Type: Filesystem (universal compatibility)");
            println!("   Basic operations");
            println!("   Software-level snapshots, dedup, compression");
        }
    }

    println!();
    Ok(())
}

/// Discover available capabilities (JSON-RPC methods)
async fn discover_capabilities() -> BinResult<()> {
    println!("Capability Discovery");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("Available JSON-RPC Methods:");
    println!();

    // Health & Discovery
    println!("   Health & Discovery:");
    println!("     • health                    - Service health check");
    println!("     • discover_capabilities     - List all capabilities");
    println!();

    println!("   Storage:");
    println!("     • storage.store             - Store key-value data");
    println!("     • storage.retrieve          - Retrieve data by key");
    println!("     • storage.exists            - Check key existence");
    println!("     • storage.delete            - Delete data by key");
    println!("     • storage.list              - List keys by prefix");
    println!("     • storage.stats             - Storage statistics");
    println!("     • storage.store_blob        - Store binary data");
    println!("     • storage.retrieve_blob     - Retrieve binary data");
    println!();

    println!("   Streaming Storage:");
    println!("     • storage.store_stream      - Begin chunked store");
    println!("     • storage.store_stream_chunk - Upload chunk");
    println!("     • storage.retrieve_stream   - Begin chunked retrieve");
    println!("     • storage.retrieve_stream_chunk - Download chunk");
    println!();

    // Model cache
    println!("   Model Cache:");
    println!("     • model.register            - Register model with metadata");
    println!("     • model.exists              - Check model availability");
    println!("     • model.locate              - Find gates with model");
    println!("     • model.metadata            - Get model registration info");
    println!();

    // Templates
    println!("   Templates:");
    println!("     • templates.store           - Store template");
    println!("     • templates.retrieve        - Retrieve template");
    println!("     • templates.list            - List templates");
    println!("     • templates.community_top   - Top community templates");
    println!();

    // Audit
    println!("   Audit:");
    println!("     • audit.store_execution     - Store execution audit");
    println!();

    let caps = capabilities::detect_backend();
    println!("   Backend: {:?}", caps.backend_type);
    println!();

    Ok(())
}

/// Discover the socket directory from an injectable [`EnvSource`].
/// Production entry points pass [`ProcessEnv`].
fn discover_socket_dir_from_env_source(
    env: &(impl EnvSource + ?Sized),
) -> Option<std::path::PathBuf> {
    // Check explicit socket path
    if let Some(socket) = env.get("NESTGATE_SOCKET") {
        let path = std::path::PathBuf::from(&socket);
        return path.parent().map(std::path::Path::to_path_buf);
    }

    // Ecosystem shared socket directory (`BIOMEOS_SOCKET_DIR`; standard wateringHole path name)
    if let Some(dir) = env.get("BIOMEOS_SOCKET_DIR") {
        return Some(std::path::PathBuf::from(dir));
    }

    // Check XDG runtime directory
    if let Some(xdg) = env.get("XDG_RUNTIME_DIR") {
        let biomeos_dir = std::path::PathBuf::from(xdg)
            .join(nestgate_core::constants::system::ecosystem_path_segment());
        if biomeos_dir.exists() {
            return Some(biomeos_dir);
        }
    }

    // Fallback: check /tmp for sockets
    let tmp_dir = std::path::PathBuf::from("/tmp");
    Some(tmp_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_socket_dir_returns_some() {
        // Should always return Some (at minimum /tmp fallback)
        let result = discover_socket_dir_from_env_source(&ProcessEnv);
        assert!(result.is_some());
    }

    #[test]
    fn test_discover_socket_dir_from_biomeos_env() {
        // Temporarily set BIOMEOS_SOCKET_DIR
        let original = std::env::var("BIOMEOS_SOCKET_DIR").ok();
        nestgate_core::env_process::set_var("BIOMEOS_SOCKET_DIR", "/tmp/test-biomeos-sockets");
        let result = discover_socket_dir_from_env_source(&ProcessEnv);
        assert_eq!(
            result,
            Some(std::path::PathBuf::from("/tmp/test-biomeos-sockets"))
        );
        // Restore
        match original {
            Some(val) => nestgate_core::env_process::set_var("BIOMEOS_SOCKET_DIR", val),
            None => nestgate_core::env_process::remove_var("BIOMEOS_SOCKET_DIR"),
        }
    }

    #[tokio::test]
    async fn test_execute_capabilities_succeeds() {
        // discover_capabilities prints to stdout - should not error
        let result = discover_capabilities().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_services_succeeds() {
        let result = discover_services().await;
        assert!(result.is_ok());
    }
}
