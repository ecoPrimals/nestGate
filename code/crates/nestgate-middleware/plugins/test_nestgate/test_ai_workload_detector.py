#!/usr/bin/env python3

import os
import json
import time
import pytest
import asyncio
from unittest.mock import Mock, patch, AsyncMock, MagicMock

# Import the module to test
from middlewared.plugins.nestgate.ai_workload_detector import AIWorkloadDetectorService


class TestAIWorkloadDetector:
    """
    Unit tests for the AI Workload Detector middleware plugin.
    """

    @pytest.fixture
    def workload_detector(self):
        """Create an AI Workload Detector instance with mocked middleware."""
        middleware = AsyncMock()
        middleware.call.side_effect = self._mock_middleware_call
        middleware.call_sync.side_effect = self._mock_middleware_call_sync
        
        detector = AIWorkloadDetectorService(middleware)
        detector._config = Mock()
        detector._config.datastore = 'nestgate.aiworkload'
        detector._config.datastore_prefix = 'ai_'
        
        # Mock the private methods
        detector._detect_pattern = AsyncMock(return_value=('training', 0.8))
        detector._apply_recommendations = AsyncMock(return_value=True)
        
        return detector
    
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
            # Return mock AI workload records
            if args[0] == 'nestgate.aiworkload':
                if len(args[1]) > 0 and args[1][0][1] == 'test_dataset':
                    return [{
                        'id': 1,
                        'dataset': 'test_dataset',
                        'enabled': True,
                        'sampling_period': 3600,
                        'min_samples': 5,
                        'auto_tune': True,
                        'last_detected': time.time(),
                        'workload_type': 'training',
                        'created': time.time(),
                        'updated': time.time()
                    }]
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
            return []
        elif service == 'nestgate.tiering.get_io_stats':
            # Return mock IO stats
            return {
                'read_ops': 1000,
                'write_ops': 500,
                'read_bytes': 102400,
                'write_bytes': 51200,
                'read_latency': 10,
                'write_latency': 5
            }
        elif service == 'nestgate.tiering.get_tier_datasets':
            # Return mock tier datasets
            return [{
                'id': 1,
                'dataset': 'test_pool/test_dataset',
                'tier': 'hot',
                'properties': {
                    'recordsize': '128K',
                    'compression': 'lz4',
                    'primarycache': 'all',
                    'secondarycache': 'all',
                    'sync': 'standard',
                    'logbias': 'throughput',
                    'atime': 'off'
                },
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
    
    @pytest.mark.asyncio
    async def test_do_create(self, workload_detector):
        """Test the do_create method."""
        # Test data
        data = {
            'dataset': 'test_dataset',
            'enabled': True,
            'sampling_period': 3600,
            'min_samples': 5,
            'auto_tune': True
        }
        
        # Call the method
        result = await workload_detector.do_create(data)
        
        # Verify calls to middleware
        workload_detector.middleware.call.assert_any_call(
            'zfs.dataset.query', 
            [['id', '=', 'test_dataset']]
        )
        
        # Verify result
        assert result is not None
        assert 'id' in result
    
    @pytest.mark.asyncio
    async def test_get_enabled_datasets(self, workload_detector):
        """Test the get_enabled_datasets method."""
        # Call the method
        result = await workload_detector.get_enabled_datasets('test_pool')
        
        # Verify calls to middleware
        workload_detector.middleware.call.assert_any_call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        # Verify result
        assert len(result) == 1
        assert result[0]['dataset'] == 'test_pool/test_dataset'
        assert result[0]['enabled'] is True
        assert result[0]['workload_type'] == 'training'
    
    @pytest.mark.asyncio
    async def test_detect_workload(self, workload_detector):
        """Test the detect_workload method."""
        # Setup mock job
        job = Mock()
        job.set_progress = Mock()
        
        # Test data
        data = {
            'dataset': 'test_dataset'
        }
        
        # Call the method
        result = await workload_detector.detect_workload(job, data)
        
        # Verify calls to middleware
        workload_detector.middleware.call.assert_any_call(
            'nestgate.tiering.get_io_stats',
            'test_dataset'
        )
        
        # For the datastore.update call, we need to verify it differently since timestamps are variable
        call_args_list = workload_detector.middleware.call.call_args_list
        update_calls = [call for call in call_args_list if call[0][0] == 'datastore.update']
        
        # Verify we have at least one update call
        assert len(update_calls) > 0
        
        # Verify the first update call has the correct parameters except for timestamps
        update_call = update_calls[0]
        assert update_call[0][1] == 'nestgate.aiworkload'
        assert update_call[0][2] == 1  # The ID
        assert update_call[0][3]['workload_type'] == 'training'
        assert 'last_detected' in update_call[0][3]
        assert 'updated' in update_call[0][3]
        assert update_call[0][4] == {'prefix': 'ai_'}
        
        # Verify job progress calls
        job.set_progress.assert_any_call(0, "Starting AI workload detection for test_dataset")
        job.set_progress.assert_any_call(50, "Analyzing IO patterns for test_dataset")
        job.set_progress.assert_any_call(100, "Detected training workload with 0.8% confidence")
        
        # Verify _detect_pattern was called
        workload_detector._detect_pattern.assert_called_once()
        
        # Verify result
        assert result['workload_type'] == 'training'
        assert result['confidence'] == 0.8
    
    @pytest.mark.asyncio
    async def test_run_detection(self, workload_detector):
        """Test the run_detection method."""
        # Mock the detect_workload method 
        workload_detector.detect_workload = AsyncMock()
        workload_detector.detect_workload.return_value = {
            'workload_type': 'training',
            'confidence': 0.8
        }
        
        # Call the method
        await workload_detector.run_detection()
        
        # Verify calls to middleware
        workload_detector.middleware.call.assert_any_call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        # Verify detect_workload was called
        workload_detector.detect_workload.assert_called_once()
    
    @pytest.mark.asyncio
    async def test_update_workload(self, workload_detector):
        """Test the update method with auto_tune enabled."""
        # Mock job
        job = Mock()
        job.set_progress = Mock()
        
        # Test data
        id = 1
        data = {
            'auto_tune': True
        }
        
        # Call the method
        result = await workload_detector.do_update(id, data)
        
        # For the datastore.update call with time values, we need to verify differently
        call_args_list = workload_detector.middleware.call.call_args_list
        update_calls = [call for call in call_args_list if call[0][0] == 'datastore.update']
        
        # Verify we have at least one update call
        assert len(update_calls) > 0
        
        # Verify the update call parameters
        update_call = update_calls[0]
        assert update_call[0][1] == 'nestgate.aiworkload'
        assert update_call[0][2] == id
        assert update_call[0][3]['enabled'] == True
        assert update_call[0][3]['sampling_period'] == 3600
        assert update_call[0][3]['min_samples'] == 5
        assert update_call[0][3]['auto_tune'] == True
        assert 'updated' in update_call[0][3]
        assert update_call[0][4] == {'prefix': 'ai_'}
        
        # Verify result
        assert result['auto_tune'] is True
    
    @pytest.mark.asyncio
    async def test_tune_dataset(self, workload_detector):
        """Test the tune_dataset method."""
        # Setup mock job
        job = Mock()
        job.set_progress = Mock()
        
        # Test data
        data = {
            'dataset': 'test_dataset',
            'workload_type': 'training'
        }
        
        # Mock the middleware.call method specifically for this test
        with patch.object(workload_detector.middleware, 'call', 
                         new=AsyncMock()) as mock_call:
            # Setup return value for datastore.query
            mock_call.side_effect = self._mock_middleware_call
            
            # Call the method
            result = await workload_detector.tune_dataset(job, data)
            
            # Verify calls to middleware for apply_workload_tuning
            mock_call.assert_any_call(
                'nestgate.tiering.apply_workload_tuning',
                {
                    'dataset': 'test_dataset',
                    'workload': 'training'
                }
            )
        
        # Verify _apply_recommendations was called
        workload_detector._apply_recommendations.assert_called_once()
        
        # Verify job progress calls
        job.set_progress.assert_any_call(0, "Tuning test_dataset for training workload")
        job.set_progress.assert_any_call(100, "Successfully tuned test_dataset for training workload")
        
        # Verify result
        assert result is True


if __name__ == '__main__':
    pytest.main(['-xvs', __file__]) 