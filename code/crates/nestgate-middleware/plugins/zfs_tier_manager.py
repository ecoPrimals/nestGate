#!/usr/bin/env python3

import os
import json
import time
import logging
from middlewared.schema import Dict, Str, Int, Bool, List, accepts
from middlewared.service import CRUDService, private, job
from middlewared.utils import run

logger = logging.getLogger('zfs_tier_manager')

class ZFSTierManagerService(CRUDService):
    class Config:
        namespace = 'nestgate.tiering'
        datastore = 'nestgate.tiering'
        datastore_prefix = 'tier_'
        private = True
        verbose_name = 'ZFS Tier Manager'
        cli_namespace = 'storage.nestgate.tiering'

    async def do_create(self, data):
        """
        Create a new tier configuration for a dataset.
        
        Args:
            data (dict): 
                dataset (str): ZFS dataset name
                tier (str): Tier type ('hot', 'warm', 'cold', 'cache')
                apply_defaults (bool): Whether to apply default tier properties
        
        Returns:
            dict: Created tier configuration
        """
        dataset = data.get('dataset')
        tier = data.get('tier')
        apply_defaults = data.get('apply_defaults', True)
        
        # Verify dataset exists
        datasets = await self.middleware.call('zfs.dataset.query', [['id', '=', dataset]])
        if not datasets:
            raise ValueError(f"Dataset {dataset} not found")
        
        # Get default properties for tier
        tier_props = await self._get_tier_properties(tier)
        
        # Apply properties to dataset if requested
        if apply_defaults:
            for prop, value in tier_props.items():
                try:
                    await self.middleware.call(
                        'zfs.dataset.update', 
                        dataset, 
                        {prop: value}
                    )
                    logger.debug(f"Applied property {prop}={value} to {dataset}")
                except Exception as e:
                    logger.error(f"Failed to set {prop}={value} on {dataset}: {str(e)}")
                    raise
        
        # Add custom property for tier identification
        try:
            # First check if the property exists
            cmd = ['zfs', 'get', 'nestgate:storage_tier', dataset]
            result = await run(cmd, check=False)
            
            # Set the property
            cmd = ['zfs', 'set', f'nestgate:storage_tier={tier}', dataset]
            await run(cmd, check=True)
            logger.debug(f"Set nestgate:storage_tier={tier} on {dataset}")
        except Exception as e:
            logger.error(f"Failed to set tier property on {dataset}: {str(e)}")
            raise
        
        # Store configuration in database
        data_to_store = {
            'dataset': dataset,
            'tier': tier,
            'properties': json.dumps(tier_props),
            'created': time.time(),
            'updated': time.time()
        }
        
        try:
            pk = await self.middleware.call(
                'datastore.insert',
                self._config.datastore,
                data_to_store,
                {'prefix': self._config.datastore_prefix}
            )
            logger.info(f"Created tier configuration for {dataset} as {tier}")
            return await self._get_instance(pk)
        except Exception as e:
            logger.error(f"Database error creating tier record: {str(e)}")
            raise
    
    @accepts(Int('id'), Dict(
        'tier_update',
        Str('tier', enum=['hot', 'warm', 'cold', 'cache']),
        Bool('apply_defaults'),
        register=True
    ))
    async def do_update(self, id, data):
        """
        Update an existing tier configuration.
        
        Args:
            id (int): Record ID
            data (dict): Updated tier data
        
        Returns:
            dict: Updated tier configuration
        """
        old = await self._get_instance(id)
        new = old.copy()
        new.update(data)
        
        # If tier has changed
        if 'tier' in data and old['tier'] != new['tier']:
            # Get new tier properties
            tier_props = await self._get_tier_properties(new['tier'])
            new['properties'] = json.dumps(tier_props)
            
            # Apply new properties if requested
            if new.get('apply_defaults', False):
                for prop, value in tier_props.items():
                    try:
                        await self.middleware.call(
                            'zfs.dataset.update', 
                            old['dataset'], 
                            {prop: value}
                        )
                    except Exception as e:
                        logger.error(f"Failed to update {prop} on {old['dataset']}: {str(e)}")
            
            # Update tier property
            try:
                cmd = ['zfs', 'set', f'nestgate:storage_tier={new["tier"]}', old['dataset']]
                await run(cmd, check=True)
            except Exception as e:
                logger.error(f"Failed to update tier property: {str(e)}")
                raise
        
        new['updated'] = time.time()
        
        await self.middleware.call(
            'datastore.update',
            self._config.datastore,
            id,
            new,
            {'prefix': self._config.datastore_prefix}
        )
        
        return await self._get_instance(id)
    
    @accepts(Int('id'))
    async def do_delete(self, id):
        """
        Delete a tier configuration.
        Note: This does not remove properties from the dataset.
        
        Args:
            id (int): Record ID
        
        Returns:
            bool: True on success
        """
        record = await self._get_instance(id)
        
        # Attempt to remove the custom property
        try:
            cmd = ['zfs', 'inherit', 'nestgate:storage_tier', record['dataset']]
            await run(cmd, check=False)
        except Exception as e:
            logger.warning(f"Failed to remove tier property from {record['dataset']}: {str(e)}")
        
        await self.middleware.call(
            'datastore.delete',
            self._config.datastore,
            id
        )
        
        return True
    
    @accepts(Dict(
        'apply_workload_tuning',
        Str('dataset', required=True),
        Str('workload', required=True, enum=['training', 'inference', 'checkpointing']),
        register=True
    ))
    @job(lock='apply_workload_tuning_{dataset}')
    async def apply_workload_tuning(self, job, data):
        """
        Apply AI workload-specific tuning to a dataset.
        
        Args:
            data (dict):
                dataset (str): ZFS dataset name
                workload (str): AI workload type
        
        Returns:
            bool: True on success
        """
        dataset = data['dataset']
        workload = data['workload']
        
        # Get current tier
        tier_info = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [['dataset', '=', dataset]],
            {'prefix': self._config.datastore_prefix, 'get': True}
        )
        
        if not tier_info:
            raise ValueError(f"Dataset {dataset} is not configured for tiering")
        
        tier = tier_info['tier']
        
        # Get workload-specific optimizations for this tier
        optimizations = await self._get_workload_optimizations(tier, workload)
        
        # Apply optimizations
        job.set_progress(0, f"Applying {workload} optimizations to {dataset}")
        
        total_props = len(optimizations)
        for i, (prop, value) in enumerate(optimizations.items()):
            progress = int((i / total_props) * 100)
            job.set_progress(progress, f"Setting {prop}={value}")
            
            try:
                await self.middleware.call(
                    'zfs.dataset.update', 
                    dataset, 
                    {prop: value}
                )
            except Exception as e:
                logger.error(f"Failed to set {prop}={value} on {dataset}: {str(e)}")
                # Continue with other properties
        
        # Update database record
        await self.middleware.call(
            'datastore.update',
            self._config.datastore,
            tier_info['id'],
            {
                'updated': time.time(),
                'workload_type': workload
            },
            {'prefix': self._config.datastore_prefix}
        )
        
        job.set_progress(100, f"Successfully applied {workload} optimizations to {dataset}")
        return True
    
    @accepts(Str('pool_name'))
    async def get_tier_datasets(self, pool_name):
        """
        Get all tier-configured datasets for a specific pool.
        
        Args:
            pool_name (str): ZFS pool name
            
        Returns:
            list: List of tier datasets
        """
        # Query all tier records from database
        tier_records = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [],
            {'prefix': self._config.datastore_prefix}
        )
        
        # Filter records to only include datasets in the specified pool
        pool_datasets = []
        for record in tier_records:
            dataset = record['dataset']
            if dataset.split('/')[0] == pool_name:
                # Add dataset properties
                try:
                    zfs_dataset = await self.middleware.call(
                        'zfs.dataset.query',
                        [['id', '=', dataset]],
                        {'get': True}
                    )
                    record['properties'] = zfs_dataset['properties']
                except Exception:
                    record['properties'] = {}
                
                pool_datasets.append(record)
        
        return pool_datasets
    
    @accepts(Str('dataset'))
    async def get_io_stats(self, dataset):
        """
        Get IO statistics for a dataset.
        
        Args:
            dataset (str): ZFS dataset name
            
        Returns:
            dict: IO statistics
        """
        pool = dataset.split('/')[0]
        
        # Run zpool iostat to get IO statistics
        cmd = ['zpool', 'iostat', '-v', pool, '1', '1']
        iostat = await run(cmd, check=False)
        
        if iostat.returncode != 0:
            logger.error(f"Error collecting IO stats for {dataset}: {iostat.stderr}")
            return {}
        
        # Parse output to get stats for this dataset
        return await self._parse_iostat(iostat.stdout.decode(), dataset)
    
    @private
    async def _parse_iostat(self, iostat_output, dataset):
        """
        Parse zpool iostat output to extract metrics for a dataset.
        
        Args:
            iostat_output (str): Output from zpool iostat command
            dataset (str): ZFS dataset name
            
        Returns:
            dict: Parsed IO metrics
        """
        import re
        
        lines = iostat_output.strip().split('\n')
        for i, line in enumerate(lines):
            if dataset in line:
                # Extract metrics from this line
                fields = re.split(r'\s+', line.strip())
                if len(fields) >= 7:
                    try:
                        return {
                            'read_ops': int(float(fields[1])),
                            'write_ops': int(float(fields[2])),
                            'read_bytes': int(float(fields[3]) * 1024),  # Convert K to bytes
                            'write_bytes': int(float(fields[4]) * 1024),  # Convert K to bytes
                        }
                    except (ValueError, IndexError):
                        logger.error(f"Failed to parse iostat output for {dataset}")
        
        return {}
    
    @private
    async def _get_tier_properties(self, tier):
        """
        Get default properties for a specific tier.
        
        Args:
            tier (str): Tier type ('hot', 'warm', 'cold', 'cache')
            
        Returns:
            dict: ZFS properties for the tier
        """
        if tier == 'hot':
            return {
                'recordsize': '128K',
                'compression': 'lz4',
                'primarycache': 'all',
                'secondarycache': 'all',
                'sync': 'standard',
                'logbias': 'throughput',
                'atime': 'off'
            }
        elif tier == 'warm':
            return {
                'recordsize': '1M',
                'compression': 'zstd',
                'primarycache': 'metadata',
                'secondarycache': 'all',
                'sync': 'standard',
                'logbias': 'latency',
                'atime': 'off'
            }
        elif tier == 'cold':
            return {
                'recordsize': '1M',
                'compression': 'zstd-19',
                'primarycache': 'metadata',
                'secondarycache': 'metadata',
                'sync': 'standard',
                'logbias': 'throughput',
                'atime': 'off'
            }
        elif tier == 'cache':
            return {
                'recordsize': '128K',
                'compression': 'lz4',
                'primarycache': 'all',
                'secondarycache': 'all',
                'sync': 'always',
                'logbias': 'latency',
                'atime': 'on'
            }
        else:
            raise ValueError(f"Unknown tier: {tier}")
    
    @private
    async def _get_workload_optimizations(self, tier, workload):
        """
        Get workload-specific optimizations for a tier.
        
        Args:
            tier (str): Tier type
            workload (str): Workload type
            
        Returns:
            dict: Optimized properties for the workload
        """
        # Base properties from tier
        base_props = await self._get_tier_properties(tier)
        
        # AI workload specific optimizations
        if workload == 'training':
            # Training workloads: high write throughput, sequential reads
            if tier == 'hot':
                return {
                    **base_props,
                    'recordsize': '1M',           # Larger record size for sequential access
                    'primarycache': 'all',        # Cache everything
                    'secondarycache': 'all',      # Use L2ARC if available
                    'logbias': 'throughput',      # Optimize for throughput
                    'prefetch': '1'               # Enable prefetch for sequential reads
                }
            elif tier == 'warm':
                return {
                    **base_props,
                    'compression': 'zstd-3',      # Better compression ratio
                    'logbias': 'throughput'       # Optimize for throughput
                }
            else:
                return base_props
                
        elif workload == 'inference':
            # Inference workloads: high read IOPS, random access patterns
            if tier == 'hot':
                return {
                    **base_props,
                    'recordsize': '16K',          # Smaller record size for random access
                    'primarycache': 'all',        # Cache everything
                    'secondarycache': 'all',      # Use L2ARC if available
                    'logbias': 'latency',         # Optimize for latency
                    'prefetch': '0'               # Disable prefetch for random access
                }
            elif tier == 'warm':
                return {
                    **base_props,
                    'recordsize': '128K',         # Smaller record size
                    'primarycache': 'all',        # Cache everything
                    'prefetch': '0'               # Disable prefetch
                }
            else:
                return base_props
                
        elif workload == 'checkpointing':
            # Checkpointing workloads: burst writes, data integrity
            if tier == 'hot':
                return {
                    **base_props,
                    'sync': 'always',             # Ensure data integrity
                    'logbias': 'latency',         # Optimize for latency
                    'redundant_metadata': 'most'  # Ensure metadata integrity
                }
            else:
                return {
                    **base_props,
                    'sync': 'always',             # Ensure data integrity
                    'redundant_metadata': 'most'  # Ensure metadata integrity
                }
        
        # Default: return base properties for tier
        return base_props


def setup(middleware):
    middleware.logger.info("Loading ZFS Tier Manager plugin") 