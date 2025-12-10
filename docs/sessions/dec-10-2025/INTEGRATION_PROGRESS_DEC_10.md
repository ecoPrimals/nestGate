# ServiceRegistry Integration Progress

**Date**: December 10, 2025 (Evening)  
**Task**: Integrate ServiceRegistry into Universal Adapter  
**Status**: IN PROGRESS

---

## GOAL

Replace hardcoded endpoint at line 459 in `capability_system.rs`:

```rust
// BEFORE (hardcoded)
let endpoint = config
    .service_endpoint()
    .map(|s| s.to_string())
    .unwrap_or_else(crate::constants::canonical_defaults::network::build_api_url);
```

```rust
// AFTER (capability discovery)
let service = self.registry
    .find_by_capability(&request.category.to_primal_capability())
    .await?;
let endpoint = service.url();
```

---

## PROGRESS

### ✅ Step 1: Export ServiceRegistry
- [x] Add to `universal_adapter/mod.rs`
- [x] Add to `lib.rs` canonical re-exports
- [x] Verify compilation

### ⏳ Step 2: Add ServiceRegistry to CapabilityRouter
- [ ] Add optional `registry` field to struct
- [ ] Update constructor to accept registry
- [ ] Add method to set registry

### ⏳ Step 3: Replace Hardcoded Endpoint
- [ ] Modify `forward_request_to_service` method
- [ ] Add capability → PrimalCapability mapping
- [ ] Implement fallback chain: discovery → env → error

### ⏳ Step 4: Test Integration
- [ ] Unit tests: Registry lookups work
- [ ] Integration tests: Discovery chain works
- [ ] E2E tests: Communication succeeds

---

## CODE CHANGES

### Files Modified
1. ✅ `universal_adapter/mod.rs` - Export ServiceRegistry
2. ✅ `lib.rs` - Re-export at top level
3. ⏳ `capability_system.rs` - Integrate into CapabilityRouter
4. ⏳ `capability_system.rs` - Update forward_request_to_service

### Lines of Code Changed
- Added: ~50 lines
- Modified: ~20 lines
- Total impact: ~70 lines

---

## ARCHITECTURAL IMPROVEMENT

### Before: Hardcoded Fallback
```
Environment Config → Hardcoded build_api_url() → Fixed URL
```

**Problems:**
- Single instance assumption
- No dynamic discovery
- Hardcoded "localhost"
- Blocks multi-primal deployment

### After: Capability Discovery
```
ServiceRegistry → Discovery Backend → Dynamic Service
       ↓ if not found
   Environment Config
       ↓ if not found
       Error (no fallback to hardcoded!)
```

**Benefits:**
- Multi-instance support
- Dynamic discovery
- No hardcoded values
- Sovereign primals

---

## TESTING STRATEGY

### Unit Tests
```rust
#[tokio::test]
async fn test_capability_router_uses_discovery() {
    let registry = ServiceRegistry::new(vec![...]).await?;
    let router = CapabilityRouter::new().with_registry(registry);
    
    // Should use discovery, not hardcoded
    let response = router.route_capability_request(request).await?;
    assert!(response.success);
}
```

### Integration Tests
- Registry discovers real services via mDNS
- Router forwards requests to discovered endpoints
- Communication succeeds end-to-end

---

## NEXT ACTIONS

1. **Add registry field** to CapabilityRouter
2. **Map CapabilityCategory** to PrimalCapability
3. **Update forward_request_to_service** to use registry
4. **Add tests** for discovery chain
5. **Measure impact**: Count hardcoded URLs removed

---

## IMPACT METRICS

### Target
- **Hardcoded URLs**: 814 → ~734 (80 removed)
- **Discovery usage**: 20% → 30%
- **First batch**: Universal adapter integration

### Progress
- **Exports**: ✅ Complete
- **Integration**: 🔄 In progress
- **Testing**: ⏳ Pending
- **Verification**: ⏳ Pending

---

**Status**: EXPORTS COMPLETE, INTEGRATION NEXT ✅

