// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Default values and environment override helpers (`NESTGATE_*`).

use nestgate_types::{EnvSource, ProcessEnv};

/// Get environment variable or return default value
pub(in crate::constants::consolidated) fn env_or(key: &str, default: &str) -> String {
    env_or_from_source(&ProcessEnv, key, default)
}

/// Like [`env_or`], but reads from an injectable [`EnvSource`].
pub(in crate::constants::consolidated) fn env_or_from_source(
    env: &(impl EnvSource + ?Sized),
    key: &str,
    default: &str,
) -> String {
    env.get(key).unwrap_or_else(|| default.to_string())
}

/// Get environment variable and parse, or return default value
pub(in crate::constants::consolidated) fn env_or_parse<T: std::str::FromStr>(
    key: &str,
    default: T,
) -> T {
    env_or_parse_from_source(&ProcessEnv, key, default)
}

/// Like [`env_or_parse`], but reads from an injectable [`EnvSource`].
pub(in crate::constants::consolidated) fn env_or_parse_from_source<T: std::str::FromStr>(
    env: &(impl EnvSource + ?Sized),
    key: &str,
    default: T,
) -> T {
    env.get(key).and_then(|v| v.parse().ok()).unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn env_or_returns_default_when_unset() -> Result<()> {
        let key = "NESTGATE_CONSOLIDATED_DEFAULTS_TEST_UNSET_ABC";
        crate::env_process::remove_var(key);
        assert_eq!(env_or(key, "fallback"), "fallback");
        Ok(())
    }

    #[test]
    fn env_or_returns_env_when_set() -> Result<()> {
        let key = "NESTGATE_CONSOLIDATED_DEFAULTS_TEST_SET_ABC";
        crate::env_process::set_var(key, "from_env");
        assert_eq!(env_or(key, "fallback"), "from_env");
        crate::env_process::remove_var(key);
        Ok(())
    }

    #[test]
    fn env_or_parse_uses_default_when_missing_or_invalid() -> Result<()> {
        let key = "NESTGATE_CONSOLIDATED_DEFAULTS_PARSE_ABC";
        crate::env_process::remove_var(key);
        assert_eq!(env_or_parse::<u16>(key, 42), 42);
        crate::env_process::set_var(key, "not_a_number");
        assert_eq!(env_or_parse::<u16>(key, 7), 7);
        crate::env_process::set_var(key, "8080");
        assert_eq!(env_or_parse::<u16>(key, 0), 8080);
        crate::env_process::remove_var(key);
        Ok(())
    }

    #[test]
    fn env_or_parse_accepts_valid_override() -> Result<()> {
        let key = "NESTGATE_CONSOLIDATED_DEFAULTS_PARSE_VALID_ABC";
        crate::env_process::set_var(key, "443");
        assert_eq!(env_or_parse::<u16>(key, 80), 443);
        crate::env_process::remove_var(key);
        Ok(())
    }
}
