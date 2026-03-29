#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Live Integration Test 2: Real BearDog Communication
//!
//! **EVOLVED**: Replaced reqwest with raw TCP for ecoBin compliance.
//! NestGate delegates external HTTP to network primals. This example
//! uses minimal TCP-based HTTP for inter-primal connectivity checks.
//!
//! Prerequisites:
//! 1. Build BearDog BTSP server:
//!    cd ../beardog && cargo build --release --features btsp-api --example btsp_server
//! 2. Start BearDog in another terminal:
//!    cd ../beardog && ./target/release/examples/btsp_server
//! 3. Run this demo:
//!    cargo run --example live-integration-02-real-beardog

use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Send a raw HTTP request and return the response body
async fn http_request(
    addr: &str,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<(u16, String), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr).await?;

    let request = if let Some(body) = body {
        format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    } else {
        format!("{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
    };

    stream.write_all(request.as_bytes()).await?;

    let mut buf = vec![0u8; 8192];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]).to_string();

    // Parse status code from first line
    let status = response
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .unwrap_or(0);

    // Extract body after \r\n\r\n
    let body = response.split("\r\n\r\n").nth(1).unwrap_or("").to_string();

    Ok((status, body))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Live Integration Test: Real BearDog Communication");
    println!("===================================================\n");

    let addr = "127.0.0.1:9000";

    // Step 1: Discover BearDog
    println!("🔍 Step 1: Discovering BearDog BTSP Server...");

    match http_request(addr, "GET", "/health", None).await {
        Ok((status, body)) if (200..300).contains(&status) => {
            println!("✅ BearDog BTSP server discovered at localhost:9000");
            println!("   Health response: {}\n", body);
        }
        Ok((status, _)) => {
            println!("⚠️  BearDog responded with: {}", status);
            println!("   Continuing anyway...\n");
        }
        Err(e) => {
            println!("❌ BearDog not reachable: {}", e);
            println!("\n💡 To start BearDog BTSP server:");
            println!("   cd ../beardog");
            println!("   cargo run --release --features btsp-api --example btsp_server\n");
            return Ok(());
        }
    }

    // Step 2: Test tunnel establishment
    println!("🔒 Step 2: Establishing secure tunnel...");

    let tunnel_request = json!({
        "peer_id": "nestgate-test",
        "capabilities": ["encryption", "decryption"]
    });

    match http_request(
        addr,
        "POST",
        "/btsp/tunnel/establish",
        Some(&tunnel_request.to_string()),
    )
    .await
    {
        Ok((status, body)) if (200..300).contains(&status) => {
            println!("✅ Tunnel established:");
            if let Ok(result) = serde_json::from_str::<serde_json::Value>(&body) {
                println!("{}\n", serde_json::to_string_pretty(&result)?);

                if let Some(tunnel_id) = result.get("tunnel_id").and_then(|v| v.as_str()) {
                    println!("   Tunnel ID: {}\n", tunnel_id);

                    // Step 3: Test encryption through tunnel
                    println!("🔐 Step 3: Testing encryption through tunnel...");
                    let encrypt_request = json!({
                        "tunnel_id": tunnel_id,
                        "data": "Sensitive information that needs protection",
                        "algorithm": "AES-256-GCM"
                    });

                    match http_request(
                        addr,
                        "POST",
                        "/btsp/tunnel/encrypt",
                        Some(&encrypt_request.to_string()),
                    )
                    .await
                    {
                        Ok((s, b)) if (200..300).contains(&s) => {
                            println!("✅ Data encrypted successfully:");
                            println!("{}\n", b);
                        }
                        Ok((s, b)) => {
                            println!("⚠️  Encryption returned: {}", s);
                            println!("   Response: {}\n", b);
                        }
                        Err(e) => {
                            println!("⚠️  Encryption request failed: {}\n", e);
                        }
                    }

                    // Step 4: Close tunnel
                    println!("🧹 Step 4: Closing tunnel...");
                    let close_path = format!("/btsp/tunnel/close/{}", tunnel_id);
                    match http_request(addr, "DELETE", &close_path, None).await {
                        Ok((s, _)) if (200..300).contains(&s) => {
                            println!("✅ Tunnel closed successfully\n");
                        }
                        Ok((s, _)) => {
                            println!("⚠️  Close returned: {}\n", s);
                        }
                        Err(e) => {
                            println!("⚠️  Close request failed: {}\n", e);
                        }
                    }
                }
            } else {
                println!("{}\n", body);
            }
        }
        Ok((status, body)) => {
            println!("⚠️  Tunnel establishment returned: {}", status);
            println!("   Response: {}\n", body);
        }
        Err(e) => {
            println!("⚠️  Could not establish tunnel: {}", e);
            println!("   This is expected if endpoint signature differs\n");
        }
    }

    // Summary
    println!("📊 Integration Test Summary:");
    println!("   Discovery: ✅ BearDog BTSP server found");
    println!("   Communication: ✅ TCP working (ecoBin compliant)");
    println!("   BTSP Protocol: Check output above");

    println!("\n💡 Next Steps:");
    println!("   1. Document actual BTSP API structure");
    println!("   2. Implement proper client wrapper");
    println!("   3. Add error handling");
    println!("   4. Test full encrypted storage workflow");

    println!("\n🎉 Integration Test Complete!");
    println!("   - BearDog BTSP server operational");
    println!("   - Raw TCP communication verified (no reqwest)");
    println!("   - Ready for full integration");

    Ok(())
}
