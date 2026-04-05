// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Maximum number of registered services
    pub const MAX_SERVICES: usize = 1000;

    /// Maximum number of concurrent requests across all services
    pub const MAX_CONCURRENT_REQUESTS: usize = 10000;

    /// Statistics retention period in seconds (24 hours)
    pub const STATS_RETENTION_SECS: u64 = 86400;

    /// Health check interval in seconds
    pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 30;

    /// Maximum number of connections per service
    pub const MAX_CONNECTIONS: usize = 1000;

    /// Maximum message size in bytes
    pub const MAX_MESSAGE_SIZE: usize = 1024;

    /// Number of retry attempts for failed messages
    pub const MESSAGE_RETRY_ATTEMPTS: u32 = 3;

    /// Maximum number of active sessions
    pub const MAX_SESSIONS: usize = 1000;

    /// Session timeout in seconds
    pub const SESSION_TIMEOUT_SECS: u64 = 300;

    /// Session duration in seconds
    pub const SESSION_DURATION_SECS: u64 = 300;

    /// Protocol version number
    pub const PROTOCOL_VERSION: u32 = 1;

    /// Maximum number of workflows
    pub const MAX_WORKFLOWS: usize = 1000;

    /// Maximum number of concurrent workflow executions
    pub const MAX_CONCURRENT_EXECUTIONS: usize = 100;

    /// Workflow execution timeout in seconds
    pub const EXECUTION_TIMEOUT_SECS: u64 = 300;

    /// Maximum number of steps per workflow
    pub const MAX_WORKFLOW_STEPS: usize = 100;

    /// Service operation timeout in seconds
    pub const SERVICE_TIMEOUT_SECS: u64 = 300;
