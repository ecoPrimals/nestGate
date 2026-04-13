// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared JSON-RPC protocol helpers (method name normalization, etc.).

use std::borrow::Cow;

/// Normalize a JSON-RPC method name by stripping legacy prefixes.
///
/// Older clients may send `nestgate.storage.store` instead of `storage.store`.
/// This function strips the `nestgate.` prefix (repeatedly, defensively) so
/// the dispatch table only needs canonical short-form entries.
#[must_use]
pub fn normalize_method(method: &str) -> Cow<'_, str> {
    let mut s = method;
    while let Some(rest) = s.strip_prefix("nestgate.") {
        s = rest;
    }
    if s.len() == method.len() {
        Cow::Borrowed(method)
    } else {
        Cow::Borrowed(s)
    }
}
