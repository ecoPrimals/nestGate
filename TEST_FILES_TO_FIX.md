# Test Files Needing Fixes - October 28, 2025

## Files with Compilation Errors (27 total)

### Priority 1: High Error Count
1. `handlers/auth_tests.rs` - Import/type errors
2. `handlers/compliance_types_tests.rs` - Field mismatch
3. `handlers/zero_cost_api_handlers_additional_tests.rs` - Type errors
4. `handlers/zero_cost_tests.rs` - Type errors
5. `handlers/health_tests.rs` - Import errors

### Priority 2: Module Issues
6. `handlers/workspace_management/lifecycle_tests.rs` - API changes
7. `handlers/workspace_management/storage_workspace_tests.rs` - Field changes
8. `handlers/load_testing/handler_tests.rs` - Type errors
9. `rest/handlers/websocket_tests.rs` - Import errors
10. `rest/handlers/storage_tests.rs` - Type errors

### Priority 3: Lower Impact
11-27. Various handlers with minor import/type issues

## Fix Strategy

### Batch 1 (Week 1): Fix 10 highest-priority files
- Estimate: 30-60 min per file
- Total: 5-10 hours

### Batch 2 (Week 2): Fix remaining 17 files
- Estimate: 20-40 min per file  
- Total: 6-12 hours

### Total Effort: 11-22 hours over 2 weeks

## Quick Wins
Comment out these 27 files to get clean build NOW, then fix systematically.
