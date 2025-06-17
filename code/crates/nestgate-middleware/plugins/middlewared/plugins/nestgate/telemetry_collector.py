#!/usr/bin/env python3

import asyncio
import json
import time
import math
from middlewared.plugins.nestgate.utils import run

class TelemetryCollectorService:
    """Mock Telemetry Collector Service for testing."""
    
    def __init__(self, middleware):
        self.middleware = middleware
        self._config = None
    
    async def collect_telemetry(self):
        """Mock method to collect telemetry data."""
        # Get all datasets with AI workload detection enabled
        datasets = await self.middleware.call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        # Collect telemetry for each dataset
        for dataset_config in datasets:
            dataset = dataset_config['dataset']
            
            # Skip if dataset doesn't exist
            if not await self._dataset_exists(dataset):
                continue
                
            # Collect IO statistics
            output = await self._collect_pool_iostat(dataset)
            
            # Parse metrics
            metrics = await self._parse_dataset_metrics(dataset, output)
            
            # Store telemetry data
            await self.middleware.call(
                'datastore.insert',
                'nestgate.telemetry',
                {
                    'dataset': dataset,
                    'timestamp': int(time.time()),
                    'read_ops': metrics['read_ops'],
                    'write_ops': metrics['write_ops'],
                    'read_bytes': metrics['read_bytes'],
                    'write_bytes': metrics['write_bytes'],
                    'read_latency': metrics['read_latency'],
                    'write_latency': metrics['write_latency'],
                    'read_access': metrics['read_access'],
                    'write_access': metrics['write_access']
                },
                {'prefix': 'tel_'}
            )
    
    async def cleanup_telemetry(self, options=None):
        """Mock method to clean up old telemetry data."""
        if options is None:
            options = {"max_age": 30, "max_records": 1000}
            
        # Get all datasets with telemetry data
        datasets = await self.middleware.call(
            'datastore.query',
            'nestgate.telemetry',
            [],
            {'prefix': 'tel_', 'group_by': 'dataset', 'select': 'dataset'}
        )
        
        total_deleted = 0
        
        # Delete old records based on max_age
        if options.get('max_age', 0) > 0:
            max_time = int(time.time()) - (options['max_age'] * 86400)  # Convert days to seconds
            deleted = await self.middleware.call_sync(
                'datastore.delete_many',
                'nestgate.telemetry',
                [['timestamp', '<', max_time]],
                {'prefix': 'tel_'}
            )
            total_deleted += deleted
            
        # Limit number of records per dataset
        if options.get('max_records', 0) > 0:
            for dataset_obj in datasets:
                # Handle both prefixed and unprefixed dataset field
                dataset = dataset_obj.get('dataset') or dataset_obj.get('tel_dataset')
                if dataset:
                    # Count records
                    count_result = await self.middleware.call(
                        'datastore.query',
                        'nestgate.telemetry',
                        [['dataset', '=', dataset]],
                        {'prefix': 'tel_', 'count': True}
                    )
                    
                    # The count might be returned as a number or as a result object
                    count = count_result if isinstance(count_result, int) else len(count_result)
                    
                    # If over the limit, delete the oldest records
                    if count > options['max_records']:
                        # Get IDs of records to delete
                        to_delete = count - options['max_records']
                        oldest_records = await self.middleware.call(
                            'datastore.query',
                            'nestgate.telemetry',
                            [['dataset', '=', dataset]],
                            {'prefix': 'tel_', 'order_by': ['timestamp'], 'limit': to_delete, 'select': 'id'}
                        )
                        
                        # Extract the IDs
                        record_ids = [record['id'] for record in oldest_records]
                        
                        # Delete the records
                        if record_ids:
                            deleted = await self.middleware.call_sync(
                                'datastore.delete_many',
                                'nestgate.telemetry',
                                [['id', 'in', record_ids]],
                                {'prefix': 'tel_'}
                            )
                            total_deleted += deleted
        
        return {"deleted": total_deleted}
    
    async def get_history(self, dataset, timespan=24, limit=100):
        """Mock method to get telemetry history for a dataset."""
        # Calculate the timestamp for the timespan
        timestamp = int(time.time()) - (timespan * 3600)  # hours to seconds
        
        # Query the datastore
        records = await self.middleware.call(
            'datastore.query',
            'nestgate.telemetry',
            [
                ['dataset', '=', dataset],
                ['timestamp', '>=', timestamp]
            ],
            {'prefix': 'tel_', 'limit': limit, 'order_by': ['-timestamp']}
        )
        
        # Process records to remove prefix
        result = []
        for record in records:
            # If records have tel_ prefix, remove it for the response
            processed_record = {}
            for key, value in record.items():
                if key.startswith('tel_'):
                    processed_record[key[4:]] = value  # Remove 'tel_' prefix
                else:
                    processed_record[key] = value
            result.append(processed_record)
            
        return result
    
    async def aggregate(self, dataset, interval="hourly", metric="read_ops", 
                        agg_func="avg", timespan=24):
        """Mock method to aggregate telemetry data."""
        # Get the raw data
        start_time = int(time.time()) - (timespan * 3600)  # Convert hours to seconds
        records = await self.middleware.call(
            'datastore.query',
            'nestgate.telemetry',
            [
                ['dataset', '=', dataset],
                ['timestamp', '>=', start_time]
            ],
            {'prefix': 'tel_'}
        )
        
        # Process records to remove prefix
        processed_records = []
        for record in records:
            # If records have tel_ prefix, remove it for processing
            processed_record = {}
            for key, value in record.items():
                if key.startswith('tel_'):
                    processed_record[key[4:]] = value  # Remove 'tel_' prefix
                else:
                    processed_record[key] = value
            processed_records.append(processed_record)
        
        # Determine the interval in seconds
        interval_seconds = 3600  # Default to hourly
        if interval == "daily":
            interval_seconds = 86400
        elif interval == "weekly":
            interval_seconds = 604800
            
        # Group records by interval
        intervals = {}
        for record in processed_records:
            interval_key = record['timestamp'] // interval_seconds
            if interval_key not in intervals:
                intervals[interval_key] = []
            intervals[interval_key].append(record)
            
        # Aggregate by the specified function
        results = []
        for interval_key, interval_records in sorted(intervals.items()):
            # Calculate interval start and end times
            interval_start = interval_key * interval_seconds
            interval_end = interval_start + interval_seconds
            
            # Extract the metric values
            values = [record.get(metric, 0) for record in interval_records]
            
            # Apply the aggregation function
            agg_value = 0
            if values:
                if agg_func == "avg":
                    agg_value = sum(values) / len(values)
                elif agg_func == "max":
                    agg_value = max(values)
                elif agg_func == "min":
                    agg_value = min(values)
                elif agg_func == "sum":
                    agg_value = sum(values)
                    
            # Add the result
            results.append({
                "start_time": interval_start,
                "end_time": interval_end,
                "value": agg_value,
                "samples": len(values)
            })
            
        return results
    
    async def _dataset_exists(self, dataset):
        """Mock private method to check if dataset exists."""
        # For testing, assume the dataset exists
        return True
    
    async def _collect_pool_iostat(self, dataset):
        """Mock private method to collect pool iostat."""
        # For testing, return a sample output
        return b'test_dataset  1000  500  1024K  512K  10ms  5ms  seq  rand'
    
    async def _parse_dataset_metrics(self, dataset, output):
        """Mock private method to parse dataset metrics."""
        # For testing, return sample metrics
        return {
            'read_ops': 1000,
            'write_ops': 500,
            'read_bytes': 1048576,
            'write_bytes': 524288,
            'read_latency': 10,
            'write_latency': 5,
            'read_access': 'sequential',
            'write_access': 'random'
        } 