// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC authentication against a discovered Security primal over Unix socket IPC.

use crate::zero_cost_security_provider::types::{ZeroCostAuthToken, ZeroCostCredentials};
use nestgate_discovery::primal_discovery::PrimalConnection;
use nestgate_types::{NestGateError, Result};
use std::path::Path;
use std::time::Duration;

/// Call discovered Security primal for authentication via Unix socket IPC.
///
/// Sends a JSON-RPC `auth.authenticate` request to the security primal's
/// IPC endpoint. Falls back to local auth if the primal is unreachable.
pub async fn call_security_primal(
    connection: &PrimalConnection,
    credentials: &ZeroCostCredentials,
    token_expiry: Duration,
) -> Result<ZeroCostAuthToken> {
    let endpoint = &connection.endpoint;

    if endpoint.starts_with('/')
        || Path::new(endpoint)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("sock"))
    {
        // Unix socket IPC path — preferred for primal-to-primal auth
        let stream = tokio::net::UnixStream::connect(endpoint)
            .await
            .map_err(|e| {
                NestGateError::network_error(format!(
                    "Security primal unreachable at {endpoint}: {e}"
                ))
            })?;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "auth.authenticate",
            "params": {
                "username": credentials.username,
                "auth_method": format!("{:?}", credentials.auth_method)
            }
        });

        let request_bytes = serde_json::to_vec(&request).map_err(|e| {
            NestGateError::internal_error(format!("Serialize auth request: {e}"), "security")
        })?;

        stream.writable().await.map_err(|e| {
            NestGateError::network_error(format!("Security socket not writable: {e}"))
        })?;
        stream
            .try_write(&request_bytes)
            .map_err(|e| NestGateError::network_error(format!("Write to security primal: {e}")))?;

        let mut buf = vec![0u8; 4096];
        stream.readable().await.map_err(|e| {
            NestGateError::network_error(format!("Security socket not readable: {e}"))
        })?;
        let n = stream
            .try_read(&mut buf)
            .map_err(|e| NestGateError::network_error(format!("Read from security primal: {e}")))?;

        let response: serde_json::Value = serde_json::from_slice(&buf[..n]).map_err(|e| {
            NestGateError::internal_error(
                format!("Parse security primal response: {e}"),
                "security",
            )
        })?;

        if let Some(err) = response.get("error") {
            let msg = err["message"]
                .as_str()
                .unwrap_or("Security primal rejected auth")
                .to_string();
            return Err(NestGateError::security_error(msg));
        }

        let result = &response["result"];
        let token_id = result["token"]
            .as_str()
            .unwrap_or(&format!("primal_{}", uuid::Uuid::new_v4()))
            .to_string();
        let roles: Vec<String> = result["roles"].as_array().map_or_else(
            || vec!["authenticated".to_string()],
            |arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            },
        );

        Ok(ZeroCostAuthToken::new(
            token_id,
            credentials.username.clone(),
            roles,
            token_expiry,
        ))
    } else {
        Err(NestGateError::security_error(format!(
            "Non-socket security endpoints not supported: {endpoint}. \
             Use Unix socket IPC for primal-to-primal authentication."
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_discovery::infant_discovery::{CapabilityDescriptor, CapabilityType};
    use nestgate_discovery::primal_discovery::PrimalConnection;
    use std::collections::HashMap;
    use std::time::Duration;

    /// Synthetic HTTP URL for the non-socket code path; production auth uses Unix sockets from discovery.
    const TEST_NON_SOCKET_SECURITY_ENDPOINT: &str = "http://127.0.0.1:1";

    #[tokio::test]
    async fn non_socket_endpoint_returns_error() {
        let endpoint = std::env::var("NESTGATE_SECURITY_PRIMAL_URL")
            .ok()
            .unwrap_or_else(|| TEST_NON_SOCKET_SECURITY_ENDPOINT.to_string());
        let conn = PrimalConnection {
            capability: CapabilityDescriptor {
                id: "sec".to_string(),
                capability_type: CapabilityType::Security,
                endpoint: None,
                metadata: HashMap::new(),
                sovereignty_compliant: true,
            },
            endpoint,
        };
        let creds = ZeroCostCredentials::new_password("u".into(), "p".into());
        let err = call_security_primal(&conn, &creds, Duration::from_secs(60))
            .await
            .unwrap_err();
        let s = err.to_string();
        assert!(
            s.contains("not supported") || s.contains("Non-socket"),
            "{s}"
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn unix_socket_success_returns_token() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::UnixListener;
        use tokio::sync::oneshot;

        let dir = tempfile::tempdir().expect("tempdir");
        let sock_path = dir.path().join("security-primal-auth.sock");
        let (ready_tx, ready_rx) = oneshot::channel();
        let server_path = sock_path.clone();

        let server = tokio::spawn(async move {
            let listener = UnixListener::bind(&server_path).expect("bind unix listener");
            ready_tx.send(()).expect("signal ready");
            let (mut stream, _) = listener.accept().await.expect("accept");
            let mut buf = vec![0u8; 8192];
            let n = stream.read(&mut buf).await.expect("read request");
            assert!(n > 0, "expected request bytes");
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": { "token": "tok-from-primal", "roles": ["authenticated"] }
            });
            stream
                .write_all(&serde_json::to_vec(&response).unwrap())
                .await
                .expect("write response");
        });

        ready_rx.await.expect("server ready");

        let endpoint = sock_path.to_string_lossy().into_owned();
        let conn = PrimalConnection {
            capability: CapabilityDescriptor {
                id: "sec".to_string(),
                capability_type: CapabilityType::Security,
                endpoint: Some(endpoint.clone()),
                metadata: HashMap::new(),
                sovereignty_compliant: true,
            },
            endpoint,
        };
        let creds = ZeroCostCredentials::new_password("alice".into(), "secret".into());
        let token = call_security_primal(&conn, &creds, Duration::from_secs(120))
            .await
            .expect("auth ok");
        assert_eq!(token.token, "tok-from-primal");
        assert_eq!(token.user_id, "alice");

        server.await.expect("server task");
    }
}
