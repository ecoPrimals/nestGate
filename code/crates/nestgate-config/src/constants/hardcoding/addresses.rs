// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Named network address literals (IPv4/IPv6 loopback and “bind all”).
//!
//! These are stable identifiers, not magic numbers scattered through the codebase.
//! Prefer importing these constants when the semantic is “localhost” vs “all interfaces”.

/// IPv4 loopback (`127.0.0.1`). Use for same-host-only listeners unless configured otherwise.
pub const LOCALHOST_IPV4: &str = "127.0.0.1";

/// IPv6 loopback (`::1`). Use for same-host-only listeners on IPv6 stacks.
pub const LOCALHOST_IPV6: &str = "::1";

/// Conventional hostname for the local machine (`localhost`). Not a substitute for IP literals in binds.
pub const LOCALHOST_NAME: &str = "localhost";

/// Bind to all IPv4 interfaces (`0.0.0.0`). Use only when public or multi-interface binding is intentional.
pub const BIND_ALL_IPV4: &str = "0.0.0.0";

/// Bind to all IPv6 interfaces (`::`). Use only when public or multi-interface binding is intentional.
pub const BIND_ALL_IPV6: &str = "::";
