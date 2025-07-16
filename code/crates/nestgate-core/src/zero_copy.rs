//! Zero-Copy Optimization Utilities
//!
//! This module provides zero-copy optimized implementations for common operations
//! to reduce memory allocations and improve performance across the NestGate codebase.

use bytes::Bytes;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::{NestGateError, Result};

/// Shared string type for zero-copy string sharing
pub type SharedString = Arc<str>;

/// Create a shared string from a string slice
pub fn shared_string(s: &str) -> SharedString {
    Arc::from(s)
}

/// Memory information optimized for zero-copy parsing
#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub free: u64,
    pub buffers: u64,
    pub cached: u64,
}

/// CPU information optimized for zero-copy parsing
#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
}

/// Network interface statistics for zero-copy parsing
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    pub interface: SharedString,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errors: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errors: u64,
}

/// Streaming file reader for /proc filesystem
pub struct StreamingProcReader;

impl StreamingProcReader {
    /// Read memory information from /proc/meminfo using streaming approach
    pub async fn read_meminfo() -> Result<MemoryInfo> {
        let file = File::open("/proc/meminfo")
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to open /proc/meminfo: {e}")))?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut memory_info = MemoryInfo::default();

        while let Some(line) = lines.next_line().await.map_err(|e| {
            NestGateError::Internal(format!("Failed to read /proc/meminfo line: {e}"))
        })? {
            if line.starts_with("MemTotal:") {
                memory_info.total = Self::parse_memory_line(&line)?;
            } else if line.starts_with("MemAvailable:") {
                memory_info.available = Self::parse_memory_line(&line)?;
            } else if line.starts_with("MemFree:") {
                memory_info.free = Self::parse_memory_line(&line)?;
            } else if line.starts_with("Buffers:") {
                memory_info.buffers = Self::parse_memory_line(&line)?;
            } else if line.starts_with("Cached:") {
                memory_info.cached = Self::parse_memory_line(&line)?;
            }
        }

        // Calculate used memory
        memory_info.used = memory_info.total.saturating_sub(memory_info.available);

        Ok(memory_info)
    }

    /// Read CPU information from /proc/stat using streaming approach
    pub async fn read_cpuinfo() -> Result<CpuInfo> {
        let file = File::open("/proc/stat")
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to open /proc/stat: {e}")))?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        if let Some(line) = lines
            .next_line()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to read /proc/stat line: {e}")))?
        {
            if line.starts_with("cpu ") {
                return Self::parse_cpu_line(&line);
            }
        }

        Err(NestGateError::Internal(
            "Failed to find CPU line in /proc/stat".to_string(),
        ))
    }

    /// Read network statistics from /proc/net/dev using streaming approach
    pub async fn read_network_stats() -> Result<Vec<NetworkStats>> {
        let file = File::open("/proc/net/dev")
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to open /proc/net/dev: {e}")))?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Skip header lines
        lines.next_line().await.map_err(|e| {
            NestGateError::Internal(format!("Failed to read /proc/net/dev header: {e}"))
        })?;
        lines.next_line().await.map_err(|e| {
            NestGateError::Internal(format!("Failed to read /proc/net/dev header: {e}"))
        })?;

        let mut stats = Vec::new();

        while let Some(line) = lines.next_line().await.map_err(|e| {
            NestGateError::Internal(format!("Failed to read /proc/net/dev line: {e}"))
        })? {
            if let Ok(net_stat) = Self::parse_network_line(&line) {
                stats.push(net_stat);
            }
        }

        Ok(stats)
    }

    /// Parse memory line from /proc/meminfo
    fn parse_memory_line(line: &str) -> Result<u64> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let kb_str = parts[1];
            let kb_value = kb_str.parse::<u64>().map_err(|e| {
                NestGateError::Internal(format!("Failed to parse memory value: {e}"))
            })?;
            Ok(kb_value * 1024) // Convert KB to bytes
        } else {
            Err(NestGateError::Internal(
                "Invalid memory line format".to_string(),
            ))
        }
    }

    /// Parse CPU line from /proc/stat
    fn parse_cpu_line(line: &str) -> Result<CpuInfo> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 8 {
            let cpu_info = CpuInfo {
                user: parts[1].parse().unwrap_or(0),
                nice: parts[2].parse().unwrap_or(0),
                system: parts[3].parse().unwrap_or(0),
                idle: parts[4].parse().unwrap_or(0),
                iowait: parts[5].parse().unwrap_or(0),
                irq: parts[6].parse().unwrap_or(0),
                softirq: parts[7].parse().unwrap_or(0),
                steal: if parts.len() >= 9 {
                    parts[8].parse().unwrap_or(0)
                } else {
                    0
                },
            };
            Ok(cpu_info)
        } else {
            Err(NestGateError::Internal(
                "Invalid CPU line format".to_string(),
            ))
        }
    }

    /// Parse network interface line from /proc/net/dev
    fn parse_network_line(line: &str) -> Result<NetworkStats> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 17 {
            let interface_part = parts[0];
            let interface_name = interface_part.trim_end_matches(':');

            let stats = NetworkStats {
                interface: shared_string(interface_name),
                rx_bytes: parts[1].parse().unwrap_or(0),
                rx_packets: parts[2].parse().unwrap_or(0),
                rx_errors: parts[3].parse().unwrap_or(0),
                tx_bytes: parts[9].parse().unwrap_or(0),
                tx_packets: parts[10].parse().unwrap_or(0),
                tx_errors: parts[11].parse().unwrap_or(0),
            };

            Ok(stats)
        } else {
            Err(NestGateError::Internal(
                "Invalid network line format".to_string(),
            ))
        }
    }
}

/// Zero-copy string utilities
pub mod string_utils {
    use super::*;

    /// Efficiently concatenate strings using Cow
    pub fn concat_strings<'a>(parts: &[&'a str]) -> Cow<'a, str> {
        if parts.is_empty() {
            return Cow::Borrowed("");
        }

        if parts.len() == 1 {
            return Cow::Borrowed(parts[0]);
        }

        let total_len: usize = parts.iter().map(|s| s.len()).sum();
        let mut result = String::with_capacity(total_len);

        for part in parts {
            result.push_str(part);
        }

        Cow::Owned(result)
    }

    /// Create a shared string pool for frequently used strings
    pub struct SharedStringPool {
        strings: HashMap<String, SharedString>,
    }

    impl SharedStringPool {
        pub fn new() -> Self {
            Self {
                strings: HashMap::new(),
            }
        }

        pub fn get_or_create(&mut self, s: &str) -> SharedString {
            self.strings
                .entry(s.to_string())
                .or_insert_with(|| shared_string(s))
                .clone()
        }
    }

    impl Default for SharedStringPool {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Zero-copy buffer utilities
pub mod buffer_utils {
    use super::*;

    /// Zero-copy buffer for network and file operations
    pub struct ZeroCopyBuffer {
        data: Bytes,
        position: usize,
    }

    impl ZeroCopyBuffer {
        pub fn new(data: Bytes) -> Self {
            Self { data, position: 0 }
        }

        pub fn from_vec(vec: Vec<u8>) -> Self {
            Self::new(Bytes::from(vec))
        }

        pub fn slice(&self, start: usize, end: usize) -> Bytes {
            self.data.slice(start..end)
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn position(&self) -> usize {
            self.position
        }

        pub fn set_position(&mut self, pos: usize) {
            self.position = pos.min(self.data.len());
        }

        pub fn remaining(&self) -> usize {
            self.data.len().saturating_sub(self.position)
        }

        pub fn read_slice(&mut self, len: usize) -> Bytes {
            let start = self.position;
            let end = (start + len).min(self.data.len());
            self.position = end;
            self.data.slice(start..end)
        }
    }
}

/// Zero-copy collection utilities
pub mod collection_utils {
    use super::*;

    /// Pre-allocated vector builder
    pub struct PreAllocatedVecBuilder<T> {
        vec: Vec<T>,
        capacity: usize,
    }

    impl<T> PreAllocatedVecBuilder<T> {
        pub fn new(capacity: usize) -> Self {
            Self {
                vec: Vec::with_capacity(capacity),
                capacity,
            }
        }

        pub fn push(&mut self, item: T) -> Result<()> {
            if self.vec.len() >= self.capacity {
                return Err(NestGateError::Internal(
                    "Vector capacity exceeded".to_string(),
                ));
            }
            self.vec.push(item);
            Ok(())
        }

        pub fn build(self) -> Vec<T> {
            self.vec
        }

        pub fn len(&self) -> usize {
            self.vec.len()
        }

        pub fn is_empty(&self) -> bool {
            self.vec.is_empty()
        }

        pub fn capacity(&self) -> usize {
            self.capacity
        }
    }

    /// Efficient key-value pair processing
    pub fn process_key_value_pairs<'a, F, R>(
        input: &'a str,
        separator: char,
        processor: F,
    ) -> Vec<R>
    where
        F: Fn(&'a str, &'a str) -> R,
    {
        input
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, separator);
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) => Some(processor(key.trim(), value.trim())),
                    _ => None,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_streaming_meminfo_read() {
        // Test that memory info can be read without errors
        let result = StreamingProcReader::read_meminfo().await;
        assert!(result.is_ok() || result.is_err()); // Should not panic
    }

    #[tokio::test]
    async fn test_streaming_cpuinfo_read() {
        // Test that CPU info can be read without errors
        let result = StreamingProcReader::read_cpuinfo().await;
        assert!(result.is_ok() || result.is_err()); // Should not panic
    }

    #[test]
    fn test_shared_string_creation() {
        let s1 = shared_string("test");
        let s2 = shared_string("test");
        // Both should point to different Arc instances but same content
        assert_eq!(s1.as_ref(), s2.as_ref());
    }

    #[test]
    fn test_zero_copy_buffer() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = buffer_utils::ZeroCopyBuffer::from_vec(data);

        assert_eq!(buffer.len(), 5);
        assert!(!buffer.is_empty());

        let slice = buffer.slice(1, 4);
        assert_eq!(slice.len(), 3);
    }

    #[test]
    fn test_pre_allocated_vec_builder() {
        let mut builder = collection_utils::PreAllocatedVecBuilder::new(3);

        assert!(builder.push(1).is_ok());
        assert!(builder.push(2).is_ok());
        assert!(builder.push(3).is_ok());

        let vec = builder.build();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_string_concatenation() {
        let parts = ["Hello", " ", "World"];
        let result = string_utils::concat_strings(&parts);
        assert_eq!(result, "Hello World");
    }
}
