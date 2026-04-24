use super::*;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

struct MockHandler;

impl RpcHandler for MockHandler {
    fn handle_request(&self, _request: Value) -> Pin<Box<dyn Future<Output = Value> + Send + '_>> {
        Box::pin(async move {
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": "ok",
                "id": 1
            })
        })
    }
}

#[test]
fn test_server_creation() {
    let handler = Arc::new(MockHandler);
    let _server = IsomorphicIpcServer::new("test-service".to_string(), handler);
}

#[tokio::test]
async fn test_mock_handler_returns_valid_json_rpc() {
    let handler = MockHandler;
    let request = serde_json::json!({"jsonrpc": "2.0", "method": "test", "id": 1});
    let response = handler.handle_request(request).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["result"], "ok");
    assert_eq!(response["id"], 1);
}

#[test]
fn test_server_creation_with_different_service_names() {
    let handler = Arc::new(MockHandler);
    let _server1 = IsomorphicIpcServer::new("nestgate".to_string(), handler.clone());
    let _server2 = IsomorphicIpcServer::new("test-svc-123".to_string(), handler);
}

#[tokio::test]
async fn test_mock_handler_handles_empty_request() {
    let handler = MockHandler;
    let request = serde_json::json!({});
    let response = handler.handle_request(request).await;
    assert!(response.get("jsonrpc").is_some());
}

#[test]
fn get_socket_path_ends_with_service_sock() {
    let handler = Arc::new(MockHandler);
    let server = IsomorphicIpcServer::new("svc-name-test".to_string(), handler);
    let p = server.get_socket_path().expect("path");
    assert!(
        p.to_string_lossy().ends_with("svc-name-test.sock"),
        "got {p:?}"
    );
}

#[test]
fn prepare_socket_path_creates_parent_and_removes_stale_socket() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("nested").join("test.sock");
    IsomorphicIpcServer::prepare_socket_path(&sock).expect("prepare");
    assert!(sock.parent().expect("parent").is_dir());
    std::fs::write(&sock, b"x").expect("write stale socket file for test");
    IsomorphicIpcServer::prepare_socket_path(&sock).expect("prepare again");
    assert!(!sock.exists());
}

#[test]
fn get_socket_path_prefers_xdg_runtime_dir_when_set() {
    temp_env::with_vars([("XDG_RUNTIME_DIR", Some("/run/user/4242"))], || {
        let handler = Arc::new(MockHandler);
        let server = IsomorphicIpcServer::new("ipc-test-svc".to_string(), handler);
        let p = server.get_socket_path().expect("path");
        assert_eq!(p.to_string_lossy(), "/run/user/4242/ipc-test-svc.sock");
    });
}

#[cfg(unix)]
#[tokio::test]
async fn handle_unix_connection_exits_on_immediate_peer_close() {
    use tokio::time::{Duration, timeout};

    temp_env::async_with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None::<&str>),
        ],
        async {
            let (server_sock, client_sock) = tokio::net::UnixStream::pair().expect("unix pair");
            let handler = Arc::new(MockHandler);

            drop(client_sock);

            let server = IsomorphicIpcServer::handle_unix_connection(server_sock, handler);
            timeout(Duration::from_secs(2), server)
                .await
                .expect("server should finish")
                .expect("server loop");
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn handle_unix_connection_btsp_required_bypasses_handshake_when_json_first_byte() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    temp_env::async_with_vars(
        [
            ("FAMILY_ID", Some("custom-prod-family")),
            ("BIOMEOS_INSECURE", None::<&str>),
        ],
        async {
            let (server_sock, client_sock) = tokio::net::UnixStream::pair().expect("unix pair");
            let handler = Arc::new(MockHandler);

            let (mut read_half, mut write_half) = tokio::io::split(client_sock);
            let client = async {
                let req = br#"{"jsonrpc":"2.0","method":"demo","id":7}"#;
                write_half.write_all(req).await.expect("write");
                write_half.write_all(b"\n").await.expect("newline");
                write_half.shutdown().await.expect("shutdown write half");

                let mut buf_reader = BufReader::new(&mut read_half);
                let mut line = String::new();
                buf_reader
                    .read_line(&mut line)
                    .await
                    .expect("read response");
                let v: Value = serde_json::from_str(line.trim()).expect("json");
                assert_eq!(v["id"], 1);
                assert_eq!(v["result"], "ok");
            };

            let server = IsomorphicIpcServer::handle_unix_connection(server_sock, handler);

            tokio::join!(client, server).1.expect("server loop");
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn handle_unix_connection_serves_multiple_requests_on_one_session() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    temp_env::async_with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None::<&str>),
        ],
        async {
            let (server_sock, client_sock) = tokio::net::UnixStream::pair().expect("unix pair");
            let handler = Arc::new(MockHandler);

            let (mut read_half, mut write_half) = tokio::io::split(client_sock);
            let client = async {
                for id in [1_i64, 2_i64] {
                    let req = format!(r#"{{"jsonrpc":"2.0","method":"demo","id":{id}}}"#);
                    write_half.write_all(req.as_bytes()).await.expect("write");
                    write_half.write_all(b"\n").await.expect("newline");
                }
                write_half.shutdown().await.expect("shutdown write half");

                let mut buf_reader = BufReader::new(&mut read_half);
                for _ in 0..2 {
                    let mut line = String::new();
                    buf_reader
                        .read_line(&mut line)
                        .await
                        .expect("read response");
                    let v: Value = serde_json::from_str(line.trim()).expect("json");
                    assert_eq!(v["result"], "ok");
                    assert_eq!(v["id"], 1);
                }
            };

            let server = IsomorphicIpcServer::handle_unix_connection(server_sock, handler);

            tokio::join!(client, server).1.expect("server loop");
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn handle_unix_connection_returns_parse_error_for_invalid_json() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    temp_env::async_with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None::<&str>),
        ],
        async {
            let (server_sock, client_sock) = tokio::net::UnixStream::pair().expect("unix pair");
            let handler = Arc::new(MockHandler);

            let (read_c, mut write_c) = tokio::io::split(client_sock);
            let client = async {
                write_c.write_all(b"not-valid-json\n").await.expect("write");
                write_c.shutdown().await.expect("shutdown write half");

                let mut buf_reader = BufReader::new(read_c);
                let mut line = String::new();
                buf_reader.read_line(&mut line).await.expect("read line");
                let v: Value = serde_json::from_str(line.trim()).expect("json");
                assert_eq!(v["error"]["code"], -32700);
                assert_eq!(v["error"]["message"], "Parse error");
            };

            let server = IsomorphicIpcServer::handle_unix_connection(server_sock, handler);

            tokio::join!(client, server).1.expect("server loop");
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn handle_unix_connection_round_trips_json_rpc() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    temp_env::async_with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None::<&str>),
        ],
        async {
            let (server_sock, client_sock) = tokio::net::UnixStream::pair().expect("unix pair");
            let handler = Arc::new(MockHandler);

            let (mut read_half, mut write_half) = tokio::io::split(client_sock);
            let client = async {
                let req = br#"{"jsonrpc":"2.0","method":"demo","id":1}"#;
                write_half.write_all(req).await.expect("write");
                write_half.write_all(b"\n").await.expect("newline");
                write_half.shutdown().await.expect("shutdown write half");

                let mut buf_reader = BufReader::new(&mut read_half);
                let mut line = String::new();
                buf_reader
                    .read_line(&mut line)
                    .await
                    .expect("read response");
                let v: Value = serde_json::from_str(line.trim()).expect("json");
                assert_eq!(v["jsonrpc"], "2.0");
                assert_eq!(v["result"], "ok");
                assert_eq!(v["id"], 1);
            };

            let server = IsomorphicIpcServer::handle_unix_connection(server_sock, handler);

            tokio::join!(client, server).1.expect("server loop");
        },
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn handle_unix_connection_skips_empty_lines() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    temp_env::async_with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None::<&str>),
        ],
        async {
            let (server_sock, client_sock) = tokio::net::UnixStream::pair().expect("unix pair");
            let handler = Arc::new(MockHandler);

            let (mut read_half, mut write_half) = tokio::io::split(client_sock);
            let client = async {
                write_half.write_all(b"\n\n").await.expect("blank lines");
                let req = br#"{"jsonrpc":"2.0","method":"x","id":1}"#;
                write_half.write_all(req).await.expect("write");
                write_half.write_all(b"\n").await.expect("newline");
                write_half.shutdown().await.expect("shutdown write half");

                let mut buf_reader = BufReader::new(&mut read_half);
                let mut line = String::new();
                buf_reader
                    .read_line(&mut line)
                    .await
                    .expect("read response");
                let v: Value = serde_json::from_str(line.trim()).expect("json");
                assert_eq!(v["result"], "ok");
            };

            let server = IsomorphicIpcServer::handle_unix_connection(server_sock, handler);

            tokio::join!(client, server).1.expect("server loop");
        },
    )
    .await;
}

#[test]
fn connection_idle_limit_matches_five_minute_policy() {
    assert_eq!(
        IsomorphicIpcServer::CONNECTION_IDLE_LIMIT.as_secs(),
        300,
        "idle policy is documented as 300s for abandoned connections"
    );
}

#[tokio::test(start_paused = true)]
async fn json_rpc_keep_alive_loop_sends_idle_close_notification() {
    use tokio::io::{AsyncBufReadExt, BufReader, split};

    let (client_end, server_end) = tokio::io::duplex(16_384);
    let (read_half, mut write_half) = split(server_end);
    let mut buf_reader = BufReader::new(read_half);
    let handler: Arc<dyn RpcHandler> = Arc::new(MockHandler);

    let server = tokio::spawn(async move {
        match IsomorphicIpcServer::json_rpc_keep_alive_loop(
            &mut buf_reader,
            &mut write_half,
            &handler,
        )
        .await
        {
            Ok(()) => {}
            Err(e) => panic!("keep-alive loop: {e}"),
        }
    });

    tokio::time::advance(
        IsomorphicIpcServer::CONNECTION_IDLE_LIMIT + std::time::Duration::from_secs(1),
    )
    .await;
    tokio::task::yield_now().await;

    let mut client_read = BufReader::new(client_end);
    let mut line = String::new();
    let n = match client_read.read_line(&mut line).await {
        Ok(n) => n,
        Err(e) => panic!("read_line: {e}"),
    };
    assert!(n > 0, "expected idle close notification");
    let v: serde_json::Value = match serde_json::from_str(line.trim()) {
        Ok(v) => v,
        Err(e) => panic!("parse JSON: {e}"),
    };
    assert_eq!(v["method"], "connection.closing");
    let params = match v["params"].as_object() {
        Some(p) => p,
        None => panic!("expected params object"),
    };
    assert_eq!(params.get("reason"), Some(&serde_json::json!("idle")));

    server.abort();
}
