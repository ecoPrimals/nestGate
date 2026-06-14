// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared JSON-RPC protocol helpers (method name normalization, riboCipher
//! signal acceptance, etc.).

use std::borrow::Cow;
use std::time::Duration;
use tokio::io::AsyncBufReadExt;

/// Maximum idle time before a keep-alive JSON-RPC connection is closed.
///
/// The timer resets on every successful request so active connections are never
/// reaped. Only truly idle (half-open, abandoned) connections are affected.
/// Shared by UDS, isomorphic IPC, and TCP fallback connection handlers.
pub const CONNECTION_IDLE_LIMIT: Duration = Duration::from_secs(300);

/// Default TTL for capability announcements via `announce_capability`.
///
/// Used by `discovery.capability.register`, `route.register`, and tarpc
/// `register_capability` so that all announcement paths share a single
/// source of truth.
pub const CAPABILITY_ANNOUNCE_TTL: Duration = Duration::from_secs(60);

/// riboCipher ecosystem signal prefix.
///
/// `cellMembrane` probes send `[0xEC, 0x01]` before the JSON-RPC payload to
/// identify ecosystem-aware connections. Primals must accept and discard this
/// prefix (guideStone amendment: "riboCipher signal handling MANDATORY on UDS").
pub const RIBOCIPHER_PREFIX: [u8; 2] = [0xEC, 0x01];

/// Peek at the first bytes of a buffered reader and consume the riboCipher
/// signal prefix `[0xEC, 0x01]` if present. A no-op for plain JSON-RPC
/// clients that start with `{`.
pub async fn strip_ribocipher_prefix<R: AsyncBufReadExt + Unpin>(reader: &mut R) {
    match reader.fill_buf().await {
        Ok(buf) if buf.len() >= 2 && buf[0] == RIBOCIPHER_PREFIX[0] && buf[1] == RIBOCIPHER_PREFIX[1] => {
            tracing::debug!("riboCipher signal [0xEC, 0x01] accepted, stripping prefix");
            reader.consume(2);
        }
        _ => {}
    }
}

/// Normalize a JSON-RPC method name by stripping legacy prefixes.
///
/// Older clients may send `nestgate.storage.store` instead of `storage.store`.
/// This function strips the `nestgate.` prefix (repeatedly, defensively) so
/// the dispatch table only needs canonical short-form entries.
#[must_use]
pub fn normalize_method(method: &str) -> Cow<'_, str> {
    let mut s = method;
    while let Some(rest) = s.strip_prefix("nestgate.") {
        s = rest;
    }
    if s.len() == method.len() {
        Cow::Borrowed(method)
    } else {
        Cow::Borrowed(s)
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_method, strip_ribocipher_prefix, RIBOCIPHER_PREFIX};
    use std::borrow::Cow;

    #[test]
    fn normalize_leaves_canonical_methods_unchanged() {
        let m = "storage.object.store";
        match normalize_method(m) {
            Cow::Borrowed(s) => assert_eq!(s, m),
            Cow::Owned(_) => panic!("expected borrowed unchanged slice"),
        }
    }

    #[test]
    fn normalize_strips_single_legacy_prefix() {
        match normalize_method("nestgate.health.check") {
            Cow::Borrowed(s) => assert_eq!(s, "health.check"),
            Cow::Owned(_) => panic!("expected borrowed normalized slice"),
        }
    }

    #[test]
    fn normalize_strips_repeated_nestgate_prefixes() {
        match normalize_method("nestgate.nestgate.storage.store") {
            Cow::Borrowed(s) => assert_eq!(s, "storage.store"),
            Cow::Owned(_) => panic!("expected borrowed normalized slice"),
        }
    }

    #[test]
    fn normalize_empty_string() {
        match normalize_method("") {
            Cow::Borrowed(s) => assert_eq!(s, ""),
            Cow::Owned(_) => panic!("expected borrowed empty"),
        }
    }

    #[test]
    fn normalize_does_not_strip_non_prefix() {
        let m = "myapp.nestgate.service";
        match normalize_method(m) {
            Cow::Borrowed(s) => assert_eq!(s, m),
            Cow::Owned(_) => panic!("expected unchanged borrow"),
        }
    }

    #[test]
    fn normalize_only_nestgate_dot_is_stripped_completely() {
        match normalize_method("nestgate.") {
            Cow::Borrowed(s) => assert_eq!(s, ""),
            Cow::Owned(_) => panic!("expected borrowed empty remainder"),
        }
    }

    #[test]
    fn connection_idle_limit_is_five_minutes() {
        assert_eq!(
            super::CONNECTION_IDLE_LIMIT,
            std::time::Duration::from_secs(300),
        );
    }

    #[test]
    fn capability_announce_ttl_is_one_minute() {
        assert_eq!(
            super::CAPABILITY_ANNOUNCE_TTL,
            std::time::Duration::from_secs(60),
        );
    }

    #[test]
    fn ribocipher_prefix_constant() {
        assert_eq!(RIBOCIPHER_PREFIX, [0xEC, 0x01]);
    }

    #[tokio::test]
    async fn strip_ribocipher_consumes_prefix() {
        let data: &[u8] = &[0xEC, 0x01, b'{', b'"', b'j', b's', b'o', b'n', b'"'];
        let mut cursor = tokio::io::BufReader::new(data);
        strip_ribocipher_prefix(&mut cursor).await;

        use tokio::io::AsyncBufReadExt;
        let remaining = cursor.fill_buf().await.unwrap();
        assert_eq!(remaining[0], b'{', "JSON payload should start after strip");
    }

    #[tokio::test]
    async fn strip_ribocipher_noop_on_plain_json() {
        let data: &[u8] = b"{\"jsonrpc\":\"2.0\"}\n";
        let mut cursor = tokio::io::BufReader::new(data);
        strip_ribocipher_prefix(&mut cursor).await;

        use tokio::io::AsyncBufReadExt;
        let remaining = cursor.fill_buf().await.unwrap();
        assert_eq!(remaining[0], b'{', "plain JSON should be untouched");
        assert_eq!(remaining.len(), data.len(), "no bytes consumed");
    }

    #[tokio::test]
    async fn strip_ribocipher_noop_on_empty_stream() {
        let data: &[u8] = &[];
        let mut cursor = tokio::io::BufReader::new(data);
        strip_ribocipher_prefix(&mut cursor).await;

        use tokio::io::AsyncBufReadExt;
        let remaining = cursor.fill_buf().await.unwrap();
        assert!(remaining.is_empty());
    }

    #[tokio::test]
    async fn strip_ribocipher_noop_on_single_byte() {
        let data: &[u8] = &[0xEC];
        let mut cursor = tokio::io::BufReader::new(data);
        strip_ribocipher_prefix(&mut cursor).await;

        use tokio::io::AsyncBufReadExt;
        let remaining = cursor.fill_buf().await.unwrap();
        assert_eq!(remaining, &[0xEC], "single 0xEC byte should not be consumed");
    }

    #[tokio::test]
    async fn strip_ribocipher_noop_on_wrong_second_byte() {
        let data: &[u8] = &[0xEC, 0x02, b'{'];
        let mut cursor = tokio::io::BufReader::new(data);
        strip_ribocipher_prefix(&mut cursor).await;

        use tokio::io::AsyncBufReadExt;
        let remaining = cursor.fill_buf().await.unwrap();
        assert_eq!(remaining[0], 0xEC, "wrong second byte should not trigger strip");
        assert_eq!(remaining.len(), 3);
    }

    #[tokio::test]
    async fn strip_ribocipher_full_jsonrpc_after_prefix() {
        let mut payload = vec![0xEC, 0x01];
        payload.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"method\":\"health\",\"id\":1}\n");
        let mut cursor = tokio::io::BufReader::new(payload.as_slice());
        strip_ribocipher_prefix(&mut cursor).await;

        use tokio::io::AsyncBufReadExt;
        let mut line = String::new();
        cursor.read_line(&mut line).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(parsed["method"], "health");
    }
}
