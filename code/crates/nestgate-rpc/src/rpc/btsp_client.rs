// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Security capability provider socket resolution.
//!
//! Resolves the Unix domain socket path for the security capability provider
//! (e.g. bearDog) used by both the BTSP server handshake and the BTSP client
//! handshake. The actual wire-level handshake is in
//! [`super::btsp_client_handshake`].

use std::path::PathBuf;

/// Final fallback path when [`resolve_security_socket_path`] exhausts all 5 higher-priority
/// tiers (env vars and `$XDG_RUNTIME_DIR/biomeos/` discovery).
///
/// Overridable via `NESTGATE_SECURITY_SOCKET` environment variable.
/// Constructs from `$XDG_RUNTIME_DIR` or `/run/user/{uid}` — never hardcodes
/// a fixed FHS path.
pub fn default_security_socket_path() -> PathBuf {
    if let Ok(p) = std::env::var("NESTGATE_SECURITY_SOCKET")
        && !p.is_empty()
    {
        return PathBuf::from(p);
    }
    let socket_dir = std::env::var("ECOSYSTEM_SOCKET_DIR")
        .unwrap_or_else(|_| nestgate_config::constants::system::ecosystem_path_segment());
    let runtime_base = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| format!("/run/user/{}", rustix::process::getuid().as_raw()));
    PathBuf::from(runtime_base)
        .join(socket_dir)
        .join("security.sock")
}

/// Returns `true` when BTSP is mandatory.
///
/// Delegates to [`super::btsp_server_handshake::is_btsp_required`] so
/// client and server use identical env-var resolution and sentinel logic.
#[must_use]
#[cfg(test)]
pub fn is_btsp_required() -> bool {
    super::btsp_server_handshake::is_btsp_required()
}

/// Resolves the security capability provider's Unix socket path.
///
/// Precedence:
/// 1. `SECURITY_PROVIDER_SOCKET` env
/// 2. `CRYPTO_PROVIDER_SOCKET` env
/// 3. `SECURITY_SOCKET` env
/// 4. `SECURITY_ENDPOINT` if it is a local filesystem path (not a `scheme://` URL)
/// 5. Capability-scoped discovery: `$XDG_RUNTIME_DIR/biomeos/{security,crypto}.sock`
/// 6. [`default_security_socket_path`] (overridable via `NESTGATE_SECURITY_SOCKET`)
#[must_use]
pub fn resolve_security_socket_path() -> PathBuf {
    for var in [
        "SECURITY_PROVIDER_SOCKET",
        "CRYPTO_PROVIDER_SOCKET",
        "SECURITY_SOCKET",
    ] {
        if let Ok(p) = std::env::var(var)
            && !p.is_empty()
        {
            return PathBuf::from(p);
        }
    }
    if let Ok(p) = std::env::var("SECURITY_ENDPOINT")
        && !p.contains("://")
        && !p.is_empty()
    {
        return PathBuf::from(p);
    }
    if let Some(path) = discover_security_socket_xdg() {
        return path;
    }
    default_security_socket_path()
}

/// Capability socket names probed during XDG runtime directory discovery.
///
/// Names are capability-based (not primal-specific): `security.sock` is the
/// canonical name, `crypto.sock` is an alias accepted by some providers.
const SECURITY_SOCKET_CANDIDATES: &[&str] = &["security.sock", "crypto.sock"];

/// Scans `$XDG_RUNTIME_DIR/{socket_dir}/` for a security capability provider socket.
///
/// The socket subdirectory is resolved via [`nestgate_config::constants::system::ecosystem_path_segment`]
/// (`ECOSYSTEM_NAME` env → `BIOMEOS_SERVICE_NAME` env → `"biomeos"` default), or
/// overridden with `ECOSYSTEM_SOCKET_DIR`.
fn discover_security_socket_xdg() -> Option<PathBuf> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").ok()?;
    let socket_dir = std::env::var("ECOSYSTEM_SOCKET_DIR")
        .unwrap_or_else(|_| nestgate_config::constants::system::ecosystem_path_segment());
    let base = PathBuf::from(runtime_dir).join(socket_dir);
    for name in SECURITY_SOCKET_CANDIDATES {
        let candidate = base.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_all_security_vars() -> Vec<(&'static str, Option<&'static str>)> {
        vec![
            ("SECURITY_PROVIDER_SOCKET", None),
            ("CRYPTO_PROVIDER_SOCKET", None),
            ("SECURITY_SOCKET", None),
            ("SECURITY_ENDPOINT", None),
            ("NESTGATE_SECURITY_SOCKET", None),
            ("XDG_RUNTIME_DIR", None),
            ("ECOSYSTEM_SOCKET_DIR", None),
        ]
    }

    #[test]
    fn resolve_security_provider_socket_wins() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_PROVIDER_SOCKET", Some("/provider/sec.sock")));
        vars.push(("SECURITY_SOCKET", Some("/old/sec.sock")));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                PathBuf::from("/provider/sec.sock")
            );
        });
    }

    #[test]
    fn resolve_crypto_provider_socket_second() {
        let mut vars = clear_all_security_vars();
        vars.push(("CRYPTO_PROVIDER_SOCKET", Some("/crypto/sec.sock")));
        vars.push(("SECURITY_SOCKET", Some("/old/sec.sock")));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                PathBuf::from("/crypto/sec.sock")
            );
        });
    }

    #[test]
    fn resolve_security_socket_env_order() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_SOCKET", Some("/sock/a")));
        vars.push(("SECURITY_ENDPOINT", Some("/sock/b")));
        temp_env::with_vars(vars, || {
            assert_eq!(resolve_security_socket_path(), PathBuf::from("/sock/a"));
        });
    }

    #[test]
    fn resolve_security_endpoint_skips_url() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_ENDPOINT", Some("http://127.0.0.1:9")));
        temp_env::with_vars(vars, || {
            let path = resolve_security_socket_path();
            assert!(
                path.ends_with("security.sock"),
                "expected XDG-based security.sock path, got {path:?}",
            );
            assert!(
                !path.starts_with("/run/capability"),
                "should not use hardcoded /run/capability path",
            );
        });
    }

    #[test]
    fn resolve_xdg_discovery_finds_security_sock() {
        let dir = tempfile::tempdir().expect("tempdir");
        let biomeos = dir.path().join("biomeos");
        std::fs::create_dir_all(&biomeos).unwrap();
        std::fs::write(biomeos.join("security.sock"), "").unwrap();

        let xdg_str = dir.path().to_str().unwrap().to_string();
        let mut vars = clear_all_security_vars();
        vars.push(("XDG_RUNTIME_DIR", Some(xdg_str.as_str())));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                biomeos.join("security.sock")
            );
        });
    }

    #[test]
    fn resolve_xdg_discovery_finds_crypto_sock() {
        let dir = tempfile::tempdir().expect("tempdir");
        let biomeos = dir.path().join("biomeos");
        std::fs::create_dir_all(&biomeos).unwrap();
        std::fs::write(biomeos.join("crypto.sock"), "").unwrap();

        let xdg_str = dir.path().to_str().unwrap().to_string();
        let mut vars = clear_all_security_vars();
        vars.push(("XDG_RUNTIME_DIR", Some(xdg_str.as_str())));
        temp_env::with_vars(vars, || {
            assert_eq!(resolve_security_socket_path(), biomeos.join("crypto.sock"));
        });
    }

    #[test]
    fn resolve_empty_env_skipped() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_PROVIDER_SOCKET", Some("")));
        vars.push(("SECURITY_SOCKET", Some("/real.sock")));
        temp_env::with_vars(vars, || {
            assert_eq!(resolve_security_socket_path(), PathBuf::from("/real.sock"));
        });
    }

    #[test]
    fn is_btsp_required_respects_family_and_insecure() {
        temp_env::with_vars(
            [("FAMILY_ID", None::<&str>), ("BIOMEOS_INSECURE", None)],
            || assert!(!is_btsp_required()),
        );
        temp_env::with_vars(
            [("FAMILY_ID", Some("prod")), ("BIOMEOS_INSECURE", None)],
            || assert!(is_btsp_required()),
        );
        temp_env::with_vars(
            [("FAMILY_ID", Some("prod")), ("BIOMEOS_INSECURE", Some("1"))],
            || assert!(!is_btsp_required()),
        );
        temp_env::with_vars(
            [("FAMILY_ID", Some("default")), ("BIOMEOS_INSECURE", None)],
            || assert!(!is_btsp_required()),
        );
    }
}
