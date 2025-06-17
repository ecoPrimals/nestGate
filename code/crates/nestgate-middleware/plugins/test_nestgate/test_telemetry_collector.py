#!/usr/bin/env python3

import os
import json
import time
import pytest
import asyncio
from unittest.mock import Mock, patch, AsyncMock, MagicMock, call

# Import the module to test
from middlewared.plugins.nestgate.telemetry_collector import TelemetryCollectorService


class TestTelemetryCollector:
    """
    Unit tests for the Telemetry Collector middleware plugin.
    """

    @pytest.fixture
    def telemetry_collector(self):
        """Create a Telemetry Collector instance with mocked middleware."""
        middleware = AsyncMock()
        middleware.call.side_effect = self._mock_middleware_call
        middleware.call_sync.side_effect = self._mock_middleware_call_sync
        
        collector = TelemetryCollectorService(middleware)
        collector._config = Mock()
        collector._config.datastore = 'nestgate.telemetry'
        collector._config.datastore_prefix = 'tel_'
        
        # Mock the private methods
        collector._dataset_exists = AsyncMock(return_value=True)
        collector._collect_pool_iostat = AsyncMock(
            return_value=b'test_dataset  1000  500  1024K  512K  10ms  5ms  seq  rand'
        )
        collector._parse_dataset_metrics = AsyncMock(
            return_value={
                'read_ops': 1000,
                'write_ops': 500,
                'read_bytes': 1048576,
                'write_bytes': 524288,
                'read_latency': 10,
                'write_latency': 5,
                'read_access': 'sequential',
                'write_access': 'random'
            }
        )
        
        return collector
    
    def _mock_middleware_call(self, service, *args, **kwargs):
        """Mock for middleware.call method."""
        if service == 'zfs.dataset.query':
            # Return a mock dataset
            if args[0][0][1] == 'test_dataset':
                return [{'id': 'test_dataset', 'properties': {}}]
            elif args[0][0][1] == 'test_pool/test_dataset':
                return [{'id': 'test_pool/test_dataset', 'properties': {}}]
            return []
        elif service == 'datastore.query':
            # Return mock AI workload records
            if args[0] == 'nestgate.aiworkload':
                return [{
                    'id': 1,
                    'dataset': 'test_pool/test_dataset',
                    'enabled': True,
                    'sampling_period': 3600,
                    'min_samples': 5,
                    'auto_tune': True,
                    'last_detected': time.time(),
                    'workload_type': 'training',
                    'created': time.time(),
                    'updated': time.time()
                }]
            elif args[0] == 'nestgate.telemetry':
                # Return mock telemetry records based on dataset filter
                if len(args[1]) > 0 and args[1][0][1] == 'test_dataset':
                    return [self._create_mock_telemetry_record('test_dataset', i) for i in range(10)]
                elif len(args[1]) > 1 and args[1][0][1] == 'test_dataset':
                    # For queries with dataset and timestamp filter
                    return [self._create_mock_telemetry_record('test_dataset', i) for i in range(10)]
                # Default telemetry records
                return [self._create_mock_telemetry_record('test_pool/test_dataset', i) for i in range(5)]
            return []
        elif service == 'datastore.insert':
            # Return a mock ID
            return 1
        
        return AsyncMock()
        
    def _create_mock_telemetry_record(self, dataset, index):
        """Helper to create mock telemetry records."""
        current_time = time.time()
        return {
            'id': index + 1,
            'tel_dataset': dataset,
            'tel_timestamp': current_time - (index * 3600),  # Each record is 1 hour apart
            'tel_read_ops': 1000 + (index * 100),
            'tel_write_ops': 500 + (index * 50),
            'tel_read_bytes': 1048576 + (index * 1024),
            'tel_write_bytes': 524288 + (index * 512),
            'tel_read_latency': 10 + index,
            'tel_write_latency': 5 + index,
            'tel_read_access': 'sequential',
            'tel_write_access': 'random'
        }

    def _mock_middleware_call_sync(self, service, *args, **kwargs):
        """Mock for middleware.call_sync method."""
        if service == 'zfs.dataset.query':
            # Return a mock dataset
            if len(args) > 0 and args[0][0][1] == 'test_dataset':
                return [{'id': 'test_dataset', 'properties': {}}]
            return []
        elif service == 'datastore.insert':
            # Return a mock ID
            return 1
        elif service == 'datastore.delete_many':
            # Mock deleting records
            return 5  # Number of records deleted
        return Mock()
    
    @pytest.mark.asyncio
    async def test_collect_telemetry(self, telemetry_collector):
        """Test the collect_telemetry method."""
        # Call the method
        await telemetry_collector.collect_telemetry()
        
        # Verify calls to middleware
        telemetry_collector.middleware.call.assert_any_call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        # Verify dataset_exists called
        telemetry_collector._dataset_exists.assert_called_once()
        
        # Verify collect_pool_iostat called
        telemetry_collector._collect_pool_iostat.assert_called_once()
        
        # Verify parse_dataset_metrics called
        telemetry_collector._parse_dataset_metrics.assert_called_once()
        
        # For the datastore.insert call with time values, we need to verify differently
        call_args_list = telemetry_collector.middleware.call.call_args_list
        insert_calls = [call for call in call_args_list if call[0][0] == 'datastore.insert' and call[0][1] == 'nestgate.telemetry']
        
        # Verify we have at least one insert call
        assert len(insert_calls) > 0
        
        # Verify the insert call parameters
        insert_call = insert_calls[0]
        assert insert_call[0][1] == 'nestgate.telemetry'
        assert insert_call[0][2]['dataset'] == 'test_pool/test_dataset'
        assert 'timestamp' in insert_call[0][2]
        assert insert_call[0][2]['read_ops'] == 1000
        assert insert_call[0][2]['write_ops'] == 500
        assert insert_call[0][2]['read_bytes'] == 1048576
        assert insert_call[0][2]['write_bytes'] == 524288
        assert insert_call[0][2]['read_latency'] == 10
        assert insert_call[0][2]['write_latency'] == 5
        assert insert_call[0][2]['read_access'] == 'sequential'
        assert insert_call[0][2]['write_access'] == 'random'
        assert insert_call[0][3] == {'prefix': 'tel_'}
    
    @pytest.mark.asyncio
    async def test_cleanup_telemetry(self, telemetry_collector):
        """Test the cleanup_telemetry method."""
        # Set up test options
        options = {
            'max_age': 7,  # 7 days
            'max_records': 100
        }
        
        # Call the method
        result = await telemetry_collector.cleanup_telemetry(options)
        
        # Verify calls to middleware for max_age deletion
        max_time = int(time.time()) - (options['max_age'] * 86400)
        telemetry_collector.middleware.call_sync.assert_any_call(
            'datastore.delete_many',
            'nestgate.telemetry',
            [['timestamp', '<', max_time]],
            {'prefix': 'tel_'}
        )
        
        # Verify result has deleted count
        assert 'deleted' in result
        assert isinstance(result['deleted'], int)
    
    @pytest.mark.asyncio
    async def test_get_history(self, telemetry_collector):
        """Test the get_history method."""
        # Set up test parameters
        dataset = 'test_dataset'
        timespan = 24  # 24 hours
        limit = 100
        
        # Call the method
        result = await telemetry_collector.get_history(dataset, timespan, limit)
        
        # Verify calls to middleware
        expected_time = int(time.time()) - (timespan * 3600)  # hours to seconds
        telemetry_collector.middleware.call.assert_any_call(
            'datastore.query',
            'nestgate.telemetry',
            [
                ['dataset', '=', dataset],
                ['timestamp', '>=', expected_time]
            ],
            {'prefix': 'tel_', 'limit': limit, 'order_by': ['-timestamp']}
        )
        
        # Verify result structure - we expect records based on our mock
        assert len(result) > 0  # we should have some records
        
        # Verify record structure
        record = result[0]
        assert 'dataset' in record
        assert 'timestamp' in record
        assert 'read_ops' in record
        assert 'write_ops' in record
        assert 'read_bytes' in record
        assert 'write_bytes' in record
        assert 'read_latency' in record
        assert 'write_latency' in record
        assert 'read_access' in record
        assert 'write_access' in record
    
    @pytest.mark.asyncio
    async def test_aggregate(self, telemetry_collector):
        """Test the aggregate method."""
        # Set up test parameters
        dataset = 'test_dataset'
        interval = 'hourly'  # Options: hourly, daily, weekly
        metric = 'read_ops'
        agg_func = 'avg'  # Options: avg, max, min, sum
        timespan = 24  # 24 hours
    
        # Call the method
        result = await telemetry_collector.aggregate(dataset, interval, metric, agg_func, timespan)
    
        # Verify calls to middleware
        expected_time = int(time.time()) - (timespan * 3600)  # hours to seconds
        telemetry_collector.middleware.call.assert_any_call(
            'datastore.query',
            'nestgate.telemetry',
            [
                ['dataset', '=', dataset],
                ['timestamp', '>=', expected_time]
            ],
            {'prefix': 'tel_'}
        )
    
        # Verify result structure
        assert isinstance(result, list)
        assert len(result) > 0
        
        # Verify each interval result
        for interval_result in result:
            assert isinstance(interval_result, dict)
            assert 'start_time' in interval_result
            assert 'end_time' in interval_result
            assert 'value' in interval_result
            assert 'samples' in interval_result
            assert isinstance(interval_result['start_time'], (int, float))
            assert isinstance(interval_result['end_time'], (int, float))
            assert isinstance(interval_result['value'], (int, float))
            assert isinstance(interval_result['samples'], int)
            assert interval_result['end_time'] > interval_result['start_time']
            assert interval_result['samples'] > 0


if __name__ == '__main__':
    pytest.main(['-xvs', __file__]) 