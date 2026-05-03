# Transport Layer

**Status**: Production Ready
**Version**: 1.0.0
**Updated**: Session 50, April 30, 2026

---

## Overview

Complete implementation of primal transport architecture for NestGate:
- **Unix Sockets**: Port-free IPC
- **JSON-RPC 2.0**: Universal, simple protocol
- **Security provider**: Capability-based crypto (runtime discovery)
- **HTTP Fallback**: Optional (debugging only)

---

## Module Structure

```
transport/
├── mod.rs           - Module orchestration & exports
├── config.rs        - Environment-driven configuration
├── unix_socket.rs   - Unix socket listener
├── jsonrpc.rs       - JSON-RPC 2.0 protocol
├── handlers.rs      - RPC method implementations
├── security.rs      - Security provider client (capability discovery)
├── server.rs        - Main transport server
└── README.md        - This file
```

---

## Quick Start

### Basic Server

```rust
use nestgate_api::transport::{TransportConfig, TransportServer, NestGateRpcHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TransportConfig::from_env()?;
    let handler = NestGateRpcHandler::new();
    let server = TransportServer::new(config, handler)?;
    server.start().await?;
    Ok(())
}
```

### Environment Variables

```bash
export NESTGATE_FAMILY_ID="nat0"
export NESTGATE_SOCKET_PATH="/tmp/nestgate-nat0.sock"
export NESTGATE_SECURITY_PROVIDER="/run/user/1000/biomeos/security.sock"
# export NESTGATE_HTTP_PORT="8080"  # Optional HTTP fallback
```

---

## Client Usage

### Using `socat`

```bash
# Ping
echo '{"jsonrpc":"2.0","method":"health.ping","params":{},"id":1}' | \
  socat - UNIX-CONNECT:/tmp/nestgate-nat0.sock

# Get identity
echo '{"jsonrpc":"2.0","method":"identity.get","params":{},"id":2}' | \
  socat - UNIX-CONNECT:/tmp/nestgate-nat0.sock

# Store data (family_id optional -- server defaults to its NESTGATE_FAMILY_ID)
echo '{"jsonrpc":"2.0","method":"storage.store","params":{"key":"test","value":[1,2,3]},"id":3}' | \
  socat - UNIX-CONNECT:/tmp/nestgate-nat0.sock
```

### Rust Client

```rust
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn call_rpc(method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
    let mut stream = UnixStream::connect("/tmp/nestgate-nat0.sock").await?;

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });

    stream.write_all(serde_json::to_string(&request)?.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let json: serde_json::Value = serde_json::from_str(&response)?;
    Ok(json)
}
```

---

## RPC Methods

### Health Methods

- `health.ping` - Ping server
- `health.status` - Get server status

### Identity Methods

- `identity.get` - Get primal identity
- `identity.capabilities` - List capabilities

### Storage Methods

- `storage.store` - Store key-value pair
- `storage.retrieve` - Retrieve value by key (single response, up to 64 MiB)
- `storage.delete` - Delete key
- `storage.list` - List keys with prefix
- `storage.exists` - Check if key exists
- `storage.stats` - Storage statistics
- `storage.store_blob` - Store binary blob (base64)
- `storage.retrieve_blob` - Retrieve binary blob (base64)
- `storage.retrieve_range` - Read byte range (max 4 MiB per call)
- `storage.object.size` - Get object size in bytes
- `storage.namespaces.list` - List namespaces for a family
- `storage.fetch_external` - Fetch and cache data from an external HTTPS URL

### Streaming Storage (chunked upload/download)

For large objects that exceed single-response limits, use the streaming protocol.
Each chunk carries up to 4 MiB decoded (base64-encoded in JSON).

**Upload:**

1. `storage.store_stream` `{key, family_id?, dataset?}` -- returns `{stream_id, chunk_size}`
2. `storage.store_stream_chunk` `{stream_id, data, is_last}` -- repeat until `is_last: true`

**Download:**

1. `storage.retrieve_stream` `{key, family_id?, dataset?, chunk_size?}` -- returns `{stream_id, total_size, chunk_size}`
2. `storage.retrieve_stream_chunk` `{stream_id, offset}` -- returns `{data, length, is_last}` -- repeat until `is_last: true`

Sessions expire after 1 hour of inactivity.

### System Methods

- `system.info` - Get system information

---

## Security (capability-based provider)

### Discovery

```rust
use nestgate_api::transport::SecurityProviderClient;

let mut client = SecurityProviderClient::discover("nat0").await?;
client.connect().await?;

let ciphertext = client.encrypt(b"secret data").await?;
let plaintext = client.decrypt(&ciphertext).await?;

let token = client.generate_token("nestgate").await?;
let valid = client.validate_token(&token).await?;
```

---

## Testing

### Unit Tests

```bash
cargo test --package nestgate-api --lib transport
```

### Integration Tests

```bash
cargo test --package nestgate-api --test transport_integration_test
```

### Example Server

```bash
NESTGATE_FAMILY_ID=example cargo run --example unix_socket_server
```

---

## Design Principles

- **Primal Self-Knowledge**: Only knows NestGate identity
- **Runtime Discovery**: Discovers the security provider via capability scan and socket patterns
- **Capability-Based**: No hardcoded endpoints or ports
- **Zero Technical Debt**: No `unwrap()` or `expect()` in production paths
- **Modern Idiomatic Rust**: Async/await, RAII, type-safe JSON-RPC

---

## Examples

See:
- `examples/unix_socket_server.rs` - Complete server example
- `tests/transport_integration_test.rs` - Integration tests
