//! **Zero-Copy Performance Metrics**
//!
//! Comprehensive statistics and performance tracking for zero-copy operations.
//!
//! ## Metrics Tracked
//!
//! - **Network Statistics**: Bytes sent/received, packets, zero-copy operations
//! - **Connection Statistics**: Per-connection performance tracking
//! - **Hardware Statistics**: DMA transfers, interrupts, kernel bypass metrics
//! - **Buffer Pool Statistics**: Hit rates, allocation patterns
//!
//! All metrics use atomic operations for lock-free concurrent access.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ==================== NETWORK STATISTICS ====================

/// Network-level statistics for zero-copy operations
#[derive(Debug, Default)]
pub struct NetworkStats {
    /// Total bytes sent
    pub bytes_sent: AtomicU64,
    /// Total bytes received
    pub bytes_received: AtomicU64,
    /// Total packets sent
    pub packets_sent: AtomicU64,
    /// Total packets received
    pub packets_received: AtomicU64,
    /// Zero-copy operations performed
    pub zero_copy_operations: AtomicU64,
    /// CPU cycles saved through zero-copy
    pub cpu_cycles_saved: AtomicU64,
}

impl NetworkStats {
    /// Create new network statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Get snapshot of current statistics
    pub fn snapshot(&self) -> NetworkInterfaceStats {
        NetworkInterfaceStats {
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            packets_sent: self.packets_sent.load(Ordering::Relaxed),
            packets_received: self.packets_received.load(Ordering::Relaxed),
            zero_copy_operations: self.zero_copy_operations.load(Ordering::Relaxed),
            cpu_cycles_saved: self.cpu_cycles_saved.load(Ordering::Relaxed),
            active_connections: 0, // Set by caller
            buffer_pool_stats: Default::default(), // Set by caller
        }
    }

    /// Reset all statistics
    pub fn reset(&self) {
        self.bytes_sent.store(0, Ordering::Relaxed);
        self.bytes_received.store(0, Ordering::Relaxed);
        self.packets_sent.store(0, Ordering::Relaxed);
        self.packets_received.store(0, Ordering::Relaxed);
        self.zero_copy_operations.store(0, Ordering::Relaxed);
        self.cpu_cycles_saved.store(0, Ordering::Relaxed);
    }
}

/// Immutable snapshot of network interface statistics
#[derive(Debug, Clone)]
pub struct NetworkInterfaceStats {
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Total packets sent
    pub packets_sent: u64,
    /// Total packets received
    pub packets_received: u64,
    /// Zero-copy operations performed
    pub zero_copy_operations: u64,
    /// CPU cycles saved
    pub cpu_cycles_saved: u64,
    /// Active connections count
    pub active_connections: usize,
    /// Buffer pool statistics
    pub buffer_pool_stats: crate::zero_copy::BufferPoolStats,
}

impl NetworkInterfaceStats {
    /// Calculate throughput in bytes per second
    pub fn throughput(&self, elapsed_secs: f64) -> f64 {
        if elapsed_secs > 0.0 {
            (self.bytes_sent + self.bytes_received) as f64 / elapsed_secs
        } else {
            0.0
        }
    }

    /// Calculate zero-copy efficiency percentage
    pub fn zero_copy_efficiency(&self) -> f64 {
        let total_ops = self.packets_sent + self.packets_received;
        if total_ops > 0 {
            (self.zero_copy_operations as f64 / total_ops as f64) * 100.0
        } else {
            0.0
        }
    }
}

// ==================== CONNECTION STATISTICS ====================

/// Per-connection statistics
#[derive(Debug, Default)]
pub struct ConnectionStats {
    /// Bytes transmitted on this connection
    pub bytes_transmitted: AtomicU64,
    /// Packets transmitted on this connection
    pub packets_transmitted: AtomicU64,
    /// Zero-copy transfers on this connection
    pub zero_copy_transfers: AtomicU64,
    /// Last activity timestamp (microseconds since epoch)
    pub last_activity: AtomicU64,
}

impl ConnectionStats {
    /// Create new connection statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Update last activity timestamp
    pub fn touch(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        self.last_activity.store(now, Ordering::Relaxed);
    }

    /// Get connection uptime in seconds
    pub fn uptime_secs(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let last = self.last_activity.load(Ordering::Relaxed);
        if last > 0 {
            (now - last) / 1_000_000
        } else {
            0
        }
    }
}

// ==================== HARDWARE STATISTICS ====================

/// Hardware-level statistics for kernel bypass operations
#[derive(Debug)]
pub struct HardwareStats {
    /// DMA transfers performed
    pub dma_transfers: AtomicU64,
    /// Hardware interrupts received
    pub hardware_interrupts: AtomicU64,
    /// Packets that bypassed kernel
    pub kernel_bypassed_packets: AtomicU64,
    /// Average latency in microseconds
    pub latency_microseconds: AtomicU64,
}

impl Clone for HardwareStats {
    fn clone(&self) -> Self {
        Self {
            dma_transfers: AtomicU64::new(self.dma_transfers.load(Ordering::Relaxed)),
            hardware_interrupts: AtomicU64::new(self.hardware_interrupts.load(Ordering::Relaxed)),
            kernel_bypassed_packets: AtomicU64::new(
                self.kernel_bypassed_packets.load(Ordering::Relaxed),
            ),
            latency_microseconds: AtomicU64::new(
                self.latency_microseconds.load(Ordering::Relaxed),
            ),
        }
    }
}

impl Default for HardwareStats {
    fn default() -> Self {
        Self {
            dma_transfers: AtomicU64::new(0),
            hardware_interrupts: AtomicU64::new(0),
            kernel_bypassed_packets: AtomicU64::new(0),
            latency_microseconds: AtomicU64::new(0),
        }
    }
}

impl HardwareStats {
    /// Create new hardware statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Get snapshot of hardware statistics
    pub fn snapshot(&self) -> HardwareStatsSnapshot {
        HardwareStatsSnapshot {
            dma_transfers: self.dma_transfers.load(Ordering::Relaxed),
            hardware_interrupts: self.hardware_interrupts.load(Ordering::Relaxed),
            kernel_bypassed_packets: self.kernel_bypassed_packets.load(Ordering::Relaxed),
            latency_microseconds: self.latency_microseconds.load(Ordering::Relaxed),
        }
    }
}

/// Immutable snapshot of hardware statistics
#[derive(Debug, Clone, Copy)]
pub struct HardwareStatsSnapshot {
    /// DMA transfers
    pub dma_transfers: u64,
    /// Hardware interrupts
    pub hardware_interrupts: u64,
    /// Kernel bypassed packets
    pub kernel_bypassed_packets: u64,
    /// Latency in microseconds
    pub latency_microseconds: u64,
}

impl HardwareStatsSnapshot {
    /// Calculate DMA efficiency (transfers per interrupt)
    pub fn dma_efficiency(&self) -> f64 {
        if self.hardware_interrupts > 0 {
            self.dma_transfers as f64 / self.hardware_interrupts as f64
        } else {
            0.0
        }
    }

    /// Calculate kernel bypass percentage
    pub fn kernel_bypass_rate(&self) -> f64 {
        let total_packets = self.kernel_bypassed_packets + self.dma_transfers;
        if total_packets > 0 {
            (self.kernel_bypassed_packets as f64 / total_packets as f64) * 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_stats() {
        let stats = NetworkStats::new();
        stats.bytes_sent.store(1000, Ordering::Relaxed);
        stats.packets_sent.store(10, Ordering::Relaxed);
        
        let snapshot = stats.snapshot();
        assert_eq!(snapshot.bytes_sent, 1000);
        assert_eq!(snapshot.packets_sent, 10);
        
        stats.reset();
        let snapshot = stats.snapshot();
        assert_eq!(snapshot.bytes_sent, 0);
    }

    #[test]
    fn test_connection_stats() {
        let stats = ConnectionStats::new();
        stats.touch();
        
        let last = stats.last_activity.load(Ordering::Relaxed);
        assert!(last > 0);
    }

    #[test]
    fn test_hardware_stats() {
        let stats = HardwareStats::new();
        stats.dma_transfers.store(100, Ordering::Relaxed);
        stats.hardware_interrupts.store(10, Ordering::Relaxed);
        
        let snapshot = stats.snapshot();
        assert_eq!(snapshot.dma_efficiency(), 10.0);
    }
}
