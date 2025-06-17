---
title: "TrueNAS ZFS Integration - Phase 1 Implementation Plan"
date: "2025-05-10"
status: "In Progress"
---

# TrueNAS ZFS Integration - Phase 1 Implementation Plan

## Overview

This document outlines the detailed implementation plan for Phase 1 (Backend Integration) of the TrueNAS ZFS Integration project. The focus is on developing the middleware components, API endpoints, database schema, and telemetry collection required for ZFS tier management in TrueNAS systems.

## Components and Tasks

### 1. TrueNAS Middleware Plugins

#### ZFS Tier Manager Plugin

```python
# middleware/plugins/zfs_tier_manager.py

import os
import json
from middlewared.service import CRUDService, private
from middlewared.schema import Dict, Str, Int, Bool, List
from middlewared.utils import run

class ZFSTierManagerService(CRUDService):
    class Config:
        namespace = 'nestgate.tiering'
        datastore = 'nestgate.tiering'
        datastore_prefix = 'tier_'
        private = True

    async def do_create(self, data):
        """Create a new tier configuration for a dataset."""
        # Implementation details
        dataset = data.get('dataset')
        tier = data.get('tier')
        
        # Get default properties for tier
        tier_props = await self._get_tier_properties(tier)
        
        # Apply properties to dataset
        for prop, value in tier_props.items():
            await self.middleware.call('zfs.dataset.update', dataset, {prop: value})
        
        # Add custom property for tier identification
        await self.middleware.call('zfs.dataset.update', dataset, 
                                  {'properties': {'nestgate:storage_tier': {'value': tier}}})
        
        # Store configuration in database
        pk = await self.middleware.call(
            'datastore.insert',
            self._config.datastore,
            {**data, 'properties': json.dumps(tier_props)}
        )
        
        return await self._get_instance(pk)
    
    @private
    async def _get_tier_properties(self, tier):
        """Get default properties for a specific tier."""
        if tier == 'hot':
            return {
                'recordsize': '128K',
                'compression': 'lz4',
                'primarycache': 'all',
                'sync': 'standard'
            }
        elif tier == 'warm':
            return {
                'recordsize': '1M',
                'compression': 'zstd',
                'primarycache': 'metadata',
                'sync': 'standard'
            }
        elif tier == 'cold':
            return {
                'recordsize': '1M',
                'compression': 'zstd-19',
                'primarycache': 'metadata',
                'sync': 'standard'
            }
        elif tier == 'cache':
            return {
                'recordsize': '128K',
                'compression': 'lz4',
                'primarycache': 'all',
                'sync': 'always'
            }
        else:
            raise ValueError(f"Unknown tier: {tier}")
```

#### AI Workload Detector Plugin

```python
# middleware/plugins/ai_workload_detector.py

import asyncio
import time
from middlewared.service import Service, job, private
from middlewared.schema import Dict, Str, Int, Bool
from middlewared.utils import run

class AIWorkloadDetectorService(Service):
    class Config:
        namespace = 'nestgate.aidetector'
        datastore = 'nestgate.aiworkload'
        datastore_prefix = 'ai_'
        private = True

    @job(lock='aidetector_scan')
    async def scan(self, job, options):
        """Scan datasets for AI workload patterns."""
        datasets = await self.middleware.call('zfs.dataset.query')
        
        for dataset in datasets:
            dataset_name = dataset['name']
            
            # Check if AI detection is enabled for this dataset
            config = await self.middleware.call(
                'datastore.query',
                'nestgate.aiworkload',
                [['dataset', '=', dataset_name]], 
                {'get': True, 'prefix': self._config.datastore_prefix}
            )
            
            if not config or not config['enabled']:
                continue
                
            # Collect workload metrics
            metrics = await self._collect_metrics(dataset_name)
            
            # Analyze workload pattern
            workload_type = await self._analyze_workload(metrics)
            
            if workload_type:
                # Apply optimizations for detected workload
                await self.middleware.call(
                    'nestgate.tiering.apply_workload_tuning',
                    {'dataset': dataset_name, 'workload': workload_type}
                )
                
                # Update detection record
                await self.middleware.call(
                    'datastore.update',
                    self._config.datastore,
                    config['id'],
                    {'last_detected': time.time(), 'workload_type': workload_type},
                    {'prefix': self._config.datastore_prefix}
                )
    
    @private
    async def _collect_metrics(self, dataset):
        """Collect performance metrics for a dataset."""
        # Implementation details for collecting metrics
        # This would use iostat, zpool iostat, or custom monitoring tools
        
        # For prototype: use basic filesystem stats
        stats = {}
        
        # Get read/write operations data
        zpool_stats = await run(['zpool', 'iostat', '-v', '1', '1'], check=False)
        
        # Get file access patterns
        dataset_path = os.path.join('/mnt', dataset.replace('/', '-'))
        access_patterns = await run(['lsof', dataset_path], check=False)
        
        # Process and return formatted metrics
        return {
            'read_ops': self._parse_read_ops(zpool_stats.stdout),
            'write_ops': self._parse_write_ops(zpool_stats.stdout),
            'access_patterns': self._parse_access_patterns(access_patterns.stdout)
        }
    
    @private
    async def _analyze_workload(self, metrics):
        """Analyze metrics to determine workload type."""
        # Implementation details for workload analysis
        # This would use heuristics or ML models to classify workloads
        
        # For prototype: use simple heuristics
        read_ops = metrics['read_ops']
        write_ops = metrics['write_ops']
        access_patterns = metrics['access_patterns']
        
        # Simple AI workload detection logic
        if read_ops > 1000 and write_ops < 100:
            return 'inference'
        elif read_ops > 500 and write_ops > 500:
            return 'training'
        elif read_ops < 100 and write_ops > 1000:
            return 'checkpointing'
        
        return None
```

#### Tier Operations Plugin

```python
# middleware/plugins/tier_operations.py

import os
import json
from middlewared.service import Service, job
from middlewared.schema import Dict, Str, Int, Bool, List
from middlewared.utils import run

class TierOperationsService(Service):
    class Config:
        namespace = 'nestgate.tierops'
        datastore = 'nestgate.tierops'
        datastore_prefix = 'op_'
        private = True

    @job(lock='tierops_migrate')
    async def migrate(self, job, options):
        """Migrate data between tiers."""
        source_dataset = options.get('source_dataset')
        target_tier = options.get('target_tier')
        
        # Get target tier properties
        tier_props = await self.middleware.call(
            'nestgate.tiering._get_tier_properties', 
            target_tier
        )
        
        # Create target dataset with appropriate properties
        target_dataset = f"{source_dataset}_migrate_{int(time.time())}"
        await self.middleware.call('zfs.dataset.create', {
            'name': target_dataset,
            'properties': tier_props
        })
        
        # Create snapshot of source
        snapshot_name = f"{source_dataset}@migrate_{int(time.time())}"
        await self.middleware.call('zfs.snapshot.create', {
            'dataset': source_dataset,
            'name': snapshot_name.split('@')[1]
        })
        
        # Send snapshot to target
        await self.middleware.call('zfs.snapshot.send', {
            'snapshot': snapshot_name,
            'target_dataset': target_dataset
        })
        
        # Record migration operation
        return await self.middleware.call(
            'datastore.insert',
            self._config.datastore,
            {
                'source': source_dataset,
                'target': target_dataset,
                'tier': target_tier,
                'time_started': time.time(),
                'time_completed': time.time(),
                'status': 'completed'
            },
            {'prefix': self._config.datastore_prefix}
        )
```

### 2. API Endpoints Implementation

```python
# api/resources/nestgate_tiering.py

from middlewared.schema import Dict, Str, Int, Bool, List, accepts
from middlewared.service import Service, CRUDService, job

class NestGateTieringAPI(CRUDService):
    class Config:
        namespace = 'nestgate.tiering.api'
        private = False

    @accepts()
    async def pools(self):
        """List pools with tiering information."""
        pools = await self.middleware.call('zfs.pool.query')
        
        # Enhance pool data with tiering information
        for pool in pools:
            # Get tier datasets within this pool
            tier_datasets = await self.middleware.call(
                'nestgate.tiering.get_tier_datasets',
                pool['name']
            )
            
            pool['tier_info'] = {
                'hot': next((d for d in tier_datasets if d['tier'] == 'hot'), None),
                'warm': next((d for d in tier_datasets if d['tier'] == 'warm'), None),
                'cold': next((d for d in tier_datasets if d['tier'] == 'cold'), None),
                'cache': next((d for d in tier_datasets if d['tier'] == 'cache'), None)
            }
        
        return pools

    @accepts()
    async def datasets(self):
        """List datasets and their tier status."""
        datasets = await self.middleware.call('zfs.dataset.query')
        
        # Enhance with tier information
        for dataset in datasets:
            if 'properties' in dataset and 'nestgate:storage_tier' in dataset['properties']:
                dataset['tier'] = dataset['properties']['nestgate:storage_tier']['value']
            else:
                dataset['tier'] = 'unassigned'
        
        return datasets

    @accepts(Str('dataset'), Dict(
        'tune_options',
        Str('tier', required=True, enum=['hot', 'warm', 'cold', 'cache']),
        Bool('apply_defaults', default=True),
    ))
    async def tune(self, dataset, options):
        """Apply tier-specific tuning to a dataset."""
        return await self.middleware.call('nestgate.tiering.do_create', {
            'dataset': dataset,
            'tier': options['tier'],
            'apply_defaults': options.get('apply_defaults', True)
        })

    @accepts(Str('dataset'), Dict(
        'autodetect_options',
        Bool('enabled', required=True),
        Int('sampling_period', default=3600),
        Int('min_samples', default=10),
    ))
    async def autodetect(self, dataset, options):
        """Enable AI workload detection for a dataset."""
        return await self.middleware.call(
            'datastore.insert',
            'nestgate.aiworkload',
            {
                'dataset': dataset,
                'enabled': options['enabled'],
                'sampling_period': options['sampling_period'],
                'min_samples': options['min_samples'],
                'last_detected': None,
                'workload_type': None
            },
            {'prefix': 'ai_'}
        )

    @accepts(Str('dataset'))
    async def stats(self, dataset):
        """Get tier performance statistics."""
        # Implementation details
        # This would collect and return performance metrics
        
        # For prototype: return basic ZFS stats
        stats = {}
        
        # Get basic dataset properties
        dataset_info = await self.middleware.call(
            'zfs.dataset.query',
            [['name', '=', dataset]], 
            {'get': True}
        )
        
        # Get performance metrics
        if 'properties' in dataset_info:
            stats['compressratio'] = dataset_info['properties'].get('compressratio', {}).get('value')
            stats['used'] = dataset_info['properties'].get('used', {}).get('value')
            stats['available'] = dataset_info['properties'].get('available', {}).get('value')
            
        # Get real-time IO stats
        io_stats = await self.middleware.call(
            'nestgate.tiering.get_io_stats', 
            dataset
        )
        
        stats['io'] = io_stats
        
        return stats

    @accepts(Dict(
        'migrate_options',
        Str('source_dataset', required=True),
        Str('target_tier', required=True, enum=['hot', 'warm', 'cold', 'cache']),
    ))
    @job(lock='tierops_migrate')
    async def migrate(self, job, options):
        """Migrate data between tiers."""
        return await self.middleware.call('nestgate.tierops.migrate', options)
```

### 3. Database Schema

```sql
-- Schema for TrueNAS PostgreSQL

-- Tier Configuration
CREATE TABLE nestgate_tiering (
    id SERIAL PRIMARY KEY,
    tier_dataset VARCHAR(255) NOT NULL UNIQUE,
    tier_tier VARCHAR(50) NOT NULL,  -- 'hot', 'warm', 'cold', or 'cache'
    tier_properties TEXT NOT NULL,   -- JSON string of ZFS properties
    tier_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tier_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- AI Workload Detection
CREATE TABLE nestgate_aiworkload (
    id SERIAL PRIMARY KEY,
    ai_dataset VARCHAR(255) NOT NULL UNIQUE,
    ai_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ai_sampling_period INTEGER NOT NULL DEFAULT 3600,
    ai_min_samples INTEGER NOT NULL DEFAULT 10,
    ai_last_detected TIMESTAMP,
    ai_workload_type VARCHAR(50),  -- 'training', 'inference', 'checkpointing', etc.
    ai_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ai_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tier Operations
CREATE TABLE nestgate_tierops (
    id SERIAL PRIMARY KEY,
    op_source VARCHAR(255) NOT NULL,
    op_target VARCHAR(255) NOT NULL,
    op_tier VARCHAR(50) NOT NULL,
    op_time_started TIMESTAMP NOT NULL,
    op_time_completed TIMESTAMP,
    op_status VARCHAR(50) NOT NULL,  -- 'in_progress', 'completed', 'failed'
    op_error_msg TEXT
);

-- Tier Telemetry
CREATE TABLE nestgate_telemetry (
    id SERIAL PRIMARY KEY,
    tel_dataset VARCHAR(255) NOT NULL,
    tel_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tel_read_ops INTEGER NOT NULL DEFAULT 0,
    tel_write_ops INTEGER NOT NULL DEFAULT 0,
    tel_read_bytes BIGINT NOT NULL DEFAULT 0,
    tel_write_bytes BIGINT NOT NULL DEFAULT 0,
    tel_read_latency FLOAT,
    tel_write_latency FLOAT,
    tel_access_pattern VARCHAR(50)  -- 'sequential', 'random', 'mixed'
);

-- Create indexes for performance
CREATE INDEX idx_tiering_dataset ON nestgate_tiering(tier_dataset);
CREATE INDEX idx_aiworkload_dataset ON nestgate_aiworkload(ai_dataset);
CREATE INDEX idx_telemetry_dataset ON nestgate_telemetry(tel_dataset);
CREATE INDEX idx_telemetry_timestamp ON nestgate_telemetry(tel_timestamp);
```

### 4. Telemetry Collection for Workload Patterns

```python
# middleware/plugins/telemetry_collector.py

import asyncio
import time
import re
from middlewared.service import Service, periodic, private
from middlewared.schema import Dict, Str, Int, Bool
from middlewared.utils import run

class TelemetryCollectorService(Service):
    class Config:
        namespace = 'nestgate.telemetry'
        datastore = 'nestgate.telemetry'
        datastore_prefix = 'tel_'
        private = True

    @periodic(300)
    async def collect_telemetry(self):
        """Collect telemetry data every 5 minutes."""
        # Get all datasets with AI workload detection enabled
        datasets = await self.middleware.call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        for dataset in datasets:
            dataset_name = dataset['dataset']
            
            # Collect performance metrics
            metrics = await self._collect_io_metrics(dataset_name)
            
            if metrics:
                # Store metrics in database
                await self.middleware.call(
                    'datastore.insert',
                    self._config.datastore,
                    {
                        'dataset': dataset_name,
                        'read_ops': metrics['read_ops'],
                        'write_ops': metrics['write_ops'],
                        'read_bytes': metrics['read_bytes'],
                        'write_bytes': metrics['write_bytes'],
                        'read_latency': metrics.get('read_latency'),
                        'write_latency': metrics.get('write_latency'),
                        'access_pattern': metrics.get('access_pattern', 'unknown')
                    },
                    {'prefix': self._config.datastore_prefix}
                )
    
    @private
    async def _collect_io_metrics(self, dataset):
        """Collect IO metrics for a specific dataset."""
        # Get the ZFS pool for this dataset
        pool = dataset.split('/')[0]
        
        # Run zpool iostat to get IO statistics
        iostat = await run(['zpool', 'iostat', '-v', pool, '1', '1'], check=False)
        if iostat.returncode != 0:
            self.logger.error(f"Error collecting IO stats for {dataset}: {iostat.stderr}")
            return None
            
        # Parse the output to extract metrics for this dataset
        return self._parse_iostat(iostat.stdout, dataset)
    
    @private
    def _parse_iostat(self, iostat_output, dataset):
        """Parse zpool iostat output to extract metrics."""
        # Implementation details to parse iostat output
        # This is a simplified version; real implementation would be more robust
        
        lines = iostat_output.split('\n')
        for i, line in enumerate(lines):
            if dataset in line:
                # Extract metrics from this line
                fields = re.split(r'\s+', line.strip())
                if len(fields) >= 7:
                    try:
                        read_ops = int(float(fields[1]))
                        write_ops = int(float(fields[2]))
                        read_bytes = int(float(fields[3]) * 1024)  # Convert K to bytes
                        write_bytes = int(float(fields[4]) * 1024)  # Convert K to bytes
                        
                        # Determine access pattern
                        if read_ops > 100 and read_bytes / max(read_ops, 1) > 64 * 1024:
                            access_pattern = 'sequential'
                        elif read_ops > 100:
                            access_pattern = 'random'
                        else:
                            access_pattern = 'mixed'
                            
                        return {
                            'read_ops': read_ops,
                            'write_ops': write_ops,
                            'read_bytes': read_bytes,
                            'write_bytes': write_bytes,
                            'access_pattern': access_pattern
                        }
                    except (ValueError, IndexError):
                        continue
        
        return None
```

## Integration Points

### TrueNAS Middleware Integration

The middleware plugins will interact with the following TrueNAS middleware services:

1. `zfs.dataset` - For dataset operations and property management
2. `zfs.pool` - For pool operations and statistics
3. `zfs.snapshot` - For snapshot creation and management
4. `datastore` - For database operations

### WebSocket API Integration

The API endpoints will be exposed via the TrueNAS WebSocket API framework:

```python
# api/ws.py additions

from aiohttp import web
from middlewared.restful import RESTfulResource

class NestGateTieringResource(RESTfulResource):
    name = 'nestgate/tiering'
    
    def __init__(self, middleware):
        super().__init__(middleware)
        
    async def on_get(self, req, options):
        if req.path == '/api/v2/nestgate/tiering/pools':
            return web.json_response(await self.middleware.call('nestgate.tiering.api.pools'))
        elif req.path == '/api/v2/nestgate/tiering/datasets':
            return web.json_response(await self.middleware.call('nestgate.tiering.api.datasets'))
        elif req.path.startswith('/api/v2/nestgate/tiering/stats/'):
            dataset = req.path.split('/')[-1]
            return web.json_response(await self.middleware.call('nestgate.tiering.api.stats', dataset))
        
        return web.json_response({})
    
    async def on_post(self, req, options):
        data = await req.json()
        
        if req.path.startswith('/api/v2/nestgate/tiering/tune/'):
            dataset = req.path.split('/')[-1]
            return web.json_response(await self.middleware.call('nestgate.tiering.api.tune', dataset, data))
        elif req.path.startswith('/api/v2/nestgate/tiering/autodetect/'):
            dataset = req.path.split('/')[-1]
            return web.json_response(await self.middleware.call('nestgate.tiering.api.autodetect', dataset, data))
        elif req.path == '/api/v2/nestgate/tiering/migrate':
            job_id = await self.middleware.call('nestgate.tiering.api.migrate', data)
            return web.json_response({'job_id': job_id})
            
        return web.json_response({})
```

## Testing Plan

1. **Unit Testing**: Individual component testing for middleware plugins
   - Test tier property application
   - Test AI workload detection algorithms
   - Test telemetry collection
   - Test migration operations

2. **Integration Testing**:
   - Test API endpoint functionality
   - Test database operations
   - Test interactions with TrueNAS middleware

3. **System Testing**:
   - Test complete workflows on TrueNAS VMs
   - Performance impact testing
   - Regression testing on core TrueNAS functionality

## Implementation Timeline

| Task | Duration | Dependencies |
|------|----------|--------------|
| Database Schema Implementation | 3 days | None |
| ZFS Tier Manager Plugin | 5 days | Database Schema |
| AI Workload Detector Plugin | 7 days | Database Schema |
| Tier Operations Plugin | 4 days | ZFS Tier Manager Plugin |
| Telemetry Collection Implementation | 5 days | Database Schema |
| API Endpoints Implementation | 5 days | All Plugins |
| WebSocket API Integration | 3 days | API Endpoints |
| Unit Testing | Ongoing | Each Component |
| Integration Testing | 5 days | All Components |
| System Testing | 3 days | Integration Testing |
| Documentation | 2 days | All Tasks |

**Total Duration**: 4 weeks (with some parallel development)

## Conclusion

This implementation plan outlines the key components, code structure, and integration points needed for Phase 1 of the TrueNAS ZFS Integration. The focus is on creating a solid backend foundation that will support the frontend components to be developed in Phase 2. The plan leverages TrueNAS's middleware architecture while introducing the specialized ZFS tier management capabilities from NestGate. 