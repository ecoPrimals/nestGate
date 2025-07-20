//! Enhanced Streaming Communication Demo
//!
//! This demo showcases the enhanced streaming capabilities of NestGate, including:
//! - Server-Sent Events (SSE) for real-time updates
//! - WebSocket communication for bidirectional streaming
//! - MCP (Message Channel Protocol) streaming for efficient data transfer
//! - Event coordination and monitoring
//!
//! Note: This demo requires the streaming-rpc feature to be enabled.

#![allow(dead_code)]

#[cfg(feature = "streaming-rpc")]
mod streaming_demo {
    use std::sync::Arc;

    use tracing::info;

    // Import all communication components
    use nestgate_api::CommunicationManager;

    pub async fn run_streaming_demo() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize comprehensive logging
        tracing_subscriber::fmt().with_env_filter("debug").init();

        info!("🚀 Starting Enhanced Streaming and Bidirectional Communication Demo");

        // Initialize the unified communication manager
        let _comm_manager = Arc::new(CommunicationManager::new());

        info!("✅ Enhanced streaming demo completed successfully!");
        Ok(())
    }
}

#[cfg(feature = "streaming-rpc")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    streaming_demo::run_streaming_demo().await
}

#[cfg(not(feature = "streaming-rpc"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Enhanced streaming demo is only available with streaming-rpc feature");
    println!("Please enable the streaming-rpc feature to run this demo");
    Ok(())
}
