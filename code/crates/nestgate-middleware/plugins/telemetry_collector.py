#!/usr/bin/env python3

import asyncio
import time
import logging
import re
import os
from middlewared.schema import Dict, Str, Int, Bool, accepts
from middlewared.service import Service, periodic, private, job
from middlewared.utils import run

logger = logging.getLogger('telemetry_collector')

class TelemetryCollectorService(Service):
    class Config:
        namespace = 'nestgate.telemetry'
        datastore = 'nestgate.telemetry'
        datastore_prefix = 'tel_'
        private = True
        verbose_name = 'ZFS Telemetry Collector'
        cli_namespace = 'storage.nestgate.telemetry'
    
    @periodic(300)  # Run every 5 minutes
    async def collect_telemetry(self):
        """
        Collect telemetry data for all datasets with AI workload detection enabled.
        """
        # Get all datasets with AI detection enabled
        enabled_datasets = await self.middleware.call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        if not enabled_datasets:
            logger.debug("No datasets with AI detection enabled, skipping telemetry collection")
            return
        
        # Dictionary to collect pool data to avoid multiple zpool iostat calls
        pool_data = {}
        
        for config in enabled_datasets:
            dataset_name = config['dataset']
            
            # Verify the dataset still exists
            exists = await self._dataset_exists(dataset_name)
            if not exists:
                logger.warning(f"Dataset {dataset_name} no longer exists, skipping telemetry collection")
                continue
            
            # Get the pool for this dataset
            pool = dataset_name.split('/')[0]
            
            # Collect pool data if not already collected
            if pool not in pool_data:
                iostat = await self._collect_pool_iostat(pool)
                if iostat:
                    pool_data[pool] = iostat
                else:
                    logger.error(f"Failed to collect IO stats for pool {pool}")
                    continue
            
            # Parse the data for this dataset
            metrics = await self._parse_dataset_metrics(pool_data[pool], dataset_name)
            
            if not metrics:
                logger.warning(f"No metrics data found for {dataset_name}")
                continue
            
            # Augment with additional dataset properties
            try:
                dataset_info = await self.middleware.call(
                    'zfs.dataset.query',
                    [['id', '=', dataset_name]],
                    {'get': True}
                )
                
                if dataset_info and 'properties' in dataset_info:
                    metrics['compressratio'] = float(dataset_info['properties'].get('compressratio', {}).get('value', '1.0'))
                    metrics['recordsize'] = dataset_info['properties'].get('recordsize', {}).get('value')
            except Exception as e:
                logger.error(f"Failed to get dataset properties for {dataset_name}: {str(e)}")
            
            # Store telemetry in database
            try:
                record_id = await self.middleware.call(
                    'datastore.insert',
                    self._config.datastore,
                    {
                        'dataset': dataset_name,
                        'timestamp': time.time(),
                        'read_ops': metrics.get('read_ops', 0),
                        'write_ops': metrics.get('write_ops', 0),
                        'read_bytes': metrics.get('read_bytes', 0),
                        'write_bytes': metrics.get('write_bytes', 0),
                        'read_latency': metrics.get('read_latency'),
                        'write_latency': metrics.get('write_latency'),
                        'access_pattern': metrics.get('access_pattern', 'unknown')
                    },
                    {'prefix': self._config.datastore_prefix}
                )
                logger.debug(f"Stored telemetry for {dataset_name}")
            except Exception as e:
                logger.error(f"Failed to store telemetry for {dataset_name}: {str(e)}")
    
    @job(lock='telemetry_cleanup')
    async def cleanup_telemetry(self, job, options=None):
        """
        Clean up old telemetry data.
        
        Args:
            options (dict, optional): Cleanup options
                max_age (int): Maximum age in days (default: 30)
                max_records (int): Maximum records per dataset (default: 1000)
        """
        options = options or {}
        max_age = options.get('max_age', 30)
        max_records = options.get('max_records', 1000)
        
        # Convert days to timestamp
        max_age_timestamp = time.time() - (max_age * 86400)
        
        # Delete records older than max_age
        deleted_by_age = await self.middleware.call(
            'datastore.delete',
            self._config.datastore,
            [['timestamp', '<', max_age_timestamp]],
            {'prefix': self._config.datastore_prefix}
        )
        
        # Get all unique datasets
        datasets = await self.middleware.call(
            'datastore.sql',
            f"SELECT DISTINCT tel_dataset FROM {self._config.datastore}"
        )
        
        total_deleted = deleted_by_age
        
        # For each dataset, limit the number of records
        for dataset_row in datasets:
            dataset = dataset_row[0]
            
            # Count records for this dataset
            count = await self.middleware.call(
                'datastore.count',
                self._config.datastore,
                [['dataset', '=', dataset]],
                {'prefix': self._config.datastore_prefix}
            )
            
            if count > max_records:
                # Get IDs of oldest records to delete
                to_delete = count - max_records
                
                old_records = await self.middleware.call(
                    'datastore.query',
                    self._config.datastore,
                    [['dataset', '=', dataset]],
                    {'prefix': self._config.datastore_prefix, 'order_by': ['timestamp'], 'limit': to_delete}
                )
                
                # Delete oldest records
                for record in old_records:
                    await self.middleware.call(
                        'datastore.delete',
                        self._config.datastore,
                        record['id'],
                        {'prefix': self._config.datastore_prefix}
                    )
                    total_deleted += 1
        
        logger.info(f"Telemetry cleanup: deleted {total_deleted} records")
        job.set_progress(100, f"Deleted {total_deleted} records")
        return total_deleted
    
    @accepts(Str('dataset'), Dict(
        'time_range',
        Int('start', required=False),
        Int('end', required=False),
        Int('limit', required=False, default=100),
        register=True
    ))
    async def get_history(self, dataset, options=None):
        """
        Get telemetry history for a dataset.
        
        Args:
            dataset (str): ZFS dataset name
            options (dict, optional): Query options
                start (int): Start timestamp
                end (int): End timestamp
                limit (int): Maximum records to return
        
        Returns:
            list: Telemetry history records
        """
        options = options or {}
        limit = options.get('limit', 100)
        
        filters = [['dataset', '=', dataset]]
        
        if 'start' in options:
            filters.append(['timestamp', '>=', options['start']])
        
        if 'end' in options:
            filters.append(['timestamp', '<=', options['end']])
        
        records = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            filters,
            {
                'prefix': self._config.datastore_prefix,
                'order_by': ['-timestamp'],
                'limit': limit
            }
        )
        
        # Calculate some derived metrics
        for record in records:
            # Convert timestamp to ISO format for better readability
            record['iso_time'] = time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(record['timestamp']))
            
            # Calculate bytes per operation
            if record['read_ops'] > 0:
                record['bytes_per_read'] = record['read_bytes'] / record['read_ops']
            else:
                record['bytes_per_read'] = 0
                
            if record['write_ops'] > 0:
                record['bytes_per_write'] = record['write_bytes'] / record['write_ops']
            else:
                record['bytes_per_write'] = 0
        
        return records
    
    @accepts(Dict(
        'aggregate_options',
        Str('dataset', required=True),
        Str('metric', required=True, enum=['read_ops', 'write_ops', 'read_bytes', 'write_bytes']),
        Str('aggregation', required=True, enum=['avg', 'max', 'min', 'sum']),
        Str('interval', required=True, enum=['hour', 'day', 'week', 'month']),
        Int('start', required=False),
        Int('end', required=False),
        register=True
    ))
    async def aggregate(self, options):
        """
        Aggregate telemetry data for a dataset.
        
        Args:
            options (dict): Aggregation options
                dataset (str): ZFS dataset name
                metric (str): Metric to aggregate
                aggregation (str): Aggregation function
                interval (str): Time interval
                start (int, optional): Start timestamp
                end (int, optional): End timestamp
        
        Returns:
            list: Aggregated data points
        """
        dataset = options['dataset']
        metric = options['metric']
        aggregation = options['aggregation']
        interval = options['interval']
        
        # Define interval in seconds
        if interval == 'hour':
            interval_seconds = 3600
        elif interval == 'day':
            interval_seconds = 86400
        elif interval == 'week':
            interval_seconds = 604800
        elif interval == 'month':
            interval_seconds = 2592000
        
        # Define time range
        end_time = options.get('end', int(time.time()))
        start_time = options.get('start', end_time - (interval_seconds * 24))  # Default to 24 intervals
        
        # Get all records in the time range
        records = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [
                ['dataset', '=', dataset],
                ['timestamp', '>=', start_time],
                ['timestamp', '<=', end_time]
            ],
            {
                'prefix': self._config.datastore_prefix,
                'order_by': ['timestamp']
            }
        )
        
        if not records:
            return []
        
        # Group records by interval
        intervals = {}
        for record in records:
            interval_start = int(record['timestamp'] / interval_seconds) * interval_seconds
            if interval_start not in intervals:
                intervals[interval_start] = []
            intervals[interval_start].append(record[metric])
        
        # Calculate aggregation for each interval
        result = []
        for interval_start, values in sorted(intervals.items()):
            if aggregation == 'avg':
                value = sum(values) / len(values) if values else 0
            elif aggregation == 'max':
                value = max(values) if values else 0
            elif aggregation == 'min':
                value = min(values) if values else 0
            elif aggregation == 'sum':
                value = sum(values) if values else 0
            
            result.append({
                'timestamp': interval_start,
                'iso_time': time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(interval_start)),
                'value': value,
                'samples': len(values)
            })
        
        return result
    
    @private
    async def _dataset_exists(self, dataset):
        """
        Check if a dataset exists.
        
        Args:
            dataset (str): ZFS dataset name
            
        Returns:
            bool: True if dataset exists
        """
        datasets = await self.middleware.call('zfs.dataset.query', [['id', '=', dataset]])
        return len(datasets) > 0
    
    @private
    async def _collect_pool_iostat(self, pool):
        """
        Collect zpool iostat data for a pool.
        
        Args:
            pool (str): ZFS pool name
            
        Returns:
            str: Raw iostat output or None on error
        """
        try:
            cmd = ['zpool', 'iostat', '-v', pool, '1', '1']
            result = await run(cmd, check=False)
            
            if result.returncode != 0:
                logger.error(f"Error running zpool iostat for {pool}: {result.stderr}")
                return None
            
            return result.stdout.decode()
        except Exception as e:
            logger.error(f"Failed to collect iostat for {pool}: {str(e)}")
            return None
    
    @private
    async def _parse_dataset_metrics(self, iostat_output, dataset):
        """
        Parse iostat output to extract metrics for a specific dataset.
        
        Args:
            iostat_output (str): Raw iostat output
            dataset (str): ZFS dataset name
            
        Returns:
            dict: Dataset metrics
        """
        if not iostat_output:
            return {}
        
        lines = iostat_output.strip().split('\n')
        
        # Find the line containing this dataset
        for line in lines:
            if dataset in line:
                # Extract metrics from this line
                fields = re.split(r'\s+', line.strip())
                if len(fields) >= 7:
                    try:
                        read_ops = int(float(fields[1]))
                        write_ops = int(float(fields[2]))
                        read_bytes = int(float(fields[3]) * 1024)  # Convert K to bytes
                        write_bytes = int(float(fields[4]) * 1024)  # Convert K to bytes
                        
                        # Calculate bytes per operation (to determine sequential vs random)
                        bytes_per_read = read_bytes / max(read_ops, 1)
                        bytes_per_write = write_bytes / max(write_ops, 1)
                        
                        # Determine access pattern
                        if read_ops > 100 and bytes_per_read > 64 * 1024:
                            access_pattern = 'sequential_read'
                        elif write_ops > 100 and bytes_per_write > 64 * 1024:
                            access_pattern = 'sequential_write'
                        elif read_ops > 100:
                            access_pattern = 'random_read'
                        elif write_ops > 100:
                            access_pattern = 'random_write'
                        else:
                            access_pattern = 'mixed'
                            
                        return {
                            'read_ops': read_ops,
                            'write_ops': write_ops,
                            'read_bytes': read_bytes,
                            'write_bytes': write_bytes,
                            'bytes_per_read': bytes_per_read,
                            'bytes_per_write': bytes_per_write,
                            'access_pattern': access_pattern
                        }
                    except (ValueError, IndexError, ZeroDivisionError) as e:
                        logger.error(f"Failed to parse iostat line for {dataset}: {str(e)}")
        
        return {}


def setup(middleware):
    # Create a job schedule for telemetry cleanup
    middleware.logger.info("Loading ZFS Telemetry Collector plugin")
    
    # Schedule daily cleanup of old telemetry data
    middleware.call_sync(
        'cronjob.add', 
        {
            'name': 'telemetry_cleanup',
            'command': 'sqlite',
            'description': 'Clean up old telemetry data',
            'enabled': True,
            'hour': '3',
            'minute': '0',
            'schedule': {
                'minute': '0',
                'hour': '3',
                'dom': '*',
                'month': '*',
                'dow': '*'
            },
            'user': 'root',
            'state': 'RUNNING'
        }
    ) 