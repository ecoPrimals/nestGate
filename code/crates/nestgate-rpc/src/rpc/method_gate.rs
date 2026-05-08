// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pre-dispatch capability gate for JSON-RPC methods (JH-0).
//!
//! Every incoming RPC call passes through [`MethodGate::check`] *before*
//! reaching the dispatch table. The gate classifies methods into
//! [`MethodAccessLevel::Public`] (allowed without any token — health probes,
//! identity, capability advertisement) and [`MethodAccessLevel::Protected`]
//! (require a valid capability token once enforcement is activated).
//!
//! Two enforcement modes control behavior:
//! - **Permissive** (default): protected methods are logged but allowed,
//!   preserving backward compatibility during ecosystem rollout.
//! - **Enforced**: protected methods without a valid token are rejected
//!   with `PERMISSION_DENIED` (-32001).
//!
//! Caller identity will be extracted from `SO_PEERCRED` on Unix sockets once
//! the `peer_credentials_unix_socket` API stabilizes. Until then, the gate
//! operates on bearer tokens and connection origin.

use serde_json::{Value, json};

/// JSON-RPC error codes for the method gate (server-defined range).
pub mod error_codes {
    /// Caller identity established but lacks scope for the method.
    pub const PERMISSION_DENIED: i32 = -32_001;
}

/// Access level for a JSON-RPC method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodAccessLevel {
    /// Health probes, identity, capability advertisement — always allowed.
    Public,
    /// Requires a valid capability token when enforcement is active.
    Protected,
}

/// Prefix patterns that match public methods.
const PUBLIC_METHOD_PREFIXES: &[&str] = &["health.", "auth."];

/// Exact method names that are always public.
const PUBLIC_METHODS: &[&str] = &[
    "health",
    "identity.get",
    "capabilities.list",
    "capability.list",
    "discover_capabilities",
    "discover.capabilities",
    "discovery.capability.register",
    "lifecycle.status",
];

/// Classify a method string into its access level.
#[must_use]
pub fn classify_method(method: &str) -> MethodAccessLevel {
    if PUBLIC_METHODS.contains(&method) {
        return MethodAccessLevel::Public;
    }
    for prefix in PUBLIC_METHOD_PREFIXES {
        if method.starts_with(prefix) {
            return MethodAccessLevel::Public;
        }
    }
    MethodAccessLevel::Protected
}

/// How the caller connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionOrigin {
    /// Local Unix domain socket.
    Unix,
}

/// Identity and authorization context for an incoming RPC call.
///
/// Peer credentials (`SO_PEERCRED`) are deferred until the unstable
/// `peer_credentials_unix_socket` API stabilizes. The gate currently
/// operates on bearer tokens and connection origin.
#[derive(Debug, Clone)]
pub struct CallerContext {
    /// Optional bearer / capability token sent in the request.
    pub bearer_token: Option<String>,
    /// Where the connection came from.
    pub origin: ConnectionOrigin,
}

impl CallerContext {
    /// Create a caller context for a Unix domain socket connection.
    #[must_use]
    pub const fn unix() -> Self {
        Self {
            bearer_token: None,
            origin: ConnectionOrigin::Unix,
        }
    }
}

/// Enforcement mode for the method gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementMode {
    /// Log violations but allow all calls (backward-compatible default).
    Permissive,
    /// Reject unauthenticated calls to protected methods.
    Enforced,
}

impl EnforcementMode {
    /// Resolve from `NESTGATE_AUTH_MODE` env var.
    /// Defaults to `Permissive` if unset or unrecognized.
    #[must_use]
    pub fn from_env() -> Self {
        match std::env::var("NESTGATE_AUTH_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "enforced" | "enforce" | "strict" => Self::Enforced,
            _ => Self::Permissive,
        }
    }

    /// Human-readable label for diagnostics and `auth.mode` responses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permissive => "permissive",
            Self::Enforced => "enforced",
        }
    }
}

/// Pre-dispatch gate that checks caller authorization before method execution.
#[derive(Debug, Clone)]
pub struct MethodGate {
    mode: EnforcementMode,
}

impl MethodGate {
    /// Create a gate with the given enforcement mode.
    #[must_use]
    pub const fn new(mode: EnforcementMode) -> Self {
        Self { mode }
    }

    /// Create a gate from the environment (`NESTGATE_AUTH_MODE`).
    #[must_use]
    pub fn from_env() -> Self {
        Self::new(EnforcementMode::from_env())
    }

    /// Current enforcement mode.
    #[must_use]
    pub const fn mode(&self) -> EnforcementMode {
        self.mode
    }

    /// Pre-dispatch authorization check.
    ///
    /// Returns `Ok(())` if the call should proceed to the dispatch table.
    ///
    /// # Errors
    ///
    /// Returns a `MethodGateRejection` when a protected method is called
    /// without a valid capability token and the gate is in `Enforced` mode.
    pub fn check(&self, method: &str, caller: &CallerContext) -> Result<(), MethodGateRejection> {
        if classify_method(method) == MethodAccessLevel::Public {
            return Ok(());
        }

        let authorized = caller.bearer_token.is_some();
        if authorized {
            return Ok(());
        }

        match self.mode {
            EnforcementMode::Permissive => {
                tracing::warn!(
                    method,
                    origin = ?caller.origin,
                    "method gate: unauthenticated call to protected method (permissive — allowing)"
                );
                Ok(())
            }
            EnforcementMode::Enforced => {
                tracing::warn!(
                    method,
                    origin = ?caller.origin,
                    "method gate: REJECTED unauthenticated call to protected method"
                );
                Err(MethodGateRejection {
                    code: error_codes::PERMISSION_DENIED,
                    method: method.to_owned(),
                })
            }
        }
    }
}

/// Rejection from the method gate, carrying the error code and method name
/// for response construction at the call site.
#[derive(Debug)]
pub struct MethodGateRejection {
    pub code: i32,
    pub method: String,
}

/// Handle `auth.*` introspection methods. Returns `Some(json)` for
/// `auth.check`, `auth.mode`, `auth.peer_info`; `None` for other methods.
pub fn auth_introspection(
    method: &str,
    gate: &MethodGate,
    caller: &CallerContext,
) -> Option<Value> {
    match method {
        "auth.mode" => Some(json!({
            "mode": gate.mode().as_str(),
            "primal": "nestgate",
        })),
        "auth.check" => {
            let has_token = caller.bearer_token.is_some();
            Some(json!({
                "authenticated": has_token,
                "origin": format!("{:?}", caller.origin),
                "primal": "nestgate",
            }))
        }
        "auth.peer_info" => Some(json!({
            "origin": format!("{:?}", caller.origin),
            "peer_credentials": "deferred (SO_PEERCRED API unstable)",
            "primal": "nestgate",
        })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_methods_are_public() {
        assert_eq!(classify_method("health"), MethodAccessLevel::Public);
        assert_eq!(classify_method("health.check"), MethodAccessLevel::Public);
        assert_eq!(
            classify_method("health.liveness"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("health.readiness"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn identity_is_public() {
        assert_eq!(classify_method("identity.get"), MethodAccessLevel::Public);
    }

    #[test]
    fn capabilities_list_is_public() {
        assert_eq!(
            classify_method("capabilities.list"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("capability.list"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn discovery_methods_are_public() {
        assert_eq!(
            classify_method("discover_capabilities"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("discover.capabilities"),
            MethodAccessLevel::Public
        );
        assert_eq!(
            classify_method("discovery.capability.register"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn auth_introspection_is_public() {
        assert_eq!(classify_method("auth.check"), MethodAccessLevel::Public);
        assert_eq!(classify_method("auth.mode"), MethodAccessLevel::Public);
        assert_eq!(classify_method("auth.peer_info"), MethodAccessLevel::Public);
    }

    #[test]
    fn lifecycle_status_is_public() {
        assert_eq!(
            classify_method("lifecycle.status"),
            MethodAccessLevel::Public
        );
    }

    #[test]
    fn storage_methods_are_protected() {
        assert_eq!(
            classify_method("storage.store"),
            MethodAccessLevel::Protected
        );
        assert_eq!(
            classify_method("storage.retrieve"),
            MethodAccessLevel::Protected
        );
        assert_eq!(
            classify_method("storage.list"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn content_methods_are_protected() {
        assert_eq!(classify_method("content.put"), MethodAccessLevel::Protected);
        assert_eq!(classify_method("content.get"), MethodAccessLevel::Protected);
    }

    #[test]
    fn session_methods_are_protected() {
        assert_eq!(
            classify_method("session.create"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn empty_method_is_protected() {
        assert_eq!(classify_method(""), MethodAccessLevel::Protected);
    }

    #[test]
    fn unknown_method_is_protected() {
        assert_eq!(
            classify_method("bonding.propose"),
            MethodAccessLevel::Protected
        );
    }

    #[test]
    fn enforcement_mode_as_str() {
        assert_eq!(EnforcementMode::Permissive.as_str(), "permissive");
        assert_eq!(EnforcementMode::Enforced.as_str(), "enforced");
    }

    #[test]
    fn unix_context() {
        let ctx = CallerContext::unix();
        assert!(ctx.bearer_token.is_none());
        assert_eq!(ctx.origin, ConnectionOrigin::Unix);
    }

    #[test]
    fn public_method_always_passes_even_when_enforced() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext::unix();
        assert!(gate.check("health.check", &caller).is_ok());
        assert!(gate.check("identity.get", &caller).is_ok());
        assert!(gate.check("capabilities.list", &caller).is_ok());
        assert!(gate.check("auth.mode", &caller).is_ok());
    }

    #[test]
    fn protected_method_passes_in_permissive_mode() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext::unix();
        assert!(gate.check("storage.store", &caller).is_ok());
        assert!(gate.check("content.put", &caller).is_ok());
    }

    #[test]
    fn protected_method_rejected_in_enforced_mode_without_token() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext::unix();
        let result = gate.check("storage.store", &caller);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, error_codes::PERMISSION_DENIED);
        assert_eq!(err.method, "storage.store");
    }

    #[test]
    fn protected_method_passes_in_enforced_mode_with_token() {
        let gate = MethodGate::new(EnforcementMode::Enforced);
        let caller = CallerContext {
            bearer_token: Some("valid-token".to_owned()),
            origin: ConnectionOrigin::Unix,
        };
        assert!(gate.check("storage.store", &caller).is_ok());
    }

    #[test]
    fn auth_mode_response() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext::unix();
        let resp = auth_introspection("auth.mode", &gate, &caller).unwrap();
        assert_eq!(resp["mode"], "permissive");
        assert_eq!(resp["primal"], "nestgate");
    }

    #[test]
    fn auth_check_response() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext::unix();
        let resp = auth_introspection("auth.check", &gate, &caller).unwrap();
        assert_eq!(resp["authenticated"], false);
    }

    #[test]
    fn auth_check_with_token() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext {
            bearer_token: Some("tok".to_owned()),
            origin: ConnectionOrigin::Unix,
        };
        let resp = auth_introspection("auth.check", &gate, &caller).unwrap();
        assert_eq!(resp["authenticated"], true);
    }

    #[test]
    fn auth_peer_info_response() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext::unix();
        let resp = auth_introspection("auth.peer_info", &gate, &caller).unwrap();
        assert_eq!(resp["origin"], "Unix");
    }

    #[test]
    fn non_auth_method_returns_none() {
        let gate = MethodGate::new(EnforcementMode::Permissive);
        let caller = CallerContext::unix();
        assert!(auth_introspection("storage.store", &gate, &caller).is_none());
    }
}
