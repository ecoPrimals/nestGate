# Critical Fixes Batch 1 - Cache Variant Patterns

## Target: Fix all missing Cache variant patterns (10+ errors)

### Files to Fix:

1. **migration.rs** - Line 876 and 892
   - Add Cache variant to both match statements
   
2. **ai_integration.rs** - Line 710  
   - Add Cache variant to tier benefit estimation

3. **manager.rs** - Lines 670, 691, 910
   - Add Cache variant to tier analysis matches

## Fix Implementation:

### migration.rs fixes:
```rust
// Line 876 - ensure_target_dataset_exists
match tier {
    StorageTier::Hot => { /* existing */ },
    StorageTier::Warm => { /* existing */ },
    StorageTier::Cold => { /* existing */ },
    StorageTier::Cache => {
        properties.insert("compression".to_string(), "off".to_string());
        properties.insert("recordsize".to_string(), "64K".to_string());
    }
}

// Line 892 - core_tier conversion
let core_tier = match tier {
    StorageTier::Hot => nestgate_core::StorageTier::Hot,
    StorageTier::Warm => nestgate_core::StorageTier::Warm,
    StorageTier::Cold => nestgate_core::StorageTier::Cold,
    StorageTier::Cache => nestgate_core::StorageTier::Cache,
};
```

### ai_integration.rs fixes:
```rust
// Line 710 - estimate_tier_benefits
match tier {
    StorageTier::Hot => Ok("storage/hot".to_string()),
    StorageTier::Warm => Ok("storage/warm".to_string()),
    StorageTier::Cold => Ok("storage/cold".to_string()),
    StorageTier::Cache => Ok("storage/cache".to_string()),
}
```

## Status: Ready to implement
## Impact: Will fix 10+ compilation errors
## Next: Batch 2 - Missing imports and types 