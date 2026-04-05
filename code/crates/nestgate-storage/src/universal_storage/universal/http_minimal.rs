// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Minimal HTTP/1.1 client over `tokio::net::TcpStream` for [`super::adapter::UniversalStorageAdapter`].
//! Intentionally self-contained (no `nestgate-core`) to avoid a dependency cycle.

use nestgate_types::error::{NestGateError, Result};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

struct ParsedUrl {
    host: String,
    port: u16,
    path_query: String,
}

fn parse_http_url(url: &str) -> Result<ParsedUrl> {
    let stripped = url.strip_prefix("http://").ok_or_else(|| {
        NestGateError::api_error(
            "universal storage HTTP: only http:// URLs are supported (https requires TLS wiring)",
        )
    })?;

    let (host_port, path_part) = stripped.split_once('/').map_or_else(
        || (stripped, "/".to_string()),
        |(hp, p)| (hp, format!("/{p}")),
    );

    let (host, port) = host_port.split_once(':').map_or_else(
        || (host_port.to_string(), 80),
        |(h, p)| {
            let port = p.parse::<u16>().unwrap_or(80);
            (h.to_string(), port)
        },
    );

    Ok(ParsedUrl {
        host,
        port,
        path_query: path_part,
    })
}

fn join_url_path(base_path: &str, key: &str) -> String {
    let base = base_path.trim_end_matches('/');
    let k = key.trim_start_matches('/');
    if base.is_empty() || base == "/" {
        format!("/{k}")
    } else {
        format!("{base}/{k}")
    }
}

async fn send_request(
    method: &str,
    full_url: &str,
    body: Option<&[u8]>,
    timeout: Duration,
) -> Result<(u16, Vec<u8>)> {
    let parsed = parse_http_url(full_url)?;
    let addr = format!("{}:{}", parsed.host, parsed.port);

    let stream = tokio::time::timeout(timeout, TcpStream::connect(&addr))
        .await
        .map_err(|_| NestGateError::network_error("universal storage HTTP: connect timeout"))?
        .map_err(|e| {
            NestGateError::network_error(format!("universal storage HTTP: connect: {e}"))
        })?;

    let mut buf = Vec::with_capacity(512);
    buf.extend_from_slice(
        format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\n",
            method, parsed.path_query, parsed.host
        )
        .as_bytes(),
    );
    if let Some(b) = body {
        buf.extend_from_slice(
            format!(
                "Content-Type: application/octet-stream\r\nContent-Length: {}\r\n",
                b.len()
            )
            .as_bytes(),
        );
    }
    buf.extend_from_slice(b"Connection: close\r\n\r\n");
    if let Some(b) = body {
        buf.extend_from_slice(b);
    }

    let (mut reader, mut writer) = stream.into_split();
    writer
        .write_all(&buf)
        .await
        .map_err(|e| NestGateError::network_error(format!("universal storage HTTP: write: {e}")))?;
    writer.shutdown().await.ok();

    let mut response_buf = Vec::with_capacity(4096);
    tokio::time::timeout(timeout, reader.read_to_end(&mut response_buf))
        .await
        .map_err(|_| NestGateError::network_error("universal storage HTTP: read timeout"))?
        .map_err(|e| NestGateError::network_error(format!("universal storage HTTP: read: {e}")))?;

    parse_http_response(&response_buf)
}

fn parse_http_response(raw: &[u8]) -> Result<(u16, Vec<u8>)> {
    let header_end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| {
            NestGateError::network_error("universal storage HTTP: malformed response")
        })?;

    let header_str = std::str::from_utf8(&raw[..header_end]).map_err(|e| {
        NestGateError::network_error(format!("universal storage HTTP: header decode: {e}"))
    })?;

    let status_line = header_str.lines().next().unwrap_or("");
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let body = raw[header_end + 4..].to_vec();
    Ok((status_code, body))
}

/// Perform GET and return body on 2xx.
pub async fn http_get_object(full_url: &str, timeout: Duration) -> Result<Vec<u8>> {
    let (code, body) = send_request("GET", full_url, None, timeout).await?;
    if (200..300).contains(&code) {
        Ok(body)
    } else {
        Err(NestGateError::api_error(format!(
            "universal storage HTTP GET: unexpected status {code}"
        )))
    }
}

/// Perform PUT with raw body; treat 2xx as success.
pub async fn http_put_object(full_url: &str, data: &[u8], timeout: Duration) -> Result<()> {
    let (code, _) = send_request("PUT", full_url, Some(data), timeout).await?;
    if (200..300).contains(&code) {
        Ok(())
    } else {
        Err(NestGateError::api_error(format!(
            "universal storage HTTP PUT: unexpected status {code}"
        )))
    }
}

/// Perform DELETE; 2xx or 404 as success (idempotent delete).
pub async fn http_delete_object(full_url: &str, timeout: Duration) -> Result<()> {
    let (code, _) = send_request("DELETE", full_url, None, timeout).await?;
    if (200..300).contains(&code) || code == 404 {
        Ok(())
    } else {
        Err(NestGateError::api_error(format!(
            "universal storage HTTP DELETE: unexpected status {code}"
        )))
    }
}

/// GET and interpret body as newline-separated keys (relative to `prefix`).
pub async fn http_list_prefix(full_url: &str, timeout: Duration) -> Result<Vec<String>> {
    let (code, body) = send_request("GET", full_url, None, timeout).await?;
    if !(200..300).contains(&code) {
        return Err(NestGateError::api_error(format!(
            "universal storage HTTP LIST: unexpected status {code}"
        )));
    }
    let text = String::from_utf8(body).map_err(|e| {
        NestGateError::api_error(format!(
            "universal storage HTTP LIST: body is not UTF-8 (expected newline-separated keys): {e}"
        ))
    })?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(std::string::ToString::to_string)
        .collect())
}

/// Build a full object URL from `endpoint` base and object key.
pub fn object_url(endpoint: &str, key: &str) -> Result<String> {
    let parsed = parse_http_url(endpoint)?;
    let path = join_url_path(&parsed.path_query, key);
    Ok(format!("http://{}:{}{}", parsed.host, parsed.port, path))
}

/// URL for listing under a prefix (GET bucket/prefix/?list or plain prefix path).
pub fn list_url(endpoint: &str, prefix: &str) -> Result<String> {
    let parsed = parse_http_url(endpoint)?;
    let p = prefix.trim_start_matches('/');
    let path = if p.is_empty() {
        parsed.path_query.clone()
    } else {
        join_url_path(&parsed.path_query, p)
    };
    Ok(format!("http://{}:{}{}", parsed.host, parsed.port, path))
}
