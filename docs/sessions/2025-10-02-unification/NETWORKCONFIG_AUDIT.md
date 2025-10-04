# NetworkConfig Consolidation Audit

**Date**: October 02, 2025
**Purpose**: Map all NetworkConfig variants for consolidation

---

## All NetworkConfig Struct Definitions

code/crates/nestgate-core/src/network/native_async/config.rs:9:pub struct NetworkConfig {
code/crates/nestgate-core/src/universal_primal_discovery/stubs.rs:88:pub struct NetworkConfigAdapter {
code/crates/nestgate-core/src/canonical_modernization/unified_types.rs:309:pub struct UnifiedNetworkConfig {
code/crates/nestgate-core/src/unified_minimal.rs:36:pub struct MinimalNetworkConfig {
code/crates/nestgate-core/src/config/canonical_unified/network_security.rs:16:pub struct NetworkConfig {
code/crates/nestgate-core/src/config/canonical_unified/builders.rs:224:pub struct NetworkConfigBuilder {
code/crates/nestgate-core/src/config/canonical/domain_configs/network_configs.rs:14:pub struct CanonicalNetworkConfig {
code/crates/nestgate-core/src/config/canonical/types.rs:62:pub struct NetworkConfig {
code/crates/nestgate-core/src/config/canonical/types.rs:90:pub struct InternalNetworkConfig {
code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs:53:pub struct CanonicalNetworkConfig {
code/crates/nestgate-core/src/config/canonical_master/network_config.rs:15:pub struct NetworkConfig<const API_PORT: u16 = 8080, const TIMEOUT_MS: u64 = 30000> {
code/crates/nestgate-core/src/config/canonical_master/network_config.rs:122:pub struct ExternalNetworkConfig {
code/crates/nestgate-core/src/unified_types/mod.rs:63:pub struct NetworkConfig {
code/crates/nestgate-core/src/zero_cost/const_generic_config.rs:225:pub struct ZeroCostNetworkConfig<
code/crates/nestgate-core/src/unified_fuzz_config.rs:202:pub struct FuzzNetworkConfigData {
code/crates/nestgate-core/src/config_root/mod.rs:91:pub struct NetworkConfig {
code/crates/nestgate-core/src/environment.rs:34:pub struct NetworkConfig {
code/crates/nestgate-core/src/canonical/types/config_registry.rs:59:pub struct CanonicalNetworkConfig {
code/crates/nestgate-core/src/test_config/environment.rs:35:pub struct NetworkConfig {
code/crates/nestgate-core/src/traits_root/config.rs:47:pub struct NetworkConfig {

## Usage Count by File (Top 20)

code/crates/nestgate-core/src/universal_primal_discovery/stubs.rs:10
code/crates/nestgate-core/src/config/validation.rs:10
code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs:9
code/crates/nestgate-core/src/config/canonical_master/network_config.rs:7
code/crates/nestgate-core/src/zero_cost/const_generic_config.rs:6
code/crates/nestgate-core/src/config/canonical_unified/builders.rs:6
code/crates/nestgate-core/src/canonical/types/config_registry.rs:6
code/crates/nestgate-core/src/unified_fuzz_config.rs:5
code/crates/nestgate-core/src/environment.rs:5
code/crates/nestgate-core/src/unified_minimal.rs:4
code/crates/nestgate-core/src/unified_config_consolidation.rs:4
code/crates/nestgate-core/src/unified_canonical_config.rs:4
code/crates/nestgate-core/src/config_root/mod.rs:4
code/crates/nestgate-core/src/config/canonical/types.rs:4
code/crates/nestgate-core/src/config/canonical/domain_configs/network_configs.rs:4
code/crates/nestgate-core/src/capabilities/discovery/unified_dynamic_config.rs:4
code/crates/nestgate-core/src/ecosystem_integration/universal_adapter/config.rs:3
code/crates/nestgate-core/src/config/core.rs:3
code/crates/nestgate-core/src/config/canonical_master/mod.rs:3
code/crates/nestgate-core/src/unified_types/mod.rs:2
