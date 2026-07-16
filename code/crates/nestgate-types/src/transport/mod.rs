// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Transport and protocol types shared across the ecosystem.

mod endpoint;
pub mod jsonrpc;

pub use endpoint::{TransportEndpoint, TransportEndpointError};
