// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! TCP/HTTP bind address resolution helpers for the service command.
//!
//! Standalone HTTP bind and optional TCP JSON-RPC listen addresses are computed here so
//! [`super::service::ServiceManager`] can stay focused on lifecycle while logic remains unit-tested.

use std::net::SocketAddr;

use nestgate_core::constants::DEFAULT_API_PORT;
use nestgate_types::EnvSource;

use crate::commands::env::{env_port_if_set_source, tcp_jsonrpc_default_port_requested_source};
use crate::error::NestGateBinError;

/// Compute HTTP bind address and port for standalone mode (CLI + runtime defaults).
///
/// Used by [`crate::commands::service::ServiceManager::start_http_mode`] and unit-tested without binding sockets.
#[must_use]
pub fn resolve_standalone_http_bind(
    port: Option<u16>,
    bind: Option<&str>,
    listen: Option<SocketAddr>,
    default_api_port: u16,
    bind_all: bool,
    api_host: &str,
    bind_all_ipv4: &str,
) -> (String, u16, String) {
    listen.map_or_else(
        || {
            let http_port = port.unwrap_or(default_api_port);
            let bind_host = match bind {
                Some(b) => b.to_string(),
                None if bind_all => bind_all_ipv4.to_string(),
                None => api_host.to_string(),
            };
            let bind_addr = format!("{bind_host}:{http_port}");
            (bind_addr, http_port, bind_host)
        },
        |addr| {
            let host = addr.ip().to_string();
            (addr.to_string(), addr.port(), host)
        },
    )
}

/// Resolve TCP JSON-RPC listen address for socket-only daemon when `--port` and/or `--listen`
/// are set (`--listen` wins per `UniBin` v1.2).
pub fn tcp_jsonrpc_listen_addr(
    port: Option<u16>,
    bind: &str,
    listen: Option<SocketAddr>,
) -> Result<Option<SocketAddr>, NestGateBinError> {
    match (listen, port) {
        (Some(addr), _) => Ok(Some(addr)),
        (None, Some(p)) => {
            let s = format!("{bind}:{p}");
            s.parse().map(Some).map_err(|e| {
                NestGateBinError::service_init_error(
                    format!("Invalid TCP JSON-RPC bind address {s}: {e}"),
                    Some("tcp-jsonrpc-bind".into()),
                )
            })
        }
        (None, None) => Ok(None),
    }
}

/// Resolve optional TCP JSON-RPC listen address for socket-only mode (`nestgate server` without `--enable-http`).
///
/// Precedence: `--listen` → `--port` → explicit `NESTGATE_API_PORT` / `NESTGATE_HTTP_PORT` / `NESTGATE_PORT` →
/// `NESTGATE_JSONRPC_TCP` truthy with [`DEFAULT_API_PORT`] → no TCP (Unix socket only).
pub fn resolve_socket_only_tcp_listen_port(
    cli_port: Option<u16>,
    listen: Option<SocketAddr>,
    bind: &str,
    env: &(impl EnvSource + ?Sized),
) -> Result<Option<SocketAddr>, NestGateBinError> {
    let port = cli_port
        .or_else(|| env_port_if_set_source(env))
        .or_else(|| tcp_jsonrpc_default_port_requested_source(env).then_some(DEFAULT_API_PORT));
    tcp_jsonrpc_listen_addr(port, bind, listen)
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use nestgate_types::MapEnv;

    use super::{
        resolve_socket_only_tcp_listen_port, resolve_standalone_http_bind, tcp_jsonrpc_listen_addr,
    };

    #[test]
    fn resolve_standalone_http_bind_listen_overrides_cli_port_and_bind() {
        let listen: SocketAddr = "10.0.0.5:9000".parse().unwrap();
        let (addr, port, host) = resolve_standalone_http_bind(
            Some(80),
            Some("127.0.0.1"),
            Some(listen),
            8080,
            false,
            "localhost",
            "0.0.0.0",
        );
        assert_eq!(port, 9000);
        assert_eq!(host, "10.0.0.5");
        assert_eq!(addr, "10.0.0.5:9000");
    }

    #[test]
    fn resolve_standalone_http_bind_uses_cli_port_and_explicit_bind() {
        let (addr, port, host) = resolve_standalone_http_bind(
            Some(3000),
            Some("192.168.1.2"),
            None,
            8080,
            false,
            "127.0.0.1",
            "0.0.0.0",
        );
        assert_eq!(port, 3000);
        assert_eq!(host, "192.168.1.2");
        assert_eq!(addr, "192.168.1.2:3000");
    }

    #[test]
    fn resolve_standalone_http_bind_bind_all_uses_ipv4_wildcard() {
        let (addr, port, host) =
            resolve_standalone_http_bind(None, None, None, 8443, true, "127.0.0.1", "0.0.0.0");
        assert_eq!(port, 8443);
        assert_eq!(host, "0.0.0.0");
        assert_eq!(addr, "0.0.0.0:8443");
    }

    #[test]
    fn resolve_standalone_http_bind_no_cli_uses_default_api_port_and_api_host() {
        let (addr, port, host) =
            resolve_standalone_http_bind(None, None, None, 7777, false, "10.0.0.1", "0.0.0.0");
        assert_eq!(port, 7777);
        assert_eq!(host, "10.0.0.1");
        assert_eq!(addr, "10.0.0.1:7777");
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_none_when_no_port_or_listen() {
        assert_eq!(
            tcp_jsonrpc_listen_addr(None, "127.0.0.1", None).unwrap(),
            None
        );
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_port_and_bind() {
        let a = tcp_jsonrpc_listen_addr(Some(9090), "127.0.0.1", None)
            .unwrap()
            .expect("addr");
        assert_eq!(a, "127.0.0.1:9090".parse().unwrap());
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_listen_wins_over_port() {
        let listen: SocketAddr = "10.0.0.2:7777".parse().unwrap();
        assert_eq!(
            tcp_jsonrpc_listen_addr(Some(1111), "127.0.0.1", Some(listen)).unwrap(),
            Some(listen)
        );
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_ipv6_loopback_with_port() {
        // `bind` must be a valid `SocketAddr::parse` fragment; bare `::1` needs brackets.
        let a = tcp_jsonrpc_listen_addr(Some(4000), "[::1]", None)
            .unwrap()
            .expect("addr");
        assert_eq!(a, "[::1]:4000".parse().unwrap());
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_listen_ipv6_overrides_bind_and_port() {
        let listen: SocketAddr = "[::1]:8080".parse().unwrap();
        assert_eq!(
            tcp_jsonrpc_listen_addr(Some(9999), "127.0.0.1", Some(listen)).unwrap(),
            Some(listen)
        );
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_invalid_bind_returns_error() {
        assert!(tcp_jsonrpc_listen_addr(Some(80), "!!!not-a-valid-host!!!", None).is_err());
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_explicit_wildcard_bind() {
        let a = tcp_jsonrpc_listen_addr(Some(5555), "0.0.0.0", None)
            .unwrap()
            .expect("addr");
        assert_eq!(a, "0.0.0.0:5555".parse().unwrap());
    }

    #[test]
    fn tcp_jsonrpc_listen_addr_port_zero_is_valid_socket() {
        let a = tcp_jsonrpc_listen_addr(Some(0), "127.0.0.1", None)
            .unwrap()
            .expect("addr");
        assert_eq!(a.port(), 0);
    }

    #[test]
    fn resolve_socket_only_explicit_api_port_8080_enables_tcp() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "8080")]);
        let a = resolve_socket_only_tcp_listen_port(None, None, "127.0.0.1", &env)
            .unwrap()
            .expect("tcp");
        assert_eq!(a, "127.0.0.1:8080".parse().unwrap());
    }

    #[test]
    fn resolve_socket_only_opt_in_jsonrpc_tcp_uses_default_api_port() {
        use nestgate_core::constants::DEFAULT_API_PORT;

        let env = MapEnv::from([("NESTGATE_JSONRPC_TCP", "1")]);
        let a = resolve_socket_only_tcp_listen_port(None, None, "127.0.0.1", &env)
            .unwrap()
            .expect("tcp");
        assert_eq!(a.port(), DEFAULT_API_PORT);
    }

    #[test]
    fn resolve_socket_only_no_cli_no_env_returns_none() {
        let env = MapEnv::new();
        assert!(
            resolve_socket_only_tcp_listen_port(None, None, "127.0.0.1", &env)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn resolve_socket_only_cli_port_wins_over_opt_in_env() {
        use nestgate_core::constants::DEFAULT_API_PORT;

        let env = MapEnv::from([("NESTGATE_JSONRPC_TCP", "1"), ("NESTGATE_API_PORT", "9090")]);
        let a = resolve_socket_only_tcp_listen_port(Some(7777), None, "127.0.0.1", &env)
            .unwrap()
            .expect("tcp");
        assert_eq!(a.port(), 7777);
        assert_ne!(a.port(), DEFAULT_API_PORT);
        assert_ne!(a.port(), 9090);
    }
}
