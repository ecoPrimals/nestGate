// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Serializes mock-`PATH` integration tests so `zpool`/`zfs` resolution stays consistent.

#[cfg(test)]
use std::sync::OnceLock;
#[cfg(test)]
use tokio::sync::{Mutex, MutexGuard};

#[cfg(test)]
static CLI_PATH_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[cfg(test)]
pub async fn acquire() -> MutexGuard<'static, ()> {
    CLI_PATH_LOCK.get_or_init(|| Mutex::new(())).lock().await
}
