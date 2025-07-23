//! Penetration Testing Configuration

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PenetrationTestConfig {
    pub attack_intensity: u8,
    pub concurrent_attacks: u32,
    pub attack_duration: Duration,
    pub rate_limit_bypass_attempts: u32,
    pub auth_bypass_attempts: u32,
    pub fuzzing_iterations: u32,
    pub network_scan_timeout: Duration,
}

impl Default for PenetrationTestConfig {
    fn default() -> Self {
        Self {
            attack_intensity: 7,
            concurrent_attacks: 50,
            attack_duration: Duration::from_secs(30),
            rate_limit_bypass_attempts: 1000,
            auth_bypass_attempts: 500,
            fuzzing_iterations: 10000,
            network_scan_timeout: Duration::from_secs(10),
        }
    }
}
