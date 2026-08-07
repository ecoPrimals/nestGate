// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Ecosystem socket path resolution conventions (G66 Transport Abstraction).
//!
//! Primals use standard socket path patterns for UDS communication:
//! - `$ECOSYSTEM_SOCKET_DIR/{name}[-{family_id}].sock` (explicit override)
//! - `$XDG_RUNTIME_DIR/<ecosystem>/{name}[-{family_id}].sock` (XDG convention)
//! - `/tmp/<ecosystem>/{name}[-{family_id}].sock` (fallback)

use std::path::PathBuf;

const ECOSYSTEM_SOCKET_DIR_KEY: &str = "ECOSYSTEM_SOCKET_DIR";
const BIOMEOS_SOCKET_DIR_KEY: &str = "BIOMEOS_SOCKET_DIR";
const XDG_RUNTIME_DIR_KEY: &str = "XDG_RUNTIME_DIR";
const ECOSYSTEM_NAME_KEY: &str = "ECOSYSTEM_NAME";
const BIOMEOS_SERVICE_NAME_KEY: &str = "BIOMEOS_SERVICE_NAME";
const DEFAULT_ECOSYSTEM_DIR: &str = "biomeos";
const FALLBACK_RUNTIME_DIR: &str = "/tmp";

/// Resolve the socket path for a primal using ecosystem conventions.
///
/// Checks `ECOSYSTEM_SOCKET_DIR` (or legacy `BIOMEOS_SOCKET_DIR`),
/// then `$XDG_RUNTIME_DIR/<ecosystem>/`, then `/tmp/<ecosystem>/`.
#[must_use]
pub fn resolve_socket_path(primal_name: &str, family_id: Option<&str>) -> PathBuf {
    let eco = std::env::var(ECOSYSTEM_NAME_KEY)
        .or_else(|_| std::env::var(BIOMEOS_SERVICE_NAME_KEY))
        .unwrap_or_else(|_| DEFAULT_ECOSYSTEM_DIR.to_owned());

    let socket_dir = std::env::var(ECOSYSTEM_SOCKET_DIR_KEY)
        .or_else(|_| std::env::var(BIOMEOS_SOCKET_DIR_KEY))
        .unwrap_or_else(|_| {
            let runtime_dir = std::env::var(XDG_RUNTIME_DIR_KEY)
                .unwrap_or_else(|_| FALLBACK_RUNTIME_DIR.to_owned());
            format!("{runtime_dir}/{eco}")
        });

    socket_path_in(&socket_dir, primal_name, family_id)
}

/// Build a socket path from explicit components (no env var reads).
#[must_use]
pub fn socket_path_in(socket_dir: &str, primal_name: &str, family_id: Option<&str>) -> PathBuf {
    let filename = family_id
        .filter(|id| !id.is_empty() && *id != "default")
        .map_or_else(
            || format!("{primal_name}.sock"),
            |fid| format!("{primal_name}-{fid}.sock"),
        );
    PathBuf::from(socket_dir).join(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_path_in_without_family() {
        let path = socket_path_in("/run/biomeos", "nestgate", None);
        assert_eq!(path, PathBuf::from("/run/biomeos/nestgate.sock"));
    }

    #[test]
    fn socket_path_in_with_family() {
        let path = socket_path_in("/run/biomeos", "nestgate", Some("abc123"));
        assert_eq!(path, PathBuf::from("/run/biomeos/nestgate-abc123.sock"));
    }

    #[test]
    fn socket_path_in_default_family_ignored() {
        let path = socket_path_in("/run/biomeos", "nestgate", Some("default"));
        assert_eq!(path, PathBuf::from("/run/biomeos/nestgate.sock"));
    }

    #[test]
    fn socket_path_in_empty_family_ignored() {
        let path = socket_path_in("/run/biomeos", "nestgate", Some(""));
        assert_eq!(path, PathBuf::from("/run/biomeos/nestgate.sock"));
    }
}
