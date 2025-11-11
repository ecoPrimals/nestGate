// Quick test of the discovery logic
use std::net::ToSocketAddrs;
use std::net::TcpStream;
use std::time::Duration;

fn is_orchestrator_available(url: &str) -> bool {
    let url_without_protocol = url.trim_start_matches("http://").trim_start_matches("https://");
    let parts: Vec<&str> = url_without_protocol.split(':').collect();
    
    if parts.len() >= 2 {
        let host = parts[0];
        if let Ok(port) = parts[1].parse::<u16>() {
            // Try to parse as IP address first
            if let Ok(sock_addr) = format!("{}:{}", host, port).parse::<std::net::SocketAddr>() {
                println!("  Trying as IP: {}", sock_addr);
                return TcpStream::connect_timeout(
                    &sock_addr,
                    Duration::from_millis(100)
                ).is_ok();
            }
            
            // If not an IP, try DNS resolution (for "localhost" etc.)
            println!("  Trying DNS resolution for {}:{}", host, port);
            if let Ok(mut addrs) = format!("{}:{}", host, port).to_socket_addrs() {
                if let Some(addr) = addrs.next() {
                    println!("    Resolved to: {}", addr);
                    return TcpStream::connect_timeout(
                        &addr,
                        Duration::from_millis(100)
                    ).is_ok();
                }
            }
        }
    }
    
    false
}

fn main() {
    let urls = [
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://192.168.1.144:8080",
    ];
    
    for url in &urls {
        println!("Testing: {}", url);
        let result = is_orchestrator_available(url);
        println!("  Result: {}\n", if result { "✅ AVAILABLE" } else { "❌ NOT AVAILABLE" });
    }
}

