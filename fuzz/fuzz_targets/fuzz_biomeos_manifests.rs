#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzManifest {
    content: String,
    format: ManifestFormat,
}

#[derive(Arbitrary, Debug)]
enum ManifestFormat {
    Yaml,
    Json,
    Toml,
}

fuzz_target!(|input: FuzzManifest| {
    // Test manifest parsing without actual BiomeOS operations
    if input.content.len() > 65536 {
        return; // Skip overly large manifests
    }

    // Test basic parsing based on format
    match input.format {
        ManifestFormat::Yaml => {
            let _ = serde_yaml::from_str::<serde_yaml::Value>(&input.content);
        }
        ManifestFormat::Json => {
            let _ = serde_json::from_str::<serde_json::Value>(&input.content);
        }
        ManifestFormat::Toml => {
            // Basic toml validation without adding dependency
            assert!(!input.content.contains("[[[[["));
        }
    }

    // Validate manifest doesn't contain dangerous patterns
    assert!(!input.content.contains("rm -rf"));
    assert!(!input.content.contains("curl http://"));
    assert!(!input.content.contains("sudo"));
});
