# NestGate ZFS Tier Management for TrueNAS

This middleware plugin suite integrates NestGate's advanced ZFS tier tuning and AI workload detection capabilities with TrueNAS. The integration provides automated tier-specific tuning and AI workload detection for optimal storage performance with machine learning and AI workloads.

## Features

- **ZFS Tier Management**: Configure ZFS datasets with optimized properties for hot, warm, cold, and cache tiers
- **AI Workload Detection**: Automatically detect training, inference, and checkpointing workloads
- **Dynamic Tuning**: Apply workload-specific optimizations based on detected patterns
- **Telemetry Collection**: Collect and analyze performance metrics for tuning decisions
- **API Integration**: Expose tier management capabilities through TrueNAS APIs

## Installation

### Prerequisites

- TrueNAS SCALE 22.12.0 or later
- Administrative access to the TrueNAS system
- ZFS pools configured on the system

### Installation Steps

1. SSH into your TrueNAS system as root
2. Copy the plugin files to the middleware plugins directory:

```bash
mkdir -p /usr/local/lib/python3.9/site-packages/middlewared/plugins/nestgate
cp zfs_tier_manager.py ai_workload_detector.py telemetry_collector.py /usr/local/lib/python3.9/site-packages/middlewared/plugins/nestgate/
cp __init__.py /usr/local/lib/python3.9/site-packages/middlewared/plugins/nestgate/
```

3. Create the database schema:

```bash
mkdir -p /usr/local/etc/middleware/sql/
cp nestgate_schema.sql /usr/local/etc/middleware/sql/
sqlite3 /data/freenas-v1.db < /usr/local/etc/middleware/sql/nestgate_schema.sql
```

4. Restart the middleware service:

```bash
systemctl restart middlewared
```

5. Verify installation:

```bash
midclt call nestgate.tiering.get_tier_datasets <pool_name>
```

## Usage

### ZFS Tier Management

#### Configure a Dataset for a Specific Tier

```bash
# Apply hot tier properties to a dataset
midclt call nestgate.tiering.do_create '{"dataset": "tank/dataset1", "tier": "hot", "apply_defaults": true}'

# Apply warm tier properties to a dataset
midclt call nestgate.tiering.do_create '{"dataset": "tank/dataset1", "tier": "warm", "apply_defaults": true}'

# Apply cold tier properties to a dataset
midclt call nestgate.tiering.do_create '{"dataset": "tank/dataset1", "tier": "cold", "apply_defaults": true}'

# Apply cache tier properties to a dataset
midclt call nestgate.tiering.do_create '{"dataset": "tank/dataset1", "tier": "cache", "apply_defaults": true}'
```

#### List Tier-Configured Datasets in a Pool

```bash
midclt call nestgate.tiering.get_tier_datasets 'tank'
```

#### Get IO Statistics for a Dataset

```bash
midclt call nestgate.tiering.get_io_stats 'tank/dataset1'
```

### AI Workload Detection

#### Configure AI Workload Detection

```bash
# Enable AI workload detection for a dataset
midclt call nestgate.aidetector.configure '{"dataset": "tank/ml_data", "enabled": true, "sampling_period": 3600, "min_samples": 10, "auto_tune": true}'
```

#### Manually Detect Workload

```bash
midclt call nestgate.aidetector.manual_detect 'tank/ml_data'
```

#### Run Workload Detection Scan

```bash
midclt call nestgate.aidetector.scan
```

#### Get Detection Configuration

```bash
midclt call nestgate.aidetector.get_config 'tank/ml_data'
```

### Telemetry Collection

#### Get Telemetry History for a Dataset

```bash
midclt call nestgate.telemetry.get_history 'tank/ml_data' '{"limit": 50}'
```

#### Get Aggregated Telemetry Data

```bash
midclt call nestgate.telemetry.aggregate '{"dataset": "tank/ml_data", "metric": "read_ops", "aggregation": "avg", "interval": "hour"}'
```

#### Clean Up Old Telemetry Data

```bash
midclt call nestgate.telemetry.cleanup_telemetry
```

## API Endpoints

The following REST API endpoints are available:

- GET `/api/v2/nestgate/tiering/pools` - List pools with tiering information
- GET `/api/v2/nestgate/tiering/datasets` - List datasets and their tier status
- POST `/api/v2/nestgate/tiering/tune/{dataset}` - Apply tier-specific tuning to a dataset
- POST `/api/v2/nestgate/tiering/autodetect/{dataset}` - Enable AI workload detection for a dataset
- GET `/api/v2/nestgate/tiering/stats/{dataset}` - Get tier performance statistics
- POST `/api/v2/nestgate/tiering/migrate` - Migrate data between tiers

## Troubleshooting

### Logs

Check the middleware logs for any errors:

```bash
cat /var/log/middlewared.log | grep nestgate
```

### Common Issues

1. **Plugin Not Loading**: Ensure file permissions are correct and the middleware service has been restarted.

   ```bash
   chmod 644 /usr/local/lib/python3.9/site-packages/middlewared/plugins/nestgate/*.py
   systemctl restart middlewared
   ```

2. **Database Connection Errors**: Verify the database schema was properly created.

   ```bash
   sqlite3 /data/freenas-v1.db "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'nestgate%';"
   ```

3. **ZFS Command Errors**: Ensure the ZFS commands are available on the system.

   ```bash
   which zpool
   which zfs
   ```

4. **Permission Issues**: Verify the middleware has appropriate permissions to execute ZFS commands.

   ```bash
   midclt call core.get_jobs '[]'
   ```

## Uninstallation

To remove the plugin:

```bash
rm -rf /usr/local/lib/python3.9/site-packages/middlewared/plugins/nestgate
sqlite3 /data/freenas-v1.db "DROP TABLE nestgate_tiering; DROP TABLE nestgate_aiworkload; DROP TABLE nestgate_telemetry; DROP TABLE nestgate_tierops;"
systemctl restart middlewared
```

## Support

For support or more information, contact [support@nestgate.io](mailto:support@nestgate.io) or visit our [documentation website](https://docs.nestgate.io). 