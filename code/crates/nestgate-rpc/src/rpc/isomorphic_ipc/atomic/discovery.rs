// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Socket discovery helpers for atomic composition health checks.
//!
//! These functions scan standard socket locations to discover primal
//! endpoints at runtime, following the zero-hardcoding and capability-based
//! discovery principles from wateringHole.

use nestgate_config::constants::system::{DEFAULT_SERVICE_NAME, ecosystem_path_segment};
use nestgate_types::{EnvSource, ProcessEnv};
use std::path::PathBuf;
use tracing::debug;

pub(super) fn local_primal_id_from_env(env: &dyn EnvSource) -> String {
    env.get("NESTGATE_SERVICE_NAME")
        .or_else(|| env.get("NESTGATE_PRIMAL_ID"))
        .unwrap_or_else(|| DEFAULT_SERVICE_NAME.to_string())
}

pub(super) fn local_primal_id() -> String {
    local_primal_id_from_env(&ProcessEnv)
}

/// Discover a primal's Unix socket path via standard locations.
pub(super) fn discover_primal_socket_from_env(
    env: &dyn EnvSource,
    primal_name: &str,
) -> Option<PathBuf> {
    if let Some(dir) = env.get("BIOMEOS_SOCKET_DIR") {
        let path = PathBuf::from(dir).join(format!("{primal_name}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    if let Some(xdg) = env.get("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(xdg)
            .join(ecosystem_path_segment())
            .join(format!("{primal_name}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    let tmp_path = PathBuf::from("/tmp").join(format!("{primal_name}.sock"));
    if tmp_path.exists() {
        return Some(tmp_path);
    }

    if let (Some(family_id), Some(xdg)) =
        (env.get("NESTGATE_FAMILY_ID"), env.get("XDG_RUNTIME_DIR"))
    {
        let path = PathBuf::from(xdg)
            .join(ecosystem_path_segment())
            .join(format!("{primal_name}-{family_id}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    None
}

pub(super) fn discover_primal_socket(primal_name: &str) -> Option<PathBuf> {
    discover_primal_socket_from_env(&ProcessEnv, primal_name)
}

/// Discover available primals by scanning standard socket locations.
pub(super) fn discover_available_primals_from_env(env: &dyn EnvSource) -> Vec<String> {
    let mut primals = Vec::new();
    let socket_dirs = gather_socket_search_dirs_from_env(env);

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

pub(super) fn discover_available_primals() -> Vec<String> {
    discover_available_primals_from_env(&ProcessEnv)
}

/// Gather standard directories to search for primal sockets.
pub(super) fn gather_socket_search_dirs_from_env(env: &dyn EnvSource) -> Vec<String> {
    let mut dirs = Vec::new();

    if let Some(dir) = env.get("BIOMEOS_SOCKET_DIR") {
        dirs.push(dir);
    }

    if let Some(xdg) = env.get("XDG_RUNTIME_DIR") {
        dirs.push(format!("{xdg}/{}", ecosystem_path_segment()));
    }

    let uid = uzers::get_current_uid();
    let xdg_default = format!("/run/user/{uid}/{}", ecosystem_path_segment());
    if !dirs.contains(&xdg_default) {
        dirs.push(xdg_default);
    }

    dirs.push("/tmp".to_string());
    dirs
}
