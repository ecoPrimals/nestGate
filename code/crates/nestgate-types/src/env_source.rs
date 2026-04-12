// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Injectable environment-variable source — replaces direct `std::env::var` in
//! production code so that tests can supply isolated, concurrent-safe overrides
//! without mutating the process environment.

use std::collections::HashMap;

/// Abstraction over environment variable reads.
///
/// Production code uses [`ProcessEnv`]; tests use [`MapEnv`] to avoid
/// process-global mutation and the `#[serial]` / `temp_env` antipattern.
pub trait EnvSource: Send + Sync {
    /// Returns the value of `key`, or `None` if absent.
    fn get(&self, key: &str) -> Option<String>;

    /// Returns all key-value pairs. Used for prefix scanning
    /// (e.g. `NESTGATE_CAPABILITY_*`).
    fn vars(&self) -> Vec<(String, String)>;

    /// Returns the value of `key`, or `default` if absent.
    fn get_or(&self, key: &str, default: &str) -> String {
        self.get(key).unwrap_or_else(|| default.to_string())
    }
}

/// Parses the value of `key` from `env` as `T`, returning `default` on absence
/// or parse failure. Separated from [`EnvSource`] to keep the trait
/// object-safe (generic return types are not dyn-compatible).
pub fn env_parsed<T: std::str::FromStr>(
    env: &(impl EnvSource + ?Sized),
    key: &str,
    default: T,
) -> T {
    env.get(key).and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// Returns `env[key]` or `default` when missing (injectable alternative to reading the process env).
#[must_use]
pub fn env_var_or_default(env: &(impl EnvSource + ?Sized), key: &str, default: &str) -> String {
    env.get(key).unwrap_or_else(|| default.to_string())
}

/// Reads from the real process environment via `std::env::var`.
#[derive(Debug, Clone, Copy)]
pub struct ProcessEnv;

impl EnvSource for ProcessEnv {
    fn get(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    fn vars(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
    }
}

/// In-memory environment — fully isolated, no process-global mutation.
///
/// ```
/// use nestgate_types::env_source::MapEnv;
/// use nestgate_types::env_source::EnvSource;
///
/// let env = MapEnv::from([("PORT", "9090")]);
/// assert_eq!(env.get("PORT"), Some("9090".into()));
/// assert_eq!(env.get("MISSING"), None);
/// ```
#[derive(Debug, Clone, Default)]
pub struct MapEnv(pub HashMap<String, String>);

impl MapEnv {
    /// Creates an empty `MapEnv`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: usize> From<[(&str, &str); N]> for MapEnv {
    fn from(pairs: [(&str, &str); N]) -> Self {
        Self(
            pairs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }
}

impl EnvSource for MapEnv {
    fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn vars(&self) -> Vec<(String, String)> {
        self.0.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

/// Blanket impl so `Arc<dyn EnvSource>` itself implements `EnvSource`.
impl<T: EnvSource + ?Sized> EnvSource for std::sync::Arc<T> {
    fn get(&self, key: &str) -> Option<String> {
        (**self).get(key)
    }

    fn vars(&self) -> Vec<(String, String)> {
        (**self).vars()
    }
}

/// Blanket impl so `&T` (including `&dyn EnvSource`) works ergonomically.
impl<T: EnvSource + ?Sized> EnvSource for &T {
    fn get(&self, key: &str) -> Option<String> {
        (**self).get(key)
    }

    fn vars(&self) -> Vec<(String, String)> {
        (**self).vars()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_env_reads_real_env() {
        let env = ProcessEnv;
        let path = env.get("PATH");
        assert!(path.is_some(), "PATH should exist in process env");
    }

    #[test]
    fn map_env_isolated() {
        let env = MapEnv::from([("FOO", "bar"), ("NUM", "42")]);
        assert_eq!(env.get("FOO"), Some("bar".into()));
        assert_eq!(env.get("NUM"), Some("42".into()));
        assert_eq!(env.get("MISSING"), None);
    }

    #[test]
    fn get_or_returns_default() {
        let env = MapEnv::new();
        assert_eq!(env.get_or("X", "fallback"), "fallback");
    }

    #[test]
    fn env_parsed_returns_typed_value() {
        let env = MapEnv::from([("PORT", "9090")]);
        assert_eq!(super::env_parsed::<u16>(&env, "PORT", 0), 9090);
        assert_eq!(super::env_parsed::<u16>(&env, "MISSING", 80), 80);
        let bad = MapEnv::from([("PORT", "not_a_number")]);
        assert_eq!(super::env_parsed::<u16>(&bad, "PORT", 80), 80);
    }

    #[test]
    fn arc_env_source_delegates() {
        let env: std::sync::Arc<dyn EnvSource> = std::sync::Arc::new(MapEnv::from([("K", "V")]));
        assert_eq!(env.get("K"), Some("V".into()));
    }
}
