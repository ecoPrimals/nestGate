// **AUTHENTICATION STATISTICS**

use super::types::AuthStats;

/// Statistics manager
pub struct StatsManager {
    stats: AuthStats,
}
impl StatsManager {
    pub const fn new() -> Self {
        Self {
            stats: AuthStats::default(),
        }
    }
} 