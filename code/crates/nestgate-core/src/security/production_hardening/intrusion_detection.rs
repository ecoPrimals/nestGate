//! # Intrusion Detection System
//! Intrusion Detection functionality and utilities.
// Real-time intrusion detection and response

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

use crate::error::Result;
use super::config::{IntrusionDetectionConfig, ThreatLevel};
use super::types::{IntrusionDetectionStatistics, SuspiciousPattern};

/// **INTRUSION DETECTION SYSTEM**
///
/// Real-time intrusion detection and response
pub struct IntrusionDetectionSystem {
    /// IDS configuration
    config: IntrusionDetectionConfig,
    /// Failed authentication attempts per IP
    failed_auth_attempts: Arc<RwLock<HashMap<IpAddr, Vec<SystemTime>>>>,
    /// Blocked IPs
    blocked_ips: Arc<RwLock<HashMap<IpAddr, SystemTime>>>,
    /// Suspicious patterns
    suspicious_patterns: Vec<SuspiciousPattern>,
}
impl IntrusionDetectionSystem {
    /// Create new intrusion detection system
    pub const fn new(config: IntrusionDetectionConfig) -> Self {
        let suspicious_patterns = vec![
            SuspiciousPattern {
                name: "Port Scanning".to_string(),
                pattern: regex::Regex::new(r"(?i)(nmap|masscan|zmap)").unwrap(),
                threat_level: ThreatLevel::High,
            },
            SuspiciousPattern {
                name: "Directory Traversal".to_string(),
                pattern: regex::Regex::new(r"(\.\./|\.\.\\|%2e%2e%2f)").unwrap(),
                threat_level: ThreatLevel::High,
            },
            SuspiciousPattern {
                name: "Command Injection".to_string(),
                pattern: regex::Regex::new(r"(?i)(;|&&|\|\||\$\(|\`|nc\s|wget\s|curl\s)").unwrap(),
                threat_level: ThreatLevel::Critical,
            },
        ];

        Self {
            config,
            failed_auth_attempts: Arc::new(RwLock::new(HashMap::new())),
            blocked_ips: Arc::new(RwLock::new(HashMap::new())),
            suspicious_patterns,
        }
    }

    /// Analyze request for suspicious activity
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn analyze_request(
        &self,
        source_ip: IpAddr,
        method: &str,
        headers: &HashMap<String, String>,
        body: &[u8],
    ) -> Result<()>  {
        if !self.config.enabled {
            return Ok(());
        }

        let combined_input = format!("{} {} {}", method, path, String::from_utf8_lossy(body);

        for pattern in &self.suspicious_patterns {
            if pattern.pattern.is_match(&combined_input) {
                tracing::warn!(
                    target: "intrusion_detection",
                    pattern_name = %pattern.name,
                    threat_level = ?pattern.threat_level,
                    source_ip = %source_ip,
                    "Suspicious pattern detected"
                );

                if self.config.auto_blocking && matches!(pattern.threat_level, ThreatLevel::Critical) {
                    self.block_ip(source_ip, Duration::from_secs(3600)).await?;
                }
            }
        }

        Ok(())
    }

    /// Record authentication failure
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn record_auth_failure(&self, source_ip: IpAddr) -> Result<()>  {
        let mut attempts = self.failed_auth_attempts.write().await;
        let now = SystemTime::now();
        
        let ip_attempts = attempts.entry(source_ip).or_insert_with(Vec::new);
        ip_attempts.push(now);

        // Remove old attempts outside the window
        let window_start = now - self.config.failed_auth_window;
        ip_attempts.retain(|&time| time > window_start);

        // Check if threshold exceeded
        if ip_attempts.len() >= self.config.failed_auth_threshold as usize {
            if self.config.auto_blocking {
                self.block_ip(source_ip, Duration::from_secs(1800)).await?; // 30 minutes
            }
        }

        Ok(())
    }

    /// Block IP address
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn block_ip(&self, ip: IpAddr, duration: Duration) -> Result<()>  {
        let mut blocked_ips = self.blocked_ips.write().await;
        let unblock_time = SystemTime::now() + duration;
        blocked_ips.insert(ip, unblock_time);

        tracing::warn!(
            target: "intrusion_detection",
            ip = %ip,
            duration_secs = duration.as_secs(),
            "IP address blocked"
        );

        Ok(())
    }

    /// Check if IP is blocked
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn is_ip_blocked(&self, ip: IpAddr) -> Result<bool>  {
        let blocked_ips = self.blocked_ips.read().await;
        
        if let Some(&unblock_time) = blocked_ips.get(&ip) {
            Ok(SystemTime::now() < unblock_time)
        } else {
            Ok(false)
        }
    }

    /// Get intrusion detection statistics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_statistics(&self) -> Result<IntrusionDetectionStatistics>  {
        let failed_auth_attempts = self.failed_auth_attempts.read().await;
        let blocked_ips = self.blocked_ips.read().await;
        let now = SystemTime::now();

        let active_blocked_ips = blocked_ips.values()
            .filter(|&&unblock_time| unblock_time > now)
            .count();

        Ok(IntrusionDetectionStatistics {
            failed_auth_attempts: failed_auth_attempts.values()
                .map(|attempts| attempts.len() as u64)
                .sum(),
            blocked_ips_count: active_blocked_ips,
            suspicious_patterns_detected: 0, // Would track in production
            auto_blocked_ips: 0, // Would track in production
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_intrusion_detection() {
        let config = IntrusionDetectionConfig {
            enabled: true,
            failed_auth_threshold: 2,
            failed_auth_window: Duration::from_secs(60),
            pattern_detection: true,
            auto_blocking: false,
        };
        let ids = IntrusionDetectionSystem::new(config);

        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Record two failed authentication attempts
        ids.record_auth_failure(test_ip).await.unwrap();
        ids.record_auth_failure(test_ip).await.unwrap();

        let stats = ids.get_statistics().await.unwrap();
        assert_eq!(stats.failed_auth_attempts, 2);
    }

    #[tokio::test]
    async fn test_ip_blocking() {
        let config = IntrusionDetectionConfig {
            enabled: true,
            failed_auth_threshold: 5,
            failed_auth_window: Duration::from_secs(300),
            pattern_detection: true,
            auto_blocking: true,
        };
        let ids = IntrusionDetectionSystem::new(config);

        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // Initially not blocked
        assert!(!ids.is_ip_blocked(test_ip).await.unwrap());

        // Block IP
        ids.block_ip(test_ip, Duration::from_secs(60)).await.unwrap();

        // Should now be blocked
        assert!(ids.is_ip_blocked(test_ip).await.unwrap());
    }
} 