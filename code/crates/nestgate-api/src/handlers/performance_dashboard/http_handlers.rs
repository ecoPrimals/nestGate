//
// **CANONICAL MODERNIZATION COMPLETE**: This file has been modularized from 917 lines
// into focused, maintainable modules under `endpoints/` directory.
//
// **MODULAR ORGANIZATION**:
// - All endpoint handlers moved to `endpoints/` subdirectory
// - Each module focuses on a specific aspect of performance monitoring
// - Improved maintainability with files under 200 lines each
// - Better separation of concerns and easier testing

// ==================== MODULE RE-EXPORTS ====================

/// Re-export all modular endpoints for backward compatibility
pub use endpoints::*;

/// Modular endpoint implementations
pub mod endpoints; 