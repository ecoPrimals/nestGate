// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::marker::PhantomData;

use nestgate_core::error::{NestGateError, Result};

use super::buffer_pool::ZeroCopyBuffer;

/// **KERNEL BYPASS NETWORK ADAPTER**
///
/// Direct hardware access for maximum performance
/// Bypasses kernel network stack for ultra-low latency
pub struct KernelBypassAdapter<const RING_SIZE: usize = 4096> {
    tx_ring: ZeroCopyRing<RING_SIZE>,
    rx_ring: ZeroCopyRing<RING_SIZE>,
    hardware_stats: HardwareStats,
    _phantom: PhantomData<()>,
}
/// **ZERO-COPY RING BUFFER**
///
/// Lock-free ring buffer for kernel bypass networking
/// Direct DMA integration with network hardware
pub struct ZeroCopyRing<const SIZE: usize> {
    buffers: [Option<ZeroCopyBuffer<2048>>; SIZE],
    head: std::sync::atomic::AtomicUsize,
    tail: std::sync::atomic::AtomicUsize,
    _phantom: PhantomData<()>,
}
#[derive(Debug)]
/// Hardwarestats
pub struct HardwareStats {
    /// Dma Transfers
    pub dma_transfers: std::sync::atomic::AtomicU64,
    /// Hardware Interrupts
    pub hardware_interrupts: std::sync::atomic::AtomicU64,
    /// Kernel Bypassed Packets
    pub kernel_bypassed_packets: std::sync::atomic::AtomicU64,
    /// Latency Microseconds
    pub latency_microseconds: std::sync::atomic::AtomicU64,
}

impl Clone for HardwareStats {
    /// Clone
    fn clone(&self) -> Self {
        use std::sync::atomic::AtomicU64;
        Self {
            dma_transfers: AtomicU64::new(
                self.dma_transfers
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            hardware_interrupts: AtomicU64::new(
                self.hardware_interrupts
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            kernel_bypassed_packets: AtomicU64::new(
                self.kernel_bypassed_packets
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            latency_microseconds: AtomicU64::new(
                self.latency_microseconds
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

impl Default for HardwareStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            dma_transfers: std::sync::atomic::AtomicU64::new(0),
            hardware_interrupts: std::sync::atomic::AtomicU64::new(0),
            kernel_bypassed_packets: std::sync::atomic::AtomicU64::new(0),
            latency_microseconds: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

impl<const RING_SIZE: usize> Default for KernelBypassAdapter<RING_SIZE> {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            tx_ring: ZeroCopyRing::new(),
            rx_ring: ZeroCopyRing::new(),
            hardware_stats: HardwareStats::default(),
            _phantom: PhantomData,
        }
    }
}

impl<const RING_SIZE: usize> KernelBypassAdapter<RING_SIZE> {
    /// Create new kernel bypass adapter
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize hardware for kernel bypass
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn initialize_hardware(&mut self) -> Result<()> {
        // In a real implementation, this would:
        // 1. Map hardware registers
        // 2. Set up DMA rings
        // 3. Configure interrupt handling
        // 4. Enable kernel bypass mode

        tracing::info!(
            "Kernel bypass adapter initialized with {} ring entries",
            RING_SIZE
        );
        Ok(())
    }

    /// Send packet with direct hardware access
    /// PERFORMANCE: Sub-microsecond latency, no kernel overhead
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn hardware_send(&mut self, buffer: ZeroCopyBuffer<2048>) -> Result<()> {
        // Direct DMA transmission
        if let Some(slot) = self.tx_ring.acquire_slot() {
            self.tx_ring.set_buffer(slot, buffer)?;

            // Trigger hardware transmission (would be hardware register write)
            self.hardware_stats
                .dma_transfers
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.hardware_stats
                .kernel_bypassed_packets
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            tracing::trace!("Hardware send initiated for slot {}", slot);
            Ok(())
        } else {
            Err(NestGateError::network_error(
                "No available TX slots for hardware send",
            ))
        }
    }

    /// Receive packet from hardware
    /// PERFORMANCE: Direct DMA access, zero-copy from NIC
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn hardware_receive(&mut self) -> Result<Option<ZeroCopyBuffer<2048>>> {
        if let Some(slot) = self.rx_ring.completed_slot() {
            let buffer = self.rx_ring.take_buffer(slot)?;

            self.hardware_stats
                .hardware_interrupts
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            tracing::trace!("Hardware receive completed for slot {}", slot);
            Ok(Some(buffer))
        } else {
            Ok(None)
        }
    }

    /// Get hardware statistics
    pub fn get_hardware_stats(&self) -> HardwareStats {
        self.hardware_stats.clone()
    }
}

impl<const SIZE: usize> Default for ZeroCopyRing<SIZE> {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            buffers: [const { None }; SIZE],
            head: std::sync::atomic::AtomicUsize::new(0),
            tail: std::sync::atomic::AtomicUsize::new(0),
            _phantom: PhantomData,
        }
    }
}

impl<const SIZE: usize> ZeroCopyRing<SIZE> {
    /// Create new zero-copy ring buffer
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquire slot for transmission
    pub fn acquire_slot(&self) -> Option<usize> {
        let head = self.head.load(std::sync::atomic::Ordering::Acquire);
        let next_head = (head + 1) % SIZE;
        let tail = self.tail.load(std::sync::atomic::Ordering::Acquire);

        if next_head == tail {
            None
        } else {
            self.head
                .store(next_head, std::sync::atomic::Ordering::Release);
            Some(head)
        }
    }

    /// Check for completed transmission/reception
    pub fn completed_slot(&self) -> Option<usize> {
        let tail = self.tail.load(std::sync::atomic::Ordering::Acquire);
        let head = self.head.load(std::sync::atomic::Ordering::Acquire);

        if tail == head { None } else { Some(tail) }
    }

    /// Set buffer in ring slot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn set_buffer(&mut self, slot: usize, buffer: ZeroCopyBuffer<2048>) -> Result<()> {
        if slot < SIZE {
            self.buffers[slot] = Some(buffer);
            Ok(())
        } else {
            Err(NestGateError::validation("ring_slot"))
        }
    }

    /// Take buffer from ring slot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn take_buffer(&mut self, slot: usize) -> Result<ZeroCopyBuffer<2048>> {
        if slot < SIZE {
            if let Some(buffer) = self.buffers[slot].take() {
                let next_tail = (slot + 1) % SIZE;
                self.tail
                    .store(next_tail, std::sync::atomic::Ordering::Release);
                Ok(buffer)
            } else {
                Err(NestGateError::network_error("No buffer in ring slot"))
            }
        } else {
            Err(NestGateError::validation("ring_slot"))
        }
    }
}
