// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use bytes::Bytes;
use std::net::SocketAddr;

#[test]
fn test_zero_copy_buffer() {
    let mut buffer = ZeroCopyBuffer::<1024>::new();
    assert_eq!(buffer.capacity(), 1024);
    assert_eq!(buffer.len(), 0);

    let test_data = b"Hello, zero-copy world!";
    buffer.as_mut_slice()[..test_data.len()].copy_from_slice(test_data);
    buffer.set_length(test_data.len());

    assert_eq!(buffer.len(), test_data.len());
    assert_eq!(buffer.as_slice(), test_data);
}
#[test]
fn test_buffer_pool() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = ZeroCopyBufferPool::<1024, 10>::new();

    // Modern error handling - use ok_or for Option -> Result conversion
    let buffer1 = pool.acquire_buffer().ok_or("Failed to acquire buffer 1")?;
    let buffer2 = pool.acquire_buffer().ok_or("Failed to acquire buffer 2")?;

    pool.release_buffer(buffer1);
    pool.release_buffer(buffer2);

    let stats = pool.stats();
    assert!(stats.buffer_hits >= 2);
    Ok(())
}

#[tokio::test]
async fn test_zero_copy_interface() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let interface = ZeroCopyNetworkInterface::<1024>::new();

    use nestgate_core::constants::{DEFAULT_API_PORT, hardcoding};
    let default_endpoint = format!(
        "{}:{}",
        hardcoding::addresses::LOCALHOST_IPV4,
        DEFAULT_API_PORT
    );
    let test_endpoint = std::env::var("NESTGATE_TEST_ENDPOINT").unwrap_or(default_endpoint);
    let socket_addr: SocketAddr = test_endpoint
        .parse()
        .map_err(|e| format!("Invalid test endpoint '{}': {}", test_endpoint, e))?;
    let connection_id = interface.connect(socket_addr)?;

    let test_data = b"Test zero-copy send";
    let bytes_sent = interface.zero_copy_send(connection_id, test_data)?;

    assert_eq!(bytes_sent, test_data.len());

    let stats = interface.get_stats();
    assert_eq!(stats.bytes_sent, test_data.len() as u64);
    assert_eq!(stats.packets_sent, 1);
    assert_eq!(stats.zero_copy_operations, 1);
    Ok(())
}

#[tokio::test]
async fn test_zero_copy_send_bytes_refcount_path()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let interface = ZeroCopyNetworkInterface::<1024>::new();

    use nestgate_core::constants::{DEFAULT_API_PORT, hardcoding};
    let default_endpoint = format!(
        "{}:{}",
        hardcoding::addresses::LOCALHOST_IPV4,
        DEFAULT_API_PORT
    );
    let test_endpoint = std::env::var("NESTGATE_TEST_ENDPOINT").unwrap_or(default_endpoint);
    let socket_addr: SocketAddr = test_endpoint
        .parse()
        .map_err(|e| format!("Invalid test endpoint '{}': {}", test_endpoint, e))?;
    let connection_id = interface.connect(socket_addr)?;

    let payload = Bytes::from_static(b"refcounted-no-memcpy");
    let n = interface.zero_copy_send_bytes(connection_id, payload)?;
    assert_eq!(n, b"refcounted-no-memcpy".len());
    Ok(())
}

#[test]
fn test_kernel_bypass_adapter() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut adapter = KernelBypassAdapter::<64>::new();
    adapter.initialize_hardware()?;

    let buffer = ZeroCopyBuffer::<2048>::new();
    adapter.hardware_send(buffer)?;

    let stats = adapter.get_hardware_stats();
    assert_eq!(
        stats
            .dma_transfers
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
    Ok(())
}

#[tokio::test]
async fn test_zero_copy_receive_and_vectored_send()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let interface = ZeroCopyNetworkInterface::<1024>::new();
    use nestgate_core::constants::{DEFAULT_API_PORT, hardcoding};
    let addr: SocketAddr = format!(
        "{}:{}",
        hardcoding::addresses::LOCALHOST_IPV4,
        DEFAULT_API_PORT
    )
    .parse()?;
    let cid = interface.connect(addr)?;
    assert!(interface.zero_copy_receive(cid)?.is_none());

    let mut b1 = ZeroCopyBuffer::<1024>::new();
    b1.as_mut_slice()[0..4].copy_from_slice(b"abcd");
    b1.set_length(4);
    let total = interface.vectored_send(cid, &[b1])?;
    assert_eq!(total, 4);

    let s = interface.get_stats();
    assert!(s.buffer_pool_stats.total_buffers > 0);
    assert_eq!(s.active_connections, 1);
    Ok(())
}

#[test]
fn test_zero_copy_ring_take_buffer_errors() {
    let mut ring = ZeroCopyRing::<8>::new();
    assert!(ring.take_buffer(99).is_err());
    let slot = ring.acquire_slot().expect("slot");
    assert!(ring.take_buffer(slot).is_err());
}

#[test]
fn test_kernel_bypass_hardware_receive_none() -> nestgate_core::Result<()> {
    let mut adapter = KernelBypassAdapter::<8>::new();
    assert!(adapter.hardware_receive()?.is_none());
    Ok(())
}

#[test]
fn test_zero_copy_send_missing_connection_errors() {
    let interface = ZeroCopyNetworkInterface::<256>::new();
    assert!(
        interface
            .zero_copy_send(0xFFFF_FFFF_FFFF_FFFF, b"x")
            .is_err()
    );
}
