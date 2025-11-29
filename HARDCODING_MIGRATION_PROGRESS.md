# Hardcoding Migration Progress Report

**Generated**: 2025-11-28 15:03:34  
**Status**: In Progress

## Summary

| Category | Count | Status |
|----------|-------|--------|
| Hardcoded Ports | 735 | 🔄 Pending |
| Hardcoded IPs | 314 | 🔄 Pending |
| DEFAULT_ Constants | 194 | 🔄 Pending |

## Migration Strategy

### Phase 1: Port Migration (Estimated: 3-4 days)
Replace hardcoded ports with configuration values:

```rust
// BEFORE:
let addr = "0.0.0.0:8080".parse().unwrap();

// AFTER:
let config = Config::load()?;
let addr = format!("{}:{}", config.api.host, config.api.port)
    .parse()
    .map_err(|e| NestGateError::Configuration(e))?;
```

### Phase 2: IP/Hostname Migration (Estimated: 2-3 days)
Replace hardcoded addresses with configuration:

```rust
// BEFORE:
let redis_url = "redis://127.0.0.1:6379";

// AFTER:
let config = Config::load()?;
let redis_url = format!(
    "redis://{}:{}", 
    config.database.redis.host, 
    config.database.redis.port
);
```

### Phase 3: Constants Migration (Estimated: 5-6 days)
Move DEFAULT_ constants to configuration:

```rust
// BEFORE:
const DEFAULT_TIMEOUT: u64 = 30;

// AFTER:
// In config struct:
pub struct NetworkConfig {
    pub connect_timeout_seconds: u64,
}

// Usage:
let timeout = Duration::from_secs(config.network.connect_timeout_seconds);
```

## Priority Files to Migrate

### High Priority (API Handlers)
- [ ] `code/crates/nestgate-api/src/main.rs` - API server binding
- [ ] `code/crates/nestgate-api/src/bin/nestgate-api-server.rs` - Server initialization

### Medium Priority (Core Services)
- [ ] `code/crates/nestgate-core/src/network/client.rs` - Network client
- [ ] `code/crates/nestgate-core/src/config/network_defaults.rs` - Network defaults
- [ ] `code/crates/nestgate-core/src/constants/port_defaults.rs` - Port constants

### Low Priority (Tests & Examples)
- [ ] Test files (acceptable to keep hardcoded for testing)
- [ ] Example files (acceptable to show specific values)

## Timeline

- **Week 1**: Port migration (days 1-4)
- **Week 2**: IP/hostname migration (days 5-7) + Constants migration start (days 8-10)
- **Week 3**: Constants migration completion (days 11-14)
- **Total**: 10-14 days

## Testing Strategy

1. **Unit Tests**: Verify config loading
2. **Integration Tests**: Test with different config values
3. **E2E Tests**: Deploy with various configurations
4. **Backward Compatibility**: Support environment variables as fallback

## Next Steps

1. Run this script to generate base configuration
2. Review generated `config/canonical-master-generated.toml`
3. Start with high-priority files
4. Add tests for each migrated component
5. Update documentation

