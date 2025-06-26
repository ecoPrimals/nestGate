# ZFS Filesystem Monitoring Implementation

## Overview

This specification outlines the implementation of ZFS-aware filesystem monitoring in NestGate. It builds on the existing filesystem monitor service to provide specialized handling for ZFS datasets, properties, and operations, enabling comprehensive monitoring across tiered storage systems.

## Goals

1. Monitor file operations across ZFS datasets with specialized handling
2. Support ZFS-specific operations (snapshots, clones, property changes)
3. Enable tiered storage monitoring with tier-specific configurations
4. Provide real-time event notification for backup and data migration
5. Support monitoring across multiple datasets with different properties
6. Ensure efficient performance with large ZFS datasets

## Architecture

```
┌─────────────────────┐     ┌───────────────────────┐     ┌────────────────┐
│ Filesystem Monitor  │◄────┤ Event Collection      │◄────┤ ZFS Filesystems│
│ Service             │     │ System                │     │                │
└─────────────┬───────┘     └───────────────────────┘     └────────────────┘
              │                                                   ▲
              ▼                                                   │
┌─────────────────────┐     ┌───────────────────────┐     ┌──────┴─────────┐
│ Event Consumers     │     │ Tier-Specific         │     │ ZFS Management │
│ (Backup, Migration) │◄────┤ Event Filters         │◄────┤ Commands       │
└─────────────────────┘     └───────────────────────┘     └────────────────┘
```

## Implementation Details

### ZFS-Aware Event Handling

The filesystem monitor has been enhanced to properly handle ZFS-specific aspects:

1. **Dataset Recognition**: Identify and track events from different ZFS datasets
2. **Snapshot Visibility**: Handle events from `.zfs/snapshot` directories
3. **Clone Operation Detection**: Track events from cloned datasets
4. **Property Change Handling**: Adapt to ZFS property changes (e.g., compression, readonly)

### Tiered Storage Support

The monitoring system supports the three-tier storage architecture:

1. **Hot Tier Configuration**:
   - High-sensitivity monitoring (all events)
   - LZ4 compression optimized
   - Full directory monitoring

2. **Warm Tier Configuration**:
   - Selective monitoring (exclude temporary files)
   - Standard compression
   - Filtered events

3. **Cold Tier Configuration**:
   - Minimal monitoring (specific file types only)
   - Maximum compression
   - Highly filtered events

### Event Filtering System

Enhanced filtering capabilities to support tier-specific requirements:

1. **Path-Based Filtering**: Include/exclude specific paths
2. **Extension Filtering**: Monitor only specific file types
3. **Pattern Matching**: Regular expression support for advanced filtering
4. **Event Type Filtering**: Focus on specific operations (create, modify, delete)
5. **Directory vs File Filtering**: Separate handling for directories and files

### Performance Optimizations

Specialized handling for efficient ZFS monitoring:

1. **Buffered Event Processing**: Batch events to reduce processing overhead
2. **Recursive Monitoring**: Efficient handling of nested directories
3. **Throttling Mechanisms**: Prevent event storms during high-activity periods
4. **Memory Usage Optimization**: Efficient event storage and processing

## Implementation Status

The ZFS filesystem monitoring functionality has been implemented and tested with the following components:

1. **Core Monitoring Service**: Complete
2. **ZFS-Specific Event Handling**: Complete
3. **Tiered Storage Integration**: Complete
4. **Advanced Filtering**: Complete
5. **Performance Optimizations**: Complete

## Testing Coverage

The implementation includes comprehensive testing:

1. **ZFS Operations Tests**: Testing of snapshot, clone, and property change operations
2. **Multi-Dataset Tests**: Testing monitoring across multiple ZFS datasets simultaneously
3. **Mixed Filesystem Tests**: Testing both regular and ZFS filesystem monitoring
4. **Performance Testing**: Testing with large datasets and high event rates
5. **Filter Effectiveness Tests**: Verifying filter behavior across tiers

## Configuration Options

The filesystem monitor service supports the following ZFS-specific configuration:

```rust
// Example configuration for tiered storage monitoring
let hot_filter = EventFilter::new()
    .include_directories(true)
    .include_hidden(true);

let warm_filter = EventFilter::new()
    .include_directories(true)
    .include_hidden(false)
    .exclude_patterns(vec![".tmp$".to_string()]);

let cold_filter = EventFilter::new()
    .include_directories(true)
    .include_hidden(false)
    .with_extensions(vec!["archive".to_string(), "backup".to_string()]);

// Register monitors for each tier
service.monitor_directory("/nestpool/hot", MonitorOptions {
    id: Some("hot-tier".to_string()),
    recursive: true,
    filter: hot_filter,
    on_event_command: Some("./scripts/hot_tier_event.sh".to_string()),
});
```

## Integration Points

The ZFS filesystem monitoring integrates with other system components:

1. **Backup Service**: Trigger backups based on file modifications
2. **Migration Service**: Initiate tier movement based on access patterns
3. **Notification System**: Alert on important file events
4. **Audit System**: Log critical file operations for compliance
5. **UI Components**: Provide real-time event visualization

## Deployment Requirements

For proper ZFS monitoring deployment:

1. **ZFS Permissions**: Appropriate ZFS delegation for monitoring user
2. **Inotify Limits**: Increased limits for large datasets
   ```bash
   echo "fs.inotify.max_user_watches=524288" | sudo tee -a /etc/sysctl.conf
   sudo sysctl -p
   ```
3. **Resource Allocation**: Sufficient memory and CPU for high-volume events
4. **ZFS Configuration**: Compatible dataset properties for monitoring

## Known Limitations

Current limitations of the ZFS monitoring implementation:

1. Direct ZFS property changes are not directly monitored (only their effects)
2. Very high event rates may require additional tuning
3. Some ZFS operations may generate multiple filesystem events
4. Remote ZFS datasets require additional configuration

## Future Enhancements

Planned improvements to the ZFS monitoring system:

1. Direct ZFS property change monitoring
2. Enhanced event deduplication
3. More advanced pattern matching capabilities
4. Improved performance metrics and monitoring
5. Machine learning for event pattern recognition

## Conclusion

The ZFS filesystem monitoring implementation provides a robust foundation for the tiered storage architecture. It enables comprehensive monitoring across all storage tiers with specialized handling for ZFS-specific operations, supporting critical features like real-time backup triggering and intelligent data migration between tiers. 