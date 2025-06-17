#!/usr/bin/env python3

import asyncio
import json
import time
from unittest.mock import AsyncMock, Mock
from middlewared.plugins.nestgate.utils import run

class AIWorkloadDetectorService:
    """Mock AI Workload Detector Service for testing."""
    
    def __init__(self, middleware):
        self.middleware = middleware
        self._config = None
    
    async def do_create(self, data):
        """Mock method to create a workload detection configuration."""
        # Check if dataset exists
        await self.middleware.call('zfs.dataset.query', [['id', '=', data['dataset']]])
        
        # Insert the configuration into the datastore
        id = await self.middleware.call(
            'datastore.insert',
            'nestgate.aiworkload',
            {
                'dataset': data['dataset'],
                'enabled': data.get('enabled', True),
                'sampling_period': data.get('sampling_period', 3600),
                'min_samples': data.get('min_samples', 5),
                'auto_tune': data.get('auto_tune', True),
                'last_detected': 0,
                'workload_type': '',
                'created': time.time(),
                'updated': time.time()
            },
            {'prefix': 'ai_'}
        )
        
        return {"id": id}
    
    async def get_enabled_datasets(self, pool=None):
        """Mock method to get enabled datasets."""
        # Query the datastore for enabled configurations
        datasets = await self.middleware.call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        # Filter by pool if provided
        if pool:
            datasets = [d for d in datasets if d['dataset'].startswith(f"{pool}/")]
            
        return datasets
    
    async def detect_workload(self, job, data):
        """Mock method to detect workload."""
        job.set_progress(0, f"Starting AI workload detection for {data['dataset']}")
        
        # For testing, assume this is a new dataset or use the first config if multiple exist
        config_id = 1  # Default ID for testing
        
        # Get IO statistics
        job.set_progress(50, f"Analyzing IO patterns for {data['dataset']}")
        stats = await self.middleware.call('nestgate.tiering.get_io_stats', data['dataset'])
        
        # Detect pattern
        workload_type, confidence = await self._detect_pattern(stats)
        
        # Update the configuration with the detected workload
        await self.middleware.call(
            'datastore.update',
            'nestgate.aiworkload',
            config_id,
            {
                'workload_type': workload_type,
                'last_detected': time.time(),
                'updated': time.time()
            },
            {'prefix': 'ai_'}
        )
        
        # Make sure confidence is displayed consistently
        job.set_progress(100, f"Detected {workload_type} workload with {confidence:.1f}% confidence")
        
        return {
            'workload_type': workload_type,
            'confidence': confidence
        }
    
    async def run_detection(self):
        """Mock method to run detection process."""
        # Get all enabled datasets
        datasets = await self.middleware.call(
            'datastore.query',
            'nestgate.aiworkload',
            [['enabled', '=', True]],
            {'prefix': 'ai_'}
        )
        
        # Process each dataset
        for dataset in datasets:
            # Skip datasets that don't have enough samples
            if dataset['dataset'] == '':
                continue
                
            # Create a simple mock job
            mock_job = Mock()
            mock_job.set_progress = Mock()
                
            # Detect workload
            result = await self.detect_workload(mock_job, {
                'dataset': dataset['dataset']
            })
            
            # Auto-tune if enabled
            if dataset['auto_tune'] and result['workload_type'] != '':
                await self.tune_dataset(mock_job, {
                    'dataset': dataset['dataset'],
                    'workload_type': result['workload_type']
                })
    
    async def do_update(self, id, data):
        """Mock method to update workload detection settings."""
        # For testing, create a default config
        default_config = {
            'id': id,
            'dataset': 'test_dataset',
            'enabled': True,
            'sampling_period': 3600,
            'min_samples': 5,
            'auto_tune': False,
            'last_detected': 0,
            'workload_type': ''
        }
        
        # Update the configuration
        await self.middleware.call(
            'datastore.update',
            'nestgate.aiworkload',
            id,
            {
                'enabled': data.get('enabled', default_config['enabled']),
                'sampling_period': data.get('sampling_period', default_config['sampling_period']),
                'min_samples': data.get('min_samples', default_config['min_samples']),
                'auto_tune': data.get('auto_tune', default_config['auto_tune']),
                'updated': time.time()
            },
            {'prefix': 'ai_'}
        )
        
        # Return the updated configuration
        return {
            'id': id,
            'dataset': default_config['dataset'],
            'enabled': data.get('enabled', default_config['enabled']),
            'sampling_period': data.get('sampling_period', default_config['sampling_period']),
            'min_samples': data.get('min_samples', default_config['min_samples']),
            'auto_tune': data.get('auto_tune', default_config['auto_tune']),
            'last_detected': default_config['last_detected'],
            'workload_type': default_config['workload_type']
        }
    
    async def tune_dataset(self, job, data):
        """Mock method to tune dataset for workload."""
        job.set_progress(0, f"Tuning {data['dataset']} for {data['workload_type']} workload")
        
        # Apply workload tuning
        await self.middleware.call(
            'nestgate.tiering.apply_workload_tuning',
            {
                'dataset': data['dataset'],
                'workload': data['workload_type']
            }
        )
        
        # Apply additional recommendations
        await self._apply_recommendations(data['dataset'], data['workload_type'])
        
        job.set_progress(100, f"Successfully tuned {data['dataset']} for {data['workload_type']} workload")
        
        return True
    
    async def _detect_pattern(self, stats):
        """Mock private method to detect patterns."""
        # For testing, return a predetermined result
        return ("training", 0.85)
    
    async def _apply_recommendations(self, dataset, workload):
        """Mock private method to apply recommendations."""
        # For testing, return success
        return True 