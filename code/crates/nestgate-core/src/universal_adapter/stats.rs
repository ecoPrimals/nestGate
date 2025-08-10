/// Universal Adapter Statistics and Metrics
use serde::{Deserialize, Serialize};

/// Adapter statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdapterStats {
    /// Number of discovered security providers
    pub security_providers: usize,
    /// Number of discovered orchestration providers
    pub orchestration_providers: usize,
    /// Number of discovered compute providers
    pub compute_providers: usize,
    /// Total discovery attempts
    pub discovery_attempts: u64,
    /// Successful discoveries
    pub successful_discoveries: u64,
    /// Last discovery time
    pub last_discovery: Option<std::time::SystemTime>,
}

impl AdapterStats {
    /// Create new adapter statistics
    pub fn new() -> Self {
        Self::default()
    }
}

/// Get total discovery attempts count
pub fn get_discovery_attempts() -> u64 {
    static DISCOVERY_ATTEMPTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    DISCOVERY_ATTEMPTS.load(std::sync::atomic::Ordering::Relaxed)
}

/// Get successful discoveries count
pub fn get_successful_discoveries() -> u64 {
    static SUCCESSFUL_DISCOVERIES: std::sync::atomic::AtomicU64 =
        std::sync::atomic::AtomicU64::new(0);
    SUCCESSFUL_DISCOVERIES.load(std::sync::atomic::Ordering::Relaxed)
}

/// Get last discovery time
pub fn get_last_discovery_time() -> Option<std::time::SystemTime> {
    static LAST_DISCOVERY: std::sync::OnceLock<std::sync::Mutex<Option<std::time::SystemTime>>> =
        std::sync::OnceLock::new();
    let mutex = LAST_DISCOVERY.get_or_init(|| std::sync::Mutex::new(None));
    mutex.lock().map(|guard| *guard).unwrap_or(None)
}

/// Increment discovery attempts counter
pub fn increment_discovery_attempts() {
    static DISCOVERY_ATTEMPTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    DISCOVERY_ATTEMPTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

/// Increment successful discoveries counter and update last discovery time
pub fn increment_successful_discoveries() {
    static SUCCESSFUL_DISCOVERIES: std::sync::atomic::AtomicU64 =
        std::sync::atomic::AtomicU64::new(0);
    SUCCESSFUL_DISCOVERIES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // Update last discovery time
    static LAST_DISCOVERY: std::sync::OnceLock<std::sync::Mutex<Option<std::time::SystemTime>>> =
        std::sync::OnceLock::new();
    let mutex = LAST_DISCOVERY.get_or_init(|| std::sync::Mutex::new(None));
    if let Ok(mut last_discovery) = mutex.lock() {
        *last_discovery = Some(std::time::SystemTime::now());
    }
}
