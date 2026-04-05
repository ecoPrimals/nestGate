// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Socket discovery helpers for atomic composition health checks.
//!
//! These functions scan standard socket locations to discover primal
//! endpoints at runtime, following the zero-hardcoding and capability-based
//! discovery principles from wateringHole.

use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use std::path::PathBuf;
use tracing::debug;

pub(super) fn local_primal_id() -> String {
    std::env::var("NESTGATE_SERVICE_NAME")
        .or_else(|_| std::env::var("NESTGATE_PRIMAL_ID"))
        .unwrap_or_else(|_| DEFAULT_SERVICE_NAME.to_string())
}

/// Discover a primal's Unix socket path via standard locations.
pub(super) fn discover_primal_socket(primal_name: &str) -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
        let path = PathBuf::from(dir).join(format!("{primal_name}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(xdg)
            .join("biomeos")
            .join(format!("{primal_name}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    let tmp_path = PathBuf::from("/tmp").join(format!("{primal_name}.sock"));
    if tmp_path.exists() {
        return Some(tmp_path);
    }

    if let (Ok(family_id), Ok(xdg)) = (
        std::env::var("NESTGATE_FAMILY_ID"),
        std::env::var("XDG_RUNTIME_DIR"),
    ) {
        let path = PathBuf::from(xdg)
            .join("biomeos")
            .join(format!("{primal_name}-{family_id}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Discover available primals by scanning standard socket locations.
pub(super) fn discover_available_primals() -> Vec<String> {
    let mut primals = Vec::new();
    let socket_dirs = gather_socket_search_dirs();

    for dir in &socket_dirs {
        let dir_path = std::path::Path::new(dir);
        if !dir_path.exists() {
            continue;
        }

        let Ok(entries) = std::fs::read_dir(dir_path) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };

            if path.extension().and_then(|e| e.to_str()) == Some("sock") {
                let primal_name = name.split('-').next().unwrap_or(name);
                if !primal_name.is_empty()
                    && !primals.iter().any(|p: &String| p.as_str() == primal_name)
                {
                    primals.push(primal_name.to_string());
                }
            }
        }
    }

    debug!("Discovered primals via socket scan: {:?}", primals);
    primals
}

/// Gather standard directories to search for primal sockets.
pub(super) fn gather_socket_search_dirs() -> Vec<String> {
    let mut dirs = Vec::new();

    if let Ok(dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
        dirs.push(dir);
    }

    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        dirs.push(format!("{xdg}/biomeos"));
    }

    let uid = uzers::get_current_uid();
    let xdg_default = format!("/run/user/{uid}/biomeos");
    if !dirs.contains(&xdg_default) {
        dirs.push(xdg_default);
    }

    dirs.push("/tmp".to_string());
    dirs
}
