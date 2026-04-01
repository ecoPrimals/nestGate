// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;

#[derive(Arbitrary, Debug)]
struct FuzzPath {
    path: String,
}

fuzz_target!(|input: FuzzPath| {
    // Test path validation and sanitization
    if input.path.len() > 4096 {
        return; // Skip overly long paths
    }

    // Test basic path operations
    let path_buf = PathBuf::from(&input.path);
    let _is_absolute = path_buf.is_absolute();
    let _is_relative = path_buf.is_relative();

    // Validate path doesn't contain dangerous patterns
    assert!(!input.path.contains("../../../"));
    assert!(!input.path.contains("..\\..\\..\\"));
    assert!(!input.path.contains('\0'));

    // Test path components
    let _components: Vec<_> = path_buf.components().collect();

    // Test path to string conversion
    let _path_str = path_buf.to_string_lossy();
});
