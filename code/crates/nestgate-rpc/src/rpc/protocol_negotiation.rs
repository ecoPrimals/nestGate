// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! G65 Protocol Negotiation (Phase 3 Cephalization)
//!
//! Enables automatic protocol selection between JSON-RPC and tarpc at
//! connection time on a **single socket**, replacing the C2 dual-socket
//! pattern (`.sock` + `.tarpc.sock`).
//!
//! ## Wire Protocol
//!
//! ```text
//! Client → Server: "PROTOCOLS: tarpc,jsonrpc\n"
//! Server → Client: "PROTOCOL: tarpc\n"
//! [Connection proceeds in selected protocol]
//! ```
//!
//! ## Backward Compatibility
//!
//! If the client does not send a `PROTOCOLS:` line within 100 ms, the server
//! assumes JSON-RPC. Existing clients work with **zero changes**.
//!
//! ## Reference
//!
//! Convergent evolution from squirrel's G65 reference implementation.
//! See `wateringHole/specs/PROTOCOL_NEGOTIATION_SPEC.md`.

use super::ipc_protocol::IpcProtocol;
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tracing::{debug, info};

/// G65 negotiation timeout — legacy clients that never send a negotiation
/// line are assumed to speak JSON-RPC after this window.
pub const NEGOTIATION_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);

/// Maximum length of a `PROTOCOLS:` line before the server rejects it.
const MAX_NEGOTIATION_LINE_LEN: usize = 256;

/// G65 protocol negotiation request from a client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolRequest {
    /// Protocols supported by the client (in preference order).
    pub supported: Vec<IpcProtocol>,
}

impl ProtocolRequest {
    /// Create a new protocol request.
    #[must_use]
    pub const fn new(supported: Vec<IpcProtocol>) -> Self {
        Self { supported }
    }

    /// Request with all protocols supported by this build.
    #[must_use]
    pub fn all_supported() -> Self {
        Self {
            supported: IpcProtocol::supported(),
        }
    }

    /// Serialize to G65 wire format: `"PROTOCOLS: tarpc,jsonrpc\n"`.
    #[must_use]
    pub fn to_wire(&self) -> String {
        let names: Vec<&str> = self
            .supported
            .iter()
            .map(IpcProtocol::negotiation_name)
            .collect();
        format!("PROTOCOLS: {}\n", names.join(","))
    }

    /// Parse from G65 wire format.
    ///
    /// # Errors
    ///
    /// Returns an error when the line does not start with `PROTOCOLS: ` or
    /// contains no recognised protocol names.
    pub fn from_wire(line: &str) -> Result<Self> {
        let line = line.trim();
        let protocols_str = line
            .strip_prefix("PROTOCOLS: ")
            .ok_or_else(|| anyhow::anyhow!("Invalid protocol request: {line}"))?;

        let mut supported = Vec::new();
        for name in protocols_str.split(',') {
            if let Some(proto) = IpcProtocol::from_str(name.trim()) {
                supported.push(proto);
            }
        }

        if supported.is_empty() {
            anyhow::bail!("No valid protocols in request");
        }

        Ok(Self { supported })
    }
}

/// G65 protocol negotiation response from the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResponse {
    /// The selected protocol.
    pub selected: IpcProtocol,
}

impl ProtocolResponse {
    /// Create a new protocol response.
    #[must_use]
    pub const fn new(selected: IpcProtocol) -> Self {
        Self { selected }
    }

    /// Serialize to G65 wire format: `"PROTOCOL: tarpc\n"`.
    #[must_use]
    pub fn to_wire(&self) -> String {
        format!("PROTOCOL: {}\n", self.selected.negotiation_name())
    }

    /// Parse from G65 wire format.
    ///
    /// # Errors
    ///
    /// Returns an error when the line does not start with `PROTOCOL: ` or
    /// the protocol name is unrecognised.
    pub fn from_wire(line: &str) -> Result<Self> {
        let line = line.trim();
        let proto_name = line
            .strip_prefix("PROTOCOL: ")
            .ok_or_else(|| anyhow::anyhow!("Invalid protocol response: {line}"))?;
        let selected = IpcProtocol::from_str(proto_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown protocol: {proto_name}"))?;
        Ok(Self { selected })
    }
}

/// Negotiate protocol from the **client** side.
///
/// Sends the supported-protocol list and waits for the server's selection.
///
/// # Errors
///
/// Returns an error on I/O failure or if the server response is malformed.
pub async fn negotiate_client<T>(
    transport: &mut T,
    supported: Vec<IpcProtocol>,
) -> Result<IpcProtocol>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let request = ProtocolRequest::new(supported);
    let wire = request.to_wire();

    debug!("G65 client sending: {:?}", request);
    transport
        .write_all(wire.as_bytes())
        .await
        .context("Failed to send G65 protocol request")?;
    transport
        .flush()
        .await
        .context("Failed to flush G65 protocol request")?;

    let mut reader = BufReader::new(transport);
    let mut response_line = String::new();
    reader
        .read_line(&mut response_line)
        .await
        .context("Failed to read G65 protocol response")?;

    let response =
        ProtocolResponse::from_wire(&response_line).context("Failed to parse G65 response")?;

    info!("G65 client negotiated: {}", response.selected);
    Ok(response.selected)
}

/// Select the best mutual protocol (client preference wins).
///
/// Returns the first client-preferred protocol that the server also supports.
/// Falls back to `JsonRpc` if no intersection exists.
#[must_use]
pub fn select_protocol(
    client_supported: &[IpcProtocol],
    server_supported: &[IpcProtocol],
) -> IpcProtocol {
    for proto in client_supported {
        if server_supported.contains(proto) {
            return *proto;
        }
    }
    IpcProtocol::JsonRpc
}

/// Server-side G65 protocol negotiation on an accepted [`TransportStream`].
///
/// Peeks the first byte (100 ms timeout). If `b'P'`, reads the
/// `PROTOCOLS:` line byte-by-byte, selects the best mutual protocol, and
/// writes the `PROTOCOL:` response. Returns `None` when no negotiation
/// occurred (legacy client or timeout).
///
/// `tarpc_available` indicates whether the server can serve tarpc connections.
/// When `false`, the server will only offer JSON-RPC during negotiation.
pub async fn try_g65_server_negotiation(
    stream: &mut super::isomorphic_ipc::TransportStream,
    tarpc_available: bool,
) -> Option<IpcProtocol> {
    use tokio::io::AsyncWriteExt;
    use tracing::warn;

    let mut peek_buf = [0u8; 1];
    let first_byte = match tokio::time::timeout(NEGOTIATION_TIMEOUT, stream.peek(&mut peek_buf))
        .await
    {
        Ok(Ok(n)) if n > 0 => peek_buf[0],
        _ => return None,
    };

    if first_byte != b'P' {
        return None;
    }

    let line = match read_negotiation_line(stream).await {
        Ok(l) => l,
        Err(e) => {
            warn!("G65 negotiation line read failed: {e}");
            return None;
        }
    };

    let request = match ProtocolRequest::from_wire(&line) {
        Ok(r) => r,
        Err(e) => {
            warn!("Invalid G65 protocol request: {e}");
            let _ = stream.write_all(b"PROTOCOL: jsonrpc\n").await;
            let _ = stream.flush().await;
            return Some(IpcProtocol::JsonRpc);
        }
    };

    let server_supported = if tarpc_available {
        vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc]
    } else {
        vec![IpcProtocol::JsonRpc]
    };

    let selected = select_protocol(&request.supported, &server_supported);
    let response = ProtocolResponse::new(selected);

    if let Err(e) = stream.write_all(response.to_wire().as_bytes()).await {
        warn!("G65 response write failed: {e}");
        return None;
    }
    let _ = stream.flush().await;

    info!("G65 protocol negotiated: {selected}");
    Some(selected)
}

/// Read a single newline-terminated line **byte-by-byte** from the stream.
///
/// This avoids `BufReader` read-ahead, ensuring no bytes beyond the line
/// are consumed. Used by the server-side negotiation path so the remaining
/// stream is clean for the selected protocol's framing.
///
/// # Errors
///
/// Returns an error on I/O failure or if the line exceeds
/// [`MAX_NEGOTIATION_LINE_LEN`].
pub(crate) async fn read_negotiation_line<T: AsyncRead + Unpin>(
    stream: &mut T,
) -> Result<String> {
    use tokio::io::AsyncReadExt;

    let mut buf = Vec::with_capacity(64);
    let mut byte = [0u8; 1];

    loop {
        let n = stream.read(&mut byte).await.context("G65 line read")?;
        if n == 0 {
            break;
        }
        buf.push(byte[0]);
        if byte[0] == b'\n' {
            break;
        }
        if buf.len() > MAX_NEGOTIATION_LINE_LEN {
            anyhow::bail!("G65 negotiation line exceeds {MAX_NEGOTIATION_LINE_LEN} bytes");
        }
    }

    String::from_utf8(buf).map_err(|e| anyhow::anyhow!("Invalid UTF-8 in G65 line: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_wire_format_single() {
        let req = ProtocolRequest::new(vec![IpcProtocol::JsonRpc]);
        assert_eq!(req.to_wire(), "PROTOCOLS: jsonrpc\n");
    }

    #[test]
    fn request_wire_format_multi() {
        let req = ProtocolRequest::new(vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc]);
        assert_eq!(req.to_wire(), "PROTOCOLS: tarpc,jsonrpc\n");
    }

    #[test]
    fn request_parse_single() {
        let req = ProtocolRequest::from_wire("PROTOCOLS: jsonrpc\n").expect("parse");
        assert_eq!(req.supported, vec![IpcProtocol::JsonRpc]);
    }

    #[test]
    fn request_parse_multi() {
        let req = ProtocolRequest::from_wire("PROTOCOLS: tarpc,jsonrpc\n").expect("parse");
        assert_eq!(
            req.supported,
            vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc]
        );
    }

    #[test]
    fn response_wire_roundtrip() {
        for proto in IpcProtocol::supported() {
            let resp = ProtocolResponse::new(proto);
            let wire = resp.to_wire();
            let parsed = ProtocolResponse::from_wire(&wire).expect("roundtrip");
            assert_eq!(parsed.selected, proto);
        }
    }

    #[test]
    fn select_protocol_client_preference_wins() {
        let client = vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc];
        let server = vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc];
        assert_eq!(select_protocol(&client, &server), IpcProtocol::Tarpc);
    }

    #[test]
    fn select_protocol_server_only_jsonrpc() {
        let client = vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc];
        let server = vec![IpcProtocol::JsonRpc];
        assert_eq!(select_protocol(&client, &server), IpcProtocol::JsonRpc);
    }

    #[test]
    fn select_protocol_no_match_falls_back() {
        let client = vec![IpcProtocol::Tarpc];
        let server = vec![IpcProtocol::JsonRpc];
        assert_eq!(select_protocol(&client, &server), IpcProtocol::JsonRpc);
    }

    #[test]
    fn request_invalid_prefix_errors() {
        assert!(ProtocolRequest::from_wire("NOT_PROTOCOLS: jsonrpc\n").is_err());
    }

    #[test]
    fn request_no_valid_protocols_errors() {
        assert!(ProtocolRequest::from_wire("PROTOCOLS: unknown\n").is_err());
    }

    #[test]
    fn response_invalid_format_errors() {
        assert!(ProtocolResponse::from_wire("STATUS: ok\n").is_err());
    }

    #[test]
    fn all_supported_non_empty() {
        let req = ProtocolRequest::all_supported();
        assert!(req.supported.contains(&IpcProtocol::JsonRpc));
        assert!(req.supported.contains(&IpcProtocol::Tarpc));
    }

    #[tokio::test]
    async fn negotiate_duplex_jsonrpc() {
        let (mut client, mut server) = tokio::io::duplex(4096);
        let server_supported = IpcProtocol::supported();

        let server_task = tokio::spawn(async move {
            let mut reader = BufReader::new(&mut server);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            let req = ProtocolRequest::from_wire(&line).unwrap();
            let selected = select_protocol(&req.supported, &server_supported);
            let resp = ProtocolResponse::new(selected);
            reader
                .get_mut()
                .write_all(resp.to_wire().as_bytes())
                .await
                .unwrap();
            reader.get_mut().flush().await.unwrap();
            selected
        });

        let client_task = tokio::spawn(async move {
            negotiate_client(&mut client, vec![IpcProtocol::JsonRpc]).await
        });

        let selected = client_task.await.expect("join").expect("negotiate");
        assert_eq!(selected, IpcProtocol::JsonRpc);
        let srv = server_task.await.expect("join");
        assert_eq!(srv, IpcProtocol::JsonRpc);
    }

    #[tokio::test]
    async fn negotiate_duplex_tarpc() {
        let (mut client, mut server) = tokio::io::duplex(4096);
        let server_supported = IpcProtocol::supported();

        let server_task = tokio::spawn(async move {
            let mut reader = BufReader::new(&mut server);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            let req = ProtocolRequest::from_wire(&line).unwrap();
            let selected = select_protocol(&req.supported, &server_supported);
            let resp = ProtocolResponse::new(selected);
            reader
                .get_mut()
                .write_all(resp.to_wire().as_bytes())
                .await
                .unwrap();
            reader.get_mut().flush().await.unwrap();
            selected
        });

        let client_task = tokio::spawn(async move {
            negotiate_client(
                &mut client,
                vec![IpcProtocol::Tarpc, IpcProtocol::JsonRpc],
            )
            .await
        });

        let selected = client_task.await.expect("join").expect("negotiate");
        assert_eq!(selected, IpcProtocol::Tarpc);
        let srv = server_task.await.expect("join");
        assert_eq!(srv, IpcProtocol::Tarpc);
    }

    #[tokio::test]
    async fn read_negotiation_line_reads_exactly_one_line() {
        let data = b"PROTOCOLS: tarpc,jsonrpc\ngarbage after";
        let mut cursor = &data[..];
        let line = read_negotiation_line(&mut cursor).await.unwrap();
        assert_eq!(line, "PROTOCOLS: tarpc,jsonrpc\n");
    }

    #[tokio::test]
    async fn read_negotiation_line_rejects_overlong() {
        let data = vec![b'A'; MAX_NEGOTIATION_LINE_LEN + 2];
        let mut cursor = &data[..];
        assert!(read_negotiation_line(&mut cursor).await.is_err());
    }
}
