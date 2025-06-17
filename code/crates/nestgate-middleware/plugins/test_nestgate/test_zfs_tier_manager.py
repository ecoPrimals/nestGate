#!/usr/bin/env python3

import os
import json
import time
import pytest
import asyncio
from unittest.mock import Mock, patch, AsyncMock, MagicMock

# Import the module to test
from middlewared.plugins.nestgate.zfs_tier_manager import ZFSTierManagerService


class TestZFSTierManager:
    """
    Unit tests for the ZFS Tier Manager middleware plugin.
    """

    @pytest.fixture
    def tier_manager(self):
        """Create a ZFS Tier Manager instance with mocked middleware."""
        middleware = AsyncMock()
        middleware.call.side_effect = self._mock_middleware_call
        middleware.call_sync.side_effect = self._mock_middleware_call_sync
        
        manager = ZFSTierManagerService(middleware)
        manager._config = Mock()
        manager._config.datastore = 'nestgate.tiering'
        manager._config.datastore_prefix = 'tier_'
        
        # Mock the private methods
        manager._get_tier_properties = AsyncMock()
        manager._get_tier_properties.side_effect = self._mock_get_tier_properties
        manager._parse_iostat = AsyncMock(return_value={'read_ops': 1000, 'write_ops': 500})
        
        return manager
    
    def _mock_middleware_call(self, service, *args, **kwargs):
        """Mock for middleware.call method."""
        if service == 'zfs.dataset.query':
            # Return a mock dataset
            if args[0][0][1] == 'test_dataset':
                return [{'id': 'test_dataset', 'properties': {}}]
            return []
        elif service == 'datastore.insert':
            # Return a mock ID
            return 1
        elif service == 'datastore.query':
            # Return mock tier records
            if args[0] == 'nestgate.tiering':
                if len(args[1]) > 0 and args[1][0][1] == 'test_dataset':
                    return [{
                        'id': 1,
                        'dataset': 'test_dataset',
                        'tier': 'hot',
                        'properties': json.dumps(self._hot_tier_props()),
                        'created': time.time(),
                        'updated': time.time()
                    }]
                return [{
                    'id': 1,
                    'dataset': 'test_pool/test_dataset',
                    'tier': 'hot',
                    'properties': json.dumps(self._hot_tier_props()),
                    'created': time.time(),
                    'updated': time.time()
                }]
            return []
        elif service == 'nestgate.tiering.get_tier_datasets':
            # Return mock tier datasets
            return [{
                'id': 1,
                'dataset': 'test_pool/test_dataset',
                'tier': 'hot',
                'properties': self._hot_tier_props(),
                'created': time.time(),
                'updated': time.time()
            }]
        return AsyncMock()

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
        return Mock()
    
    def _mock_get_tier_properties(self, tier):
        """Mock for _get_tier_properties method."""
        if tier == 'hot':
            return self._hot_tier_props()
        elif tier == 'warm':
            return self._warm_tier_props()
        elif tier == 'cold':
            return self._cold_tier_props()
        elif tier == 'cache':
            return self._cache_tier_props()
        return {}
    
    def _hot_tier_props(self):
        """Return hot tier properties."""
        return {
            'recordsize': '128K',
            'compression': 'lz4',
            'primarycache': 'all',
            'secondarycache': 'all',
            'sync': 'standard',
            'logbias': 'throughput',
            'atime': 'off'
        }
    
    def _warm_tier_props(self):
        """Return warm tier properties."""
        return {
            'recordsize': '1M',
            'compression': 'zstd',
            'primarycache': 'metadata',
            'secondarycache': 'all',
            'sync': 'standard',
            'logbias': 'latency',
            'atime': 'off'
        }
    
    def _cold_tier_props(self):
        """Return cold tier properties."""
        return {
            'recordsize': '1M',
            'compression': 'zstd-19',
            'primarycache': 'metadata',
            'secondarycache': 'metadata',
            'sync': 'standard',
            'logbias': 'throughput',
            'atime': 'off'
        }
    
    def _cache_tier_props(self):
        """Return cache tier properties."""
        return {
            'recordsize': '128K',
            'compression': 'lz4',
            'primarycache': 'all',
            'secondarycache': 'all',
            'sync': 'always',
            'logbias': 'latency',
            'atime': 'on'
        }
    
    @pytest.mark.asyncio
    async def test_do_create(self, tier_manager):
        """Test the do_create method."""
        # Mock run function
        with patch('middlewared.plugins.nestgate.zfs_tier_manager.run', new=AsyncMock()) as mock_run:
            mock_run.return_value.returncode = 0
            
            # Test with hot tier
            data = {
                'dataset': 'test_dataset',
                'tier': 'hot',
                'apply_defaults': True
            }
            
            # Mock middleware calls
            tier_manager.middleware.call.side_effect = self._mock_middleware_call
            
            # Call the method
            result = await tier_manager.do_create(data)
            
            # Verify calls to middleware
            tier_manager.middleware.call.assert_any_call('zfs.dataset.query', [['id', '=', 'test_dataset']])
            tier_manager._get_tier_properties.assert_called_with('hot')
            
            # Verify result
            assert result is not None
            assert 'id' in result
    
    @pytest.mark.asyncio
    async def test_get_tier_datasets(self, tier_manager):
        """Test the get_tier_datasets method."""
        # Call the method
        result = await tier_manager.get_tier_datasets('test_pool')
        
        # Verify calls to middleware
        tier_manager.middleware.call.assert_any_call(
            'datastore.query',
            'nestgate.tiering',
            [],
            {'prefix': 'tier_'}
        )
        
        # Verify result
        assert len(result) == 1
        assert result[0]['dataset'] == 'test_pool/test_dataset'
        assert result[0]['tier'] == 'hot'
    
    @pytest.mark.asyncio
    async def test_get_io_stats(self, tier_manager):
        """Test the get_io_stats method."""
        # Mock run function
        with patch('middlewared.plugins.nestgate.zfs_tier_manager.run', new=AsyncMock()) as mock_run:
            mock_run.return_value.returncode = 0
            mock_run.return_value.stdout = b'test_dataset  1000  500  1024K  512K'
            
            # Call the method
            result = await tier_manager.get_io_stats('test_dataset')
            
            # Verify run called with correct arguments
            mock_run.assert_called_with(['zpool', 'iostat', '-v', 'test_dataset', '1', '1'], check=False)
            
            # Verify result
            assert result['read_ops'] == 1000
            assert result['write_ops'] == 500
    
    @pytest.mark.asyncio
    async def test_apply_workload_tuning(self, tier_manager):
        """Test the apply_workload_tuning method."""
        # Setup mock job
        job = Mock()
        job.set_progress = Mock()
        
        # Mock _get_workload_optimizations
        tier_manager._get_workload_optimizations = AsyncMock()
        tier_manager._get_workload_optimizations.return_value = {
            'recordsize': '1M',
            'compression': 'lz4',
            'primarycache': 'all'
        }
        
        # Call the method
        data = {
            'dataset': 'test_dataset',
            'workload': 'training'
        }
        result = await tier_manager.apply_workload_tuning(job, data)
        
        # Verify _get_workload_optimizations was called with the correct arguments
        tier_manager._get_workload_optimizations.assert_called_with('hot', 'training')
        
        # Verify job progress calls
        job.set_progress.assert_any_call(0, "Applying training optimizations to test_dataset")
        job.set_progress.assert_any_call(100, "Successfully applied training optimizations to test_dataset")
        
        # Verify result
        assert result is True


if __name__ == '__main__':
    pytest.main(['-xvs', __file__]) 