// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Authentication module: configuration, hybrid external/local orchestration,
//! Security primal IPC, JWT validation (`RustCrypto`), and standalone token helpers.

mod auth_token_manager;
mod config;
mod hybrid_manager;
mod security_primal;

#[cfg(test)]
mod tests;

pub use auth_token_manager::AuthTokenManager;
pub use config::{AuthenticationConfig, LocalTokenConfig};
pub use hybrid_manager::HybridAuthenticationManager;
