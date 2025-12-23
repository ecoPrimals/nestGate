// **AUTHENTICATION STATISTICS**

use super::types::AuthStats;

/// Statistics manager
pub struct StatsManager {
    stats: AuthStats,
}
impl StatsManager {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            stats: AuthStats::default(),
        }
    }
} 