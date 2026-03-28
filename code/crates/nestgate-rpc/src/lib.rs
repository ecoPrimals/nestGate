// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! JSON-RPC + tarpc IPC layer for primal-to-primal communication.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]

pub mod rpc;

pub use rpc::*;
