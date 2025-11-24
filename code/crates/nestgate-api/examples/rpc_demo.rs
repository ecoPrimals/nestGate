//! RPC System Demo Example
//!
//! NOTE: This example demonstrates the RPC system concept.
//! The actual implementation may have evolved. This is kept as documentation.

use serde_json::json;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🚀 NestGate RPC System Demo");
    println!("================================");
    println!();
    println!("This demo illustrates the RPC system architecture.");
    println!("The actual RPC implementation uses:");
    println!("  • UnifiedRpcRequest/Response for internal comms");
    println!("  • Capability-based routing via Universal Adapter");
    println!("  • Native async for zero-cost abstractions");
    println!();
    println!("Example RPC request structure:");
    let example_request = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "source": "nestgate",
        "target": "security",
        "method": "authenticate",
        "params": {"token": "example"},
        "timestamp": "2025-10-05T12:00:00Z",
        "streaming": false,
        "priority": "Normal"
    });
    println!("{}", serde_json::to_string_pretty(&example_request)?);
    println!();
    println!("✅ RPC system is capability-based and primal-agnostic");
    println!("✅ No hardcoded service endpoints");
    println!("✅ Runtime discovery via Universal Adapter");
    println!();
    println!("For real RPC usage, see:");
    println!("  • code/crates/nestgate-api/src/rest/rpc/");
    println!("  • code/crates/nestgate-core/src/universal_adapter/");

    Ok(())
}
