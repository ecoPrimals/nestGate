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

#[cfg(test)]
mod tests {
    use super::normalize_method;
    use std::borrow::Cow;

    #[test]
    fn normalize_leaves_canonical_methods_unchanged() {
        let m = "storage.object.store";
        match normalize_method(m) {
            Cow::Borrowed(s) => assert_eq!(s, m),
            Cow::Owned(_) => panic!("expected borrowed unchanged slice"),
        }
    }

    #[test]
    fn normalize_strips_single_legacy_prefix() {
        match normalize_method("nestgate.health.check") {
            Cow::Borrowed(s) => assert_eq!(s, "health.check"),
            Cow::Owned(_) => panic!("expected borrowed normalized slice"),
        }
    }

    #[test]
    fn normalize_strips_repeated_nestgate_prefixes() {
        match normalize_method("nestgate.nestgate.storage.store") {
            Cow::Borrowed(s) => assert_eq!(s, "storage.store"),
            Cow::Owned(_) => panic!("expected borrowed normalized slice"),
        }
    }

    #[test]
    fn normalize_empty_string() {
        match normalize_method("") {
            Cow::Borrowed(s) => assert_eq!(s, ""),
            Cow::Owned(_) => panic!("expected borrowed empty"),
        }
    }

    #[test]
    fn normalize_does_not_strip_non_prefix() {
        let m = "myapp.nestgate.service";
        match normalize_method(m) {
            Cow::Borrowed(s) => assert_eq!(s, m),
            Cow::Owned(_) => panic!("expected unchanged borrow"),
        }
    }

    #[test]
    fn normalize_only_nestgate_dot_is_stripped_completely() {
        match normalize_method("nestgate.") {
            Cow::Borrowed(s) => assert_eq!(s, ""),
            Cow::Owned(_) => panic!("expected borrowed empty remainder"),
        }
    }
}
