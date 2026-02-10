#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

#[derive(Arbitrary, Debug, Serialize, Deserialize)]
struct FuzzData {
    id: u64,
    name: String,
    values: Vec<i32>,
    metadata: std::collections::HashMap<String, String>,
}

fuzz_target!(|input: FuzzData| {
    // Test serialization/deserialization roundtrips

    // JSON roundtrip
    if let Ok(json) = serde_json::to_string(&input) {
        if json.len() < 65536 {
            let _: Result<FuzzData, _> = serde_json::from_str(&json);
        }
    }

    // YAML roundtrip
    if let Ok(yaml) = serde_yaml_ng::to_string(&input) {
        if yaml.len() < 65536 {
            let _: Result<FuzzData, _> = serde_yaml_ng::from_str(&yaml);
        }
    }

    // Validate serialized data doesn't contain dangerous patterns
    if let Ok(json) = serde_json::to_string(&input) {
        assert!(!json.contains("__proto__"));
        assert!(!json.contains("constructor"));
    }
});
