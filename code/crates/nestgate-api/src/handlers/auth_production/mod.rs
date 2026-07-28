// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **AUTH HANDLERS - PRODUCTION IMPLEMENTATION**
//!
//! Real authentication using the in-crate `auth_manager::AuthManager` (aligned with
//! `nestgate-security` types) until a single capability integration path is wired.

mod auth_manager;
mod credential_validation;
mod handler;
mod session;
mod token_management;
mod types;

#[cfg(test)]
pub use credential_validation::{authenticate, create_user};
#[cfg(test)]
pub use handler::ProductionAuthHandler;
#[cfg(test)]
pub use token_management::create_api_key;
#[cfg(test)]
pub use types::{ApiKeyRequest, ApiKeyResponse, AuthCredentials, AuthResponse, CreateUserRequest};

#[cfg(test)]
mod tests;
