//! 🤖 **SOVEREIGN SCIENCE AI AGENT SIMULATION SUITE**
//!
//! Comprehensive AI agent performance testing that simulates:
//! - Multiple AI agents performing human-like workflows via API
//! - Concurrent agent operations with realistic usage patterns
//! - Performance benchmarking under various load conditions
//! - Error handling and recovery in agent-driven scenarios
//! - Multi-modal AI agent interactions (text, file operations, analysis)
//! - Learning pattern simulation and optimization validation
//! - Resource utilization and efficiency measurement
//! - Agent coordination and conflict resolution

mod ai_agent_simulation;

// Re-export the simulation functionality
pub use ai_agent_simulation::*;

// Include the tests from the module
pub use ai_agent_simulation::tests::*;
