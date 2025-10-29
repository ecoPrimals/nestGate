// **ERROR DOMAINS**
//! Module definitions and exports.
// Domain-specific error types organized for better maintainability
//! and to keep individual files under the 2000-line complexity limit.

// **DOMAIN UNIFICATION COMPLETE** - All domain-specific errors have been
// successfully unified into the canonical error system in core_errors.rs.
// This consolidation provides:
// - Single source of truth for all error types
// - Consistent error handling patterns across domains
// - Simplified error propagation and handling
// - Reduced cognitive overhead for developers

// **MIGRATION COMPLETE**: The unified error system in variants/core_errors.rs
// now handles all domain-specific error cases with comprehensive coverage:
// - Network errors (connection, timeout, protocol)
// - Storage errors (ZFS, filesystem, backend)  
// - API errors (validation, authentication, authorization)
// - Security errors (encryption, certificates, access control)

// **RESULT**: Domain-specific error modules are no longer needed.
// The canonical error system provides complete coverage with better maintainability. 