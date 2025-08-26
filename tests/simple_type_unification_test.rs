/// Simple test to validate type unification fixes
use std::time::SystemTime;

// Test the unified types work correctly
use nestgate_automation::prediction::{AccessPattern, FileAnalysis, TierType};
use nestgate_core::Result;

#[tokio::test]
async fn test_unified_types_compile() -> Result<()> {
    println!("🧪 Testing unified type definitions...");

    // Test FileAnalysis struct (unified definition)
    let file_analysis = FileAnalysis {
        file_path: "/test/file.txt".to_string(),
        size_bytes: 1024,
        created_at: SystemTime::now(),
        modified_at: SystemTime::now(),
        accessed_at: SystemTime::now(),
        file_type: "test".to_string(),
    };

    // Test TierType enum (unified definition)
    let tier_type = TierType::Hot;
    println!("✅ TierType::Hot = {tier_type:?}");

    // Test AccessPattern struct (unified definition)
    let access_pattern = AccessPattern {
        accesses_last_24h: 10,
        accesses_last_week: 50,
        accesses_last_month: 200,
        total_accesses: 1000,
        last_access: SystemTime::now(),
        peak_access_times: vec![9, 14], // Hours in 24h format (u8)
        read_write_ratio: 0.8,
    };

    println!("✅ FileAnalysis path: {}", file_analysis.file_path);
    println!("✅ AccessPattern 24h: {}", access_pattern.accesses_last_24h);

    println!("🎉 Type unification validation passed!");
    Ok(())
}

#[tokio::test]
async fn test_nas_config_unified() -> Result<()> {
    use nestgate_nas::NasConfig;
    use std::path::PathBuf;

    println!("🌐 Testing unified NAS config...");

    let nas_config = NasConfig {
        smb_enabled: true,
        nfs_enabled: true,
        http_enabled: true,
        bind_address: "127.0.0.1".to_string(),
        smb_port: 445,
        nfs_port: 2049,
        http_port: 8080,
        share_root: PathBuf::from("/tmp/test"),
    };

    println!("✅ NAS config SMB enabled: {}", nas_config.smb_enabled);
    println!("✅ NAS config bind address: {}", nas_config.bind_address);

    println!("🎉 NAS config unification validation passed!");
    Ok(())
}
