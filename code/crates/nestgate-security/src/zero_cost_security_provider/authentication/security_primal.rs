// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! JSON-RPC authentication against a discovered Security primal over Unix socket IPC.

use crate::zero_cost_security_provider::types::{ZeroCostAuthToken, ZeroCostCredentials};
use nestgate_discovery::primal_discovery::PrimalConnection;
use nestgate_types::{NestGateError, Result};
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

    if endpoint.starts_with('/') || endpoint.ends_with(".sock") {
        // Unix socket IPC path — preferred for primal-to-primal auth
        let stream = tokio::net::UnixStream::connect(endpoint)
            .await
            .map_err(|e| {
                NestGateError::network_error(&format!(
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
            NestGateError::network_error(&format!("Security socket not writable: {e}"))
        })?;
        stream
            .try_write(&request_bytes)
            .map_err(|e| NestGateError::network_error(&format!("Write to security primal: {e}")))?;

        let mut buf = vec![0u8; 4096];
        stream.readable().await.map_err(|e| {
            NestGateError::network_error(&format!("Security socket not readable: {e}"))
        })?;
        let n = stream.try_read(&mut buf).map_err(|e| {
            NestGateError::network_error(&format!("Read from security primal: {e}"))
        })?;

        let response: serde_json::Value = serde_json::from_slice(&buf[..n]).map_err(|e| {
            NestGateError::internal_error(
                format!("Parse security primal response: {e}"),
                "security",
            )
        })?;

        if let Some(err) = response.get("error") {
            return Err(NestGateError::security_error(
                err["message"]
                    .as_str()
                    .unwrap_or("Security primal rejected auth"),
            ));
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
        Err(NestGateError::security_error(&format!(
            "Non-socket security endpoints not supported: {endpoint}. \
             Use Unix socket IPC for primal-to-primal authentication."
        )))
    }
}
