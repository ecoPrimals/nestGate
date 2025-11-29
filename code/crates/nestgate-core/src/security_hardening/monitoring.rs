//! Security Monitoring and Threat Detection
//!
//! Real-time security event monitoring with threat pattern matching

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

/// **SECURITY MONITOR**
///
/// Security event monitoring and threat detection
pub struct SecurityMonitor {
    events: Mutex<Vec<SecurityEvent>>,
    threat_patterns: Vec<ThreatPattern>,
    stats: SecurityStats,
    max_events: usize,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event type
    pub event_type: SecurityEventType,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Source IP address
    pub source_ip: String,
    /// User identifier (if applicable)
    pub user_id: Option<String>,
    /// Human-readable description
    pub description: String,
    /// Severity level
    pub severity: SecuritySeverity,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Security event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    /// Authentication failure
    AuthenticationFailure,
    /// Authorization failure
    AuthorizationFailure,
    /// Input validation failure
    InputValidationFailure,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// Suspicious activity detected
    SuspiciousActivity,
    /// Data access violation
    DataAccessViolation,
    /// Configuration change
    ConfigurationChange,
    /// System intrusion attempt
    SystemIntrusion,
}

/// Security severity level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Threat pattern
#[derive(Debug, Clone)]
pub struct ThreatPattern {
    /// Pattern name
    pub name: String,
    /// Pattern regex
    pub pattern: String,
    /// Severity level
    pub severity: SecuritySeverity,
    /// Action to take
    pub action: ThreatAction,
}

/// Threat action
#[derive(Debug, Clone)]
pub enum ThreatAction {
    /// Log the threat
    Log,
    /// Block the request
    Block,
    /// Send alert
    Alert,
    /// Quarantine the source
    Quarantine,
}

#[derive(Debug, Default)]
struct SecurityStats {
    events_recorded: AtomicU64,
    threats_detected: AtomicU64,
    actions_taken: AtomicU64,
}

impl SecurityMonitor {
    /// Create a new security monitor
    #[must_use]
    pub fn new(max_events: usize) -> Self {
        let mut monitor = Self {
            events: Mutex::new(Vec::new()),
            threat_patterns: Vec::new(),
            stats: SecurityStats::default(),
            max_events,
        };
        
        monitor.add_default_patterns();
        monitor
    }
    
    fn add_default_patterns(&mut self) {
        self.threat_patterns.extend(vec![
            ThreatPattern {
                name: "SQL Injection".to_string(),
                pattern: "(?i)(union|select|insert|update|delete|drop|create|alter)".to_string(),
                severity: SecuritySeverity::High,
                action: ThreatAction::Block,
            },
            ThreatPattern {
                name: "XSS Attempt".to_string(),
                pattern: "(?i)(<script|javascript:|onclick|onerror)".to_string(),
                severity: SecuritySeverity::High,
                action: ThreatAction::Block,
            },
            ThreatPattern {
                name: "Path Traversal".to_string(),
                pattern: r"(\.\./|\.\.\\|%2e%2e%2f)".to_string(),
                severity: SecuritySeverity::Medium,
                action: ThreatAction::Block,
            },
            ThreatPattern {
                name: "Command Injection".to_string(),
                pattern: r"(;|\||&|`|\$\(|\${)".to_string(),
                severity: SecuritySeverity::High,
                action: ThreatAction::Block,
            },
        ]);
    }
    
    /// Record a security event
    pub fn record_event(&self, event: SecurityEvent) {
        self.stats.events_recorded.fetch_add(1, Ordering::Relaxed);
        
        // SAFE: Mutex lock with poisoned mutex recovery
        let mut events = self.events.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        events.push(event);
        
        // Trim if over capacity
        if events.len() > self.max_events {
            events.remove(0);
        }
    }
    
    /// Analyze input for threats
    pub fn analyze_input(&self, input: &str, source_ip: &str) -> ThreatAnalysisResult {
        for pattern in &self.threat_patterns {
            if let Ok(regex) = regex::Regex::new(&pattern.pattern) {
                if regex.is_match(input) {
                    self.stats.threats_detected.fetch_add(1, Ordering::Relaxed);
                    
                    let event = SecurityEvent {
                        event_type: SecurityEventType::SuspiciousActivity,
                        timestamp: SystemTime::now(),
                        source_ip: source_ip.to_string(),
                        user_id: None,
                        description: format!("Threat pattern '{}' detected", pattern.name),
                        severity: pattern.severity.clone(),
                        metadata: {
                            let mut meta = HashMap::new();
                            meta.insert("pattern".to_string(), pattern.name.clone());
                            meta.insert("input_sample".to_string(), input.chars().take(100).collect());
                            meta
                        },
                    };
                    
                    self.record_event(event);
                    
                    match pattern.action {
                        ThreatAction::Block => {
                            self.stats.actions_taken.fetch_add(1, Ordering::Relaxed);
                            return ThreatAnalysisResult::Blocked(pattern.name.clone());
                        }
                        ThreatAction::Alert => {
                            self.stats.actions_taken.fetch_add(1, Ordering::Relaxed);
                            return ThreatAnalysisResult::Alert(pattern.name.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
        
        ThreatAnalysisResult::Safe
    }
    
    /// Get security events with optional severity filter
    pub fn get_events(&self, severity_filter: Option<SecuritySeverity>) -> Vec<SecurityEvent> {
        // SAFE: Mutex lock with poisoned mutex recovery
        let events = self.events.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        
        match severity_filter {
            Some(severity) => events.iter()
                .filter(|e| std::mem::discriminant(&e.severity) == std::mem::discriminant(&severity))
                .cloned()
                .collect(),
            None => events.clone(),
        }
    }
    
    /// Get security statistics
    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.stats.events_recorded.load(Ordering::Relaxed),
            self.stats.threats_detected.load(Ordering::Relaxed),
            self.stats.actions_taken.load(Ordering::Relaxed),
        )
    }
}

/// Threat analysis result
#[derive(Debug)]
pub enum ThreatAnalysisResult {
    /// Input is safe
    Safe,
    /// Alert generated
    Alert(String),
    /// Request blocked
    Blocked(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_monitor() {
        let monitor = SecurityMonitor::new(1000);
        
        let event = SecurityEvent {
            event_type: SecurityEventType::AuthenticationFailure,
            timestamp: SystemTime::now(),
            source_ip: "192.168.1.1".to_string(),
            user_id: Some("testuser".to_string()),
            description: "Failed login attempt".to_string(),
            severity: SecuritySeverity::Medium,
            metadata: HashMap::new(),
        };
        
        monitor.record_event(event);
        
        let (recorded, _, _) = monitor.stats();
        assert_eq!(recorded, 1);
    }
    
    #[test]
    fn test_threat_detection() {
        let monitor = SecurityMonitor::new(1000);
        
        // Should detect SQL injection
        match monitor.analyze_input("SELECT * FROM users", "test_ip") {
            ThreatAnalysisResult::Blocked(_) => {},
            _ => panic!("Should block SQL injection"),
        }
        
        // Should allow normal input
        match monitor.analyze_input("normal search query", "test_ip") {
            ThreatAnalysisResult::Safe => {},
            _ => panic!("Should allow normal input"),
        }
    }
}

