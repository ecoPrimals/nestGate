// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Transport and protocol types shared across the ecosystem.
//!
//! G66 Transport Abstraction: platform-neutral endpoint resolution and
//! socket path conventions. Primals express *what* they connect to, not *how*.

mod endpoint;
pub mod jsonrpc;
mod socket;

pub use endpoint::{TransportEndpoint, TransportEndpointError};
pub use socket::{resolve_socket_path, socket_path_in};
