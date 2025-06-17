# Migration Plan for src/ Directory

## Files Already Migrated
These files have already been migrated to crates and can be removed from src/:

- [x] `src/ai_node.rs` -> Already migrated to `crates/nestgate-core/src/ai_node.rs`
- [x] `src/config.rs` -> Already migrated to `crates/nestgate-core/src/config.rs`
- [x] `src/error.rs` -> Already migrated to `crates/nestgate-core/src/error.rs`
- [x] `src/http/api.rs` -> Already migrated to `crates/nestgate-network/src/http/api.rs`
- [x] `src/http/middleware/api_key.rs` -> Already migrated to `crates/nestgate-network/src/http/middleware/api_key.rs`
- [x] `src/http/middleware/auth.rs` -> Already migrated to `crates/nestgate-network/src/http/middleware/auth.rs`
- [x] `src/http/middleware/error.rs` -> Already migrated to `crates/nestgate-network/src/http/middleware/error.rs`
- [x] `src/http/middleware/logging.rs` -> Already migrated to `crates/nestgate-network/src/http/middleware/logging.rs`
- [x] `src/http/middleware/mod.rs` -> Already migrated to `crates/nestgate-network/src/http/middleware/mod.rs`
- [x] `src/http/mod.rs` -> Already migrated to `crates/nestgate-network/src/http/mod.rs`
- [x] `src/http/websocket.rs` -> Already migrated to `crates/nestgate-network/src/http/websocket.rs`
- [x] `src/lib.rs` -> Already migrated to `crates/nestgate-core/src/lib.rs`
- [x] `src/main.rs` -> Already migrated to `crates/nestgate-core/src/main.rs`
- [x] `src/metrics.rs` -> Already migrated to `crates/nestgate-core/src/metrics.rs`
- [x] `src/mount.rs` -> Already migrated to `crates/nestgate-core/src/mount.rs`
- [x] `src/protocol/handlers/mod.rs` -> Already migrated to `crates/nestgate-core/src/protocol/handlers/mod.rs`
- [x] `src/protocol/iscsi/mod.rs` -> Already migrated to `crates/nestgate-core/src/protocol/iscsi/mod.rs`
- [x] `src/protocol/mod.rs` -> Already migrated to `crates/nestgate-core/src/protocol/mod.rs`
- [x] `src/protocol/nfs/common.rs` -> Already migrated to `crates/nestgate-network/src/protocol/nfs/common.rs`
- [x] `src/protocol/nfs/exports.rs` -> Already migrated to `crates/nestgate-core/src/protocol/nfs/exports.rs`
- [x] `src/protocol/nfs/mod.rs` -> Already migrated to `crates/nestgate-core/src/protocol/nfs/mod.rs`
- [x] `src/protocol/nfs/mounts.rs` -> Already migrated to `crates/nestgate-network/src/protocol/nfs/mounts.rs`
- [x] `src/protocol/smb/mod.rs` -> Already migrated to `crates/nestgate-core/src/protocol/smb/mod.rs`
- [x] `src/protocol_test.rs` -> Already migrated to `crates/nestgate-network/src/protocol_test.rs`
- [x] `src/protocol/utils.rs` -> Already migrated to `crates/nestgate-mcp/src/protocol/utils.rs`
- [x] `src/provider.rs` -> Already migrated to `crates/nestgate-mcp/src/provider.rs`
- [x] `src/scheduler.rs` -> Already migrated to `crates/nestgate-core/src/scheduler.rs`
- [x] `src/storage/cold.rs` -> Already migrated to `crates/nestgate-core/src/storage/cold.rs`
- [x] `src/storage/mod.rs` -> Already migrated to `crates/nestgate-core/src/storage/mod.rs`
- [x] `src/storage/zfs.rs` -> Already migrated to `crates/nestgate-core/src/storage/zfs.rs`
- [x] `src/tests/mod.rs` -> Already migrated to `crates/nestgate-core/src/tests/mod.rs`
- [x] `src/test_utils.rs` -> Already migrated to `crates/nestgate-mcp/src/test_utils.rs`
- [x] `src/types.rs` -> Already migrated to `crates/nestgate-core/src/types.rs`
- [x] `src/utils.rs` -> Already migrated to `crates/nestgate-core/src/utils.rs`
- [x] `src/volume.rs` -> Already migrated to `crates/nestgate-mcp/src/volume.rs`

## Files To Be Migrated
These files need to be migrated to appropriate crates:

- [ ] `src/bin/api_key_manager.rs` -> Should be migrated to `crates/nestgate-core/src/bin/api_key_manager.rs`
- [ ] `src/bin/test_api.rs` -> Should be migrated to `crates/nestgate-network/src/bin/test_api.rs`
- [ ] `src/config/tests.rs` -> Should be migrated to appropriate test directory in crates
- [ ] `src/errors.rs` -> Should be migrated to `crates/nestgate-core/src/errors.rs`
- [ ] `src/lib2.rs` -> Should be migrated to `crates/nestgate-core/src/lib2.rs`
- [ ] `src/libzfs/mod.rs` -> Should be migrated to `crates/nestgate-core/src/libzfs/mod.rs`
- [ ] `src/security/api_keys.rs` -> Should be migrated to appropriate test directory in crates
- [ ] `src/security/auth.rs` -> Should be migrated to `crates/nestgate-ui/src/security/auth.rs`
- [ ] `src/security/crypto.rs` -> Should be migrated to `crates/nestgate-core/src/security/crypto.rs`
- [ ] `src/security/mod.rs` -> Should be migrated to `crates/nestgate-ui/src/security/mod.rs`
- [ ] `src/storage/api_key.rs` -> Should be migrated to `crates/nestgate-ui/src/storage/api_key.rs`
- [ ] `src/storage/hot.rs` -> Should be migrated to `crates/nestgate-core/src/storage/hot.rs`
- [ ] `src/storage/warm.rs` -> Should be migrated to `crates/nestgate-ui/src/storage/warm.rs`
- [ ] `src/tests/api_client.rs` -> Should be migrated to `crates/nestgate-ui/src/tests/api_client.rs`
- [ ] `src/tests/api_errors.rs` -> Should be migrated to `crates/nestgate-network/src/tests/api_errors.rs`
- [ ] `src/tests/api_integration_test.rs` -> Should be migrated to `crates/nestgate-network/src/tests/api_integration_test.rs`
- [ ] `src/tests/api_test.rs` -> Should be migrated to `crates/nestgate-network/src/tests/api_test.rs`
- [ ] `src/tests/websocket_test.rs` -> Should be migrated to `crates/nestgate-network/src/tests/websocket_test.rs`
- [ ] `src/validation/mod.rs` -> Should be migrated to `crates/nestgate-network/src/validation/mod.rs`

## Non-Rust Files
These non-Rust files should also be considered:

- [ ] `src/app/api/zfsApi.ts` -> Determine appropriate location based on content
- [ ] `src/app/App.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/App.tsx`
- [ ] `src/app/components/common/DataSourceIndicator.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/components/common/DataSourceIndicator.tsx`
- [ ] `src/app/components/dashboard/StorageUsageCard.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/components/dashboard/StorageUsageCard.tsx`
- [ ] `src/app/components/layout/AppHeader.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/components/layout/AppHeader.tsx`
- [ ] `src/app/components/layout/AppLayout.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/components/layout/AppLayout.tsx`
- [ ] `src/app/components/layout/Sidebar.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/components/layout/Sidebar.tsx`
- [ ] `src/app/pages/Dashboard.tsx` -> Should be migrated to `crates/nestgate-ui/src/app/pages/Dashboard.tsx`
- [ ] `src/app/theme.ts` -> Determine appropriate location based on content
- [ ] `src/index.html` -> Should be migrated to `crates/nestgate-ui/src/index.html`
- [ ] `src/index.tsx` -> Should be migrated to `crates/nestgate-ui/src/index.tsx`
- [ ] `src/lib.rs.new` -> Determine appropriate location based on content

## Directories
These directories should be handled accordingly:

- [ ] `src/app/` -> Should be migrated to `crates/nestgate-ui/src/app/`
- [ ] `src/app/api/` -> Should be migrated to `crates/nestgate-ui/src/app/api/`
- [ ] `src/app/components/` -> Should be migrated to `crates/nestgate-ui/src/app/components/`
- [ ] `src/app/components/common/` -> Should be migrated to `crates/nestgate-ui/src/app/components/common/`
- [ ] `src/app/components/dashboard/` -> Should be migrated to `crates/nestgate-ui/src/app/components/dashboard/`
- [ ] `src/app/components/layout/` -> Should be migrated to `crates/nestgate-ui/src/app/components/layout/`
- [ ] `src/app/hooks/` -> Should be migrated to `crates/nestgate-ui/src/app/hooks/`
- [ ] `src/app/models/` -> Should be migrated to `crates/nestgate-ui/src/app/models/`
- [ ] `src/app/pages/` -> Should be migrated to `crates/nestgate-ui/src/app/pages/`
- [ ] `src/app/services/` -> Should be migrated to `crates/nestgate-ui/src/app/services/`
- [ ] `src/app/utils/` -> Should be migrated to `crates/nestgate-ui/src/app/utils/`
- [ ] `src/bin/` -> Should be migrated to `crates/nestgate-core/src/bin/`
- [ ] `src/config/` -> Should be migrated to `crates/nestgate-core/src/config/`
- [ ] `src/http/` -> Should be migrated to `crates/nestgate-network/src/http/`
- [ ] `src/http/middleware/` -> Should be migrated to `crates/nestgate-network/src/http/middleware/`
- [ ] `src/libzfs/` -> Should be migrated to `crates/nestgate-core/src/libzfs/`
- [ ] `src/protocol/` -> Already partially migrated, review remaining files
- [ ] `src/protocol/handlers/` -> Already partially migrated, review remaining files
- [ ] `src/protocol/iscsi/` -> Already partially migrated, review remaining files
- [ ] `src/protocol/nfs/` -> Already partially migrated, review remaining files
- [ ] `src/protocol/smb/` -> Already partially migrated, review remaining files
- [ ] `src/security/` -> Should be migrated to `crates/nestgate-core/src/security/`
- [ ] `src/storage/` -> Should be migrated to `crates/nestgate-core/src/storage/`
- [ ] `src/tests/` -> Should be migrated to appropriate test directories in crates
- [ ] `src/validation/` -> Should be migrated to `crates/nestgate-core/src/validation/`
