// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// AVX2 vector width in bytes
    pub const AVX2_WIDTH: usize = 32;

    /// SSE2 vector width in bytes  
    pub const SSE2_WIDTH: usize = 16;

    /// SIMD memory alignment requirement
    pub const SIMD_ALIGNMENT: usize = 32;

    /// Minimum size for SIMD operations
    pub const MIN_SIMD_SIZE: usize = 64;

    /// Default SIMD batch size
    pub const SIMD_BATCH_SIZE: usize = 32;

    /// Cache line size for memory layout optimization
    pub const CACHE_LINE_SIZE: usize = 64;

    /// CRC table size
    pub const CRC_TABLE_SIZE: usize = 256;

    /// Compression header size for ZFS operations
    pub const COMPRESSION_HEADER_SIZE: usize = 12;
