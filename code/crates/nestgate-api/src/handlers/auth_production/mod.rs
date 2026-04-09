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

pub use credential_validation::{authenticate, create_user};
pub use handler::ProductionAuthHandler;
pub use session::get_auth_status;
pub use token_management::{create_api_key, validate_api_key};
pub use types::{ApiKeyRequest, ApiKeyResponse, AuthCredentials, AuthResponse, CreateUserRequest};

#[cfg(test)]
mod tests;
