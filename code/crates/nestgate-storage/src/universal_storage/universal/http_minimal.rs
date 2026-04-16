// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Minimal HTTP/1.1 client over `tokio::net::TcpStream` for [`super::adapter::UniversalStorageAdapter`].
//! Intentionally self-contained (no `nestgate-core`) to avoid a dependency cycle.

use nestgate_types::error::{NestGateError, Result};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    #[test]
    fn parse_http_url_rejects_non_http() {
        let Err(err) = parse_http_url("https://example.com/x") else {
            panic!("expected https URL to be rejected");
        };
        assert!(
            err.to_string().contains("http://") || err.to_string().contains("TLS"),
            "{err}"
        );
    }

    #[test]
    fn parse_http_url_host_default_port_and_path() {
        let p = parse_http_url("http://127.0.0.1").expect("parse");
        assert_eq!(p.host, "127.0.0.1");
        assert_eq!(p.port, 80);
        assert_eq!(p.path_query, "/");

        let p2 = parse_http_url("http://example.org:9000/foo/bar?x=1").expect("parse2");
        assert_eq!(p2.port, 9000);
        assert_eq!(p2.path_query, "/foo/bar?x=1");
    }

    #[test]
    fn join_url_path_edge_cases() {
        assert_eq!(join_url_path("/", "k"), "/k");
        assert_eq!(join_url_path("", "k"), "/k");
        assert_eq!(join_url_path("/api/v1", "obj"), "/api/v1/obj");
    }

    #[test]
    fn parse_http_response_ok_and_malformed() {
        let raw = b"HTTP/1.1 201 Created\r\nContent-Length: 0\r\n\r\n";
        let (code, body) = parse_http_response(raw).expect("ok");
        assert_eq!(code, 201);
        assert!(body.is_empty());

        let err = parse_http_response(b"nope").expect_err("no header end");
        assert!(
            err.to_string().contains("malformed") || err.to_string().contains("Malformed"),
            "{err}"
        );
    }

    #[test]
    fn object_and_list_url_builders() {
        let u = object_url("http://localhost:8080/prefix/", "my/key").expect("object_url");
        assert_eq!(u, "http://localhost:8080/prefix/my/key");
        let l = list_url("http://h:1/base/", "pre").expect("list_url");
        assert_eq!(l, "http://h:1/base/pre");
        let l2 = list_url("http://h:1/base/", "").expect("empty prefix");
        assert_eq!(l2, "http://h:1/base/");
    }

    async fn read_http_request_head<R: tokio::io::AsyncReadExt + Unpin>(mut stream: R) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut chunk = [0_u8; 256];
        loop {
            let n = tokio::io::AsyncReadExt::read(&mut stream, &mut chunk)
                .await
                .expect("read");
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
            if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        buf
    }

    #[tokio::test]
    async fn http_get_put_delete_and_list_integration() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");

        let server = tokio::spawn(async move {
            for _ in 0..5 {
                let (stream, _) = listener.accept().await.expect("accept");
                let (read_h, mut write_h) = stream.into_split();
                let buf = read_http_request_head(read_h).await;
                let head = String::from_utf8_lossy(&buf);
                let first = head.lines().next().unwrap_or("");

                if first.starts_with("GET /keys") {
                    let body = b"a\nb\n";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    write_h.write_all(resp.as_bytes()).await.expect("w");
                    write_h.write_all(body).await.expect("wb");
                } else if first.starts_with("GET /bin") {
                    let resp = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n";
                    write_h.write_all(resp.as_bytes()).await.expect("w");
                    write_h.write_all(&[0xFF, 0xFE]).await.expect("bad utf8");
                } else if first.starts_with("DELETE ") {
                    let resp =
                        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    write_h.write_all(resp.as_bytes()).await.expect("w");
                } else if first.starts_with("PUT ") {
                    let resp =
                        "HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    write_h.write_all(resp.as_bytes()).await.expect("w");
                } else {
                    let body = b"hello";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    write_h.write_all(resp.as_bytes()).await.expect("w");
                    write_h.write_all(body).await.expect("wb");
                }
                let _ = write_h.shutdown().await;
            }
        });

        let base = format!("http://127.0.0.1:{}/", addr.port());
        let t = Duration::from_secs(2);

        let body = http_get_object(&(base.clone() + "obj"), t)
            .await
            .expect("get");
        assert_eq!(body, b"hello");

        http_put_object(&(base.clone() + "put"), b"data", t)
            .await
            .expect("put");

        http_delete_object(&(base.clone() + "gone"), t)
            .await
            .expect("delete 404");

        let keys = http_list_prefix(&(base.clone() + "keys"), t)
            .await
            .expect("list");
        assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);

        let err = http_list_prefix(&(base.clone() + "bin"), t)
            .await
            .expect_err("utf8");
        assert!(
            err.to_string().contains("UTF-8") || err.to_string().contains("utf"),
            "{err}"
        );

        server.await.expect("server task");
    }
}
