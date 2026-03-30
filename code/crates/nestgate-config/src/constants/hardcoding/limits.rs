// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Common buffer sizes and connection limits (compile-time defaults).

/// Default buffer size for I/O operations (64KB)
pub const BUFFER_SIZE_DEFAULT: usize = 65536;

/// Maximum buffer size for I/O operations (1MB)
pub const BUFFER_SIZE_MAX: usize = 1_048_576;

/// Default connection pool size
pub const CONNECTION_POOL_SIZE: usize = 10;

/// Maximum concurrent connections allowed
pub const MAX_CONNECTIONS: usize = 1000;

/// Default timeout in seconds
pub const TIMEOUT_SECS: u64 = 30;

/// Maximum number of retry attempts for failed operations
pub const MAX_RETRIES: u32 = 3;
