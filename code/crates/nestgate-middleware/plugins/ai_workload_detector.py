#!/usr/bin/env python3

import asyncio
import time
import logging
import os
import re
from middlewared.schema import Dict, Str, Int, Bool, accepts
from middlewared.service import Service, job, private, periodic
from middlewared.utils import run

logger = logging.getLogger('ai_workload_detector')

class AIWorkloadDetectorService(Service):
    class Config:
        namespace = 'nestgate.aidetector'
        datastore = 'nestgate.aiworkload'
        datastore_prefix = 'ai_'
        private = True
        verbose_name = 'AI Workload Detector'
        cli_namespace = 'storage.nestgate.aidetector'

    @periodic(3600)
    async def periodic_scan(self):
        """
        Scan datasets for AI workload patterns on a periodic basis.
        """
        # Only scan if there are enabled datasets
        enabled_datasets = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [['enabled', '=', True]],
            {'prefix': self._config.datastore_prefix}
        )
        
        if not enabled_datasets:
            logger.debug("No datasets with AI detection enabled, skipping scan")
            return
        
        # Start the scan job
        await self.middleware.call('nestgate.aidetector.scan', {})

    @job(lock='aidetector_scan')
    async def scan(self, job, options=None):
        """
        Scan datasets for AI workload patterns.
        
        Args:
            options (dict, optional): Scan options
        
        Returns:
            list: Detected workload types by dataset
        """
        options = options or {}
        results = []
        
        # Get all datasets with AI detection enabled
        datasets = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [['enabled', '=', True]],
            {'prefix': self._config.datastore_prefix}
        )
        
        if not datasets:
            job.set_progress(100, "No datasets with AI detection enabled")
            return results
        
        total_datasets = len(datasets)
        job.set_progress(0, f"Scanning {total_datasets} datasets for AI workload patterns")
        
        for idx, config in enumerate(datasets):
            dataset_name = config['dataset']
            progress = int((idx / total_datasets) * 100)
            job.set_progress(progress, f"Analyzing {dataset_name}")
            
            # Verify the dataset still exists
            exists = await self._dataset_exists(dataset_name)
            if not exists:
                logger.warning(f"Dataset {dataset_name} no longer exists, skipping")
                continue
            
            # Check if enough time has passed since last scan
            last_detected = config.get('last_detected')
            if last_detected:
                # Convert from timestamp to time elapsed
                time_since_last = time.time() - last_detected
                min_period = config.get('sampling_period', 3600)
                
                if time_since_last < min_period:
                    logger.debug(f"Skipping {dataset_name}, last scan was {time_since_last}s ago (min period: {min_period}s)")
                    continue
            
            # Collect workload metrics
            metrics = await self._collect_metrics(dataset_name)
            
            if not metrics:
                logger.warning(f"Failed to collect metrics for {dataset_name}")
                continue
            
            # Analyze workload pattern
            workload_type = await self._analyze_workload(metrics, config)
            
            if workload_type:
                logger.info(f"Detected {workload_type} workload on {dataset_name}")
                
                # Store the detection in results
                results.append({
                    'dataset': dataset_name,
                    'workload_type': workload_type,
                    'metrics': metrics
                })
                
                # Apply optimizations if auto-tune is enabled
                if config.get('auto_tune', False):
                    try:
                        await self.middleware.call(
                            'nestgate.tiering.apply_workload_tuning',
                            {
                                'dataset': dataset_name,
                                'workload': workload_type
                            }
                        )
                        logger.info(f"Applied {workload_type} optimizations to {dataset_name}")
                    except Exception as e:
                        logger.error(f"Failed to apply optimizations to {dataset_name}: {str(e)}")
                
                # Update detection record
                await self.middleware.call(
                    'datastore.update',
                    self._config.datastore,
                    config['id'],
                    {
                        'last_detected': time.time(),
                        'workload_type': workload_type
                    },
                    {'prefix': self._config.datastore_prefix}
                )
        
        job.set_progress(100, f"Completed scan, detected {len(results)} AI workloads")
        return results
    
    @accepts(Dict(
        'detect_config',
        Str('dataset', required=True),
        Bool('enabled', required=True),
        Int('sampling_period', default=3600),
        Int('min_samples', default=10),
        Bool('auto_tune', default=False),
        register=True
    ))
    async def configure(self, data):
        """
        Configure AI workload detection for a dataset.
        
        Args:
            data (dict):
                dataset (str): ZFS dataset name
                enabled (bool): Whether detection is enabled
                sampling_period (int): Minimum time between scans in seconds
                min_samples (int): Minimum samples required for detection
                auto_tune (bool): Whether to auto-apply optimizations
        
        Returns:
            dict: Configuration record
        """
        dataset = data['dataset']
        
        # Verify dataset exists
        exists = await self._dataset_exists(dataset)
        if not exists:
            raise ValueError(f"Dataset {dataset} not found")
        
        # Check if config already exists
        existing = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [['dataset', '=', dataset]],
            {'prefix': self._config.datastore_prefix}
        )
        
        if existing:
            # Update existing record
            record_id = existing[0]['id']
            await self.middleware.call(
                'datastore.update',
                self._config.datastore,
                record_id,
                {
                    'enabled': data['enabled'],
                    'sampling_period': data['sampling_period'],
                    'min_samples': data['min_samples'],
                    'auto_tune': data['auto_tune'],
                    'updated': time.time()
                },
                {'prefix': self._config.datastore_prefix}
            )
            
            logger.info(f"Updated AI detection configuration for {dataset}")
            return await self.get_config(dataset)
        else:
            # Create new record
            record_id = await self.middleware.call(
                'datastore.insert',
                self._config.datastore,
                {
                    'dataset': dataset,
                    'enabled': data['enabled'],
                    'sampling_period': data['sampling_period'],
                    'min_samples': data['min_samples'],
                    'auto_tune': data['auto_tune'],
                    'created': time.time(),
                    'updated': time.time(),
                    'last_detected': None,
                    'workload_type': None
                },
                {'prefix': self._config.datastore_prefix}
            )
            
            logger.info(f"Created AI detection configuration for {dataset}")
            return await self.get_config(dataset)
    
    @accepts(Str('dataset'))
    async def get_config(self, dataset):
        """
        Get AI detection configuration for a dataset.
        
        Args:
            dataset (str): ZFS dataset name
            
        Returns:
            dict: Configuration record or None if not configured
        """
        config = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [['dataset', '=', dataset]],
            {'prefix': self._config.datastore_prefix, 'get': True}
        )
        
        return config
    
    @accepts(Str('dataset'))
    async def delete_config(self, dataset):
        """
        Delete AI detection configuration for a dataset.
        
        Args:
            dataset (str): ZFS dataset name
            
        Returns:
            bool: True on success
        """
        record = await self.middleware.call(
            'datastore.query',
            self._config.datastore,
            [['dataset', '=', dataset]],
            {'prefix': self._config.datastore_prefix, 'get': True}
        )
        
        if record:
            await self.middleware.call(
                'datastore.delete',
                self._config.datastore,
                record['id']
            )
            logger.info(f"Deleted AI detection configuration for {dataset}")
            return True
        
        return False
    
    @accepts(Str('dataset'))
    async def manual_detect(self, dataset):
        """
        Manually detect AI workload on a specific dataset.
        
        Args:
            dataset (str): ZFS dataset name
            
        Returns:
            dict: Detection results or None if no workload detected
        """
        # Verify dataset exists
        exists = await self._dataset_exists(dataset)
        if not exists:
            raise ValueError(f"Dataset {dataset} not found")
        
        # Collect metrics
        metrics = await self._collect_metrics(dataset)
        
        if not metrics:
            return {'error': "Failed to collect metrics"}
        
        # Get configuration or use defaults
        config = await self.get_config(dataset)
        if not config:
            config = {
                'min_samples': 10,
                'dataset': dataset
            }
        
        # Analyze workload
        workload_type = await self._analyze_workload(metrics, config)
        
        if workload_type:
            logger.info(f"Manually detected {workload_type} workload on {dataset}")
            
            # Update detection if the dataset is configured
            if config.get('id'):
                await self.middleware.call(
                    'datastore.update',
                    self._config.datastore,
                    config['id'],
                    {
                        'last_detected': time.time(),
                        'workload_type': workload_type
                    },
                    {'prefix': self._config.datastore_prefix}
                )
            
            return {
                'dataset': dataset,
                'workload_type': workload_type,
                'metrics': metrics,
                'timestamp': time.time()
            }
        
        return {
            'dataset': dataset,
            'workload_type': None,
            'metrics': metrics,
            'timestamp': time.time()
        }
    
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
    async def _collect_metrics(self, dataset):
        """
        Collect performance metrics for a dataset.
        
        Args:
            dataset (str): ZFS dataset name
            
        Returns:
            dict: Performance metrics
        """
        metrics = {}
        pool = dataset.split('/')[0]
        
        # 1. Collect IO statistics from zpool
        try:
            cmd = ['zpool', 'iostat', '-v', pool, '1', '1']
            zpool_stats = await run(cmd, check=False)
            
            if zpool_stats.returncode != 0:
                logger.error(f"Error collecting zpool stats: {zpool_stats.stderr}")
            else:
                metrics.update(await self._parse_iostat(zpool_stats.stdout.decode(), dataset))
        except Exception as e:
            logger.error(f"Failed to collect IO stats: {str(e)}")
        
        # 2. Collect file access patterns using lsof
        try:
            # ZFS datasets are mounted at /mnt/pool/dataset in TrueNAS
            dataset_path = os.path.join('/mnt', dataset.replace('/', '/'))
            
            if not os.path.exists(dataset_path):
                logger.warning(f"Dataset path {dataset_path} does not exist")
            else:
                cmd = ['lsof', dataset_path]
                lsof_result = await run(cmd, check=False)
                
                if lsof_result.returncode == 0:
                    metrics['access_patterns'] = await self._parse_access_patterns(
                        lsof_result.stdout.decode(),
                        dataset_path
                    )
                else:
                    # lsof returns 1 if no files are open
                    metrics['access_patterns'] = {'open_files': 0, 'processes': []}
        except Exception as e:
            logger.error(f"Failed to collect access patterns: {str(e)}")
        
        # 3. Get ZFS dataset properties
        try:
            dataset_info = await self.middleware.call(
                'zfs.dataset.query',
                [['id', '=', dataset]],
                {'get': True}
            )
            
            if 'properties' in dataset_info:
                metrics['compressratio'] = dataset_info['properties'].get('compressratio', {}).get('value')
                metrics['used'] = dataset_info['properties'].get('used', {}).get('value')
                metrics['available'] = dataset_info['properties'].get('available', {}).get('value')
                metrics['recordsize'] = dataset_info['properties'].get('recordsize', {}).get('value')
        except Exception as e:
            logger.error(f"Failed to get dataset properties: {str(e)}")
        
        # 4. Get historical telemetry if available
        try:
            telemetry = await self.middleware.call(
                'datastore.query',
                'nestgate.telemetry',
                [['tel_dataset', '=', dataset]],
                {'order_by': ['-tel_timestamp'], 'limit': 10, 'prefix': 'tel_'}
            )
            
            if telemetry:
                # Calculate average metrics
                avg_read_ops = sum(t['read_ops'] for t in telemetry) / len(telemetry)
                avg_write_ops = sum(t['write_ops'] for t in telemetry) / len(telemetry)
                avg_read_bytes = sum(t['read_bytes'] for t in telemetry) / len(telemetry)
                avg_write_bytes = sum(t['write_bytes'] for t in telemetry) / len(telemetry)
                
                metrics['historical'] = {
                    'avg_read_ops': avg_read_ops,
                    'avg_write_ops': avg_write_ops,
                    'avg_read_bytes': avg_read_bytes,
                    'avg_write_bytes': avg_write_bytes,
                    'samples': len(telemetry)
                }
                
                # Detect trend (increasing, decreasing, stable)
                if len(telemetry) >= 2:
                    first = telemetry[-1]  # Oldest
                    last = telemetry[0]    # Newest
                    
                    # Calculate percent change
                    metrics['historical']['read_ops_trend'] = (
                        (last['read_ops'] - first['read_ops']) / max(1, first['read_ops']) * 100
                    )
                    metrics['historical']['write_ops_trend'] = (
                        (last['write_ops'] - first['write_ops']) / max(1, first['write_ops']) * 100
                    )
        except Exception as e:
            logger.error(f"Failed to get historical telemetry: {str(e)}")
        
        return metrics
    
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
        lines = iostat_output.strip().split('\n')
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
                        
                        # Calculate bytes per operation (to determine sequential vs random)
                        bytes_per_read = read_bytes / max(read_ops, 1)
                        bytes_per_write = write_bytes / max(write_ops, 1)
                        
                        # Determine access pattern
                        if read_ops > 100 and bytes_per_read > 64 * 1024:
                            io_pattern = 'sequential_read'
                        elif write_ops > 100 and bytes_per_write > 64 * 1024:
                            io_pattern = 'sequential_write'
                        elif read_ops > 100:
                            io_pattern = 'random_read'
                        elif write_ops > 100:
                            io_pattern = 'random_write'
                        else:
                            io_pattern = 'mixed'
                            
                        return {
                            'read_ops': read_ops,
                            'write_ops': write_ops,
                            'read_bytes': read_bytes,
                            'write_bytes': write_bytes,
                            'bytes_per_read': bytes_per_read,
                            'bytes_per_write': bytes_per_write,
                            'io_pattern': io_pattern
                        }
                    except (ValueError, IndexError, ZeroDivisionError):
                        logger.error(f"Failed to parse iostat output for {dataset}")
        
        return {}
    
    @private
    async def _parse_access_patterns(self, lsof_output, dataset_path):
        """
        Parse lsof output to extract file access patterns.
        
        Args:
            lsof_output (str): Output from lsof command
            dataset_path (str): Path to dataset mount point
            
        Returns:
            dict: Access pattern information
        """
        lines = lsof_output.strip().split('\n')
        if len(lines) <= 1:
            # Only header, no files
            return {'open_files': 0, 'processes': []}
        
        processes = {}
        file_types = {'regular': 0, 'directory': 0, 'character': 0, 'block': 0, 'socket': 0, 'fifo': 0}
        file_modes = {'read': 0, 'write': 0, 'readwrite': 0}
        
        for line in lines[1:]:  # Skip header line
            fields = re.split(r'\s+', line.strip())
            if len(fields) >= 9:
                try:
                    command = fields[0]
                    pid = fields[1]
                    user = fields[2]
                    fd = fields[3]
                    file_type = fields[4]
                    file_path = ' '.join(fields[8:])
                    
                    # Track processes
                    proc_key = f"{command}:{pid}"
                    if proc_key not in processes:
                        processes[proc_key] = {
                            'command': command,
                            'pid': pid,
                            'user': user,
                            'files': 0
                        }
                    processes[proc_key]['files'] += 1
                    
                    # Track file types
                    if 'r' in fd:
                        file_modes['read'] += 1
                    if 'w' in fd:
                        file_modes['write'] += 1
                    if 'u' in fd:
                        file_modes['readwrite'] += 1
                    
                    # Track file type
                    if file_type == 'REG':
                        file_types['regular'] += 1
                    elif file_type == 'DIR':
                        file_types['directory'] += 1
                    elif file_type == 'CHR':
                        file_types['character'] += 1
                    elif file_type == 'BLK':
                        file_types['block'] += 1
                    elif file_type == 'SOCK':
                        file_types['socket'] += 1
                    elif file_type == 'FIFO':
                        file_types['fifo'] += 1
                except (IndexError, ValueError):
                    continue
        
        return {
            'open_files': len(lines) - 1,
            'processes': list(processes.values()),
            'file_types': file_types,
            'file_modes': file_modes
        }
    
    @private
    async def _analyze_workload(self, metrics, config):
        """
        Analyze metrics to determine AI workload type.
        
        Args:
            metrics (dict): Collected metrics
            config (dict): Detection configuration
            
        Returns:
            str: Detected workload type or None
        """
        # Check if we have enough data
        if not metrics or 'read_ops' not in metrics or 'write_ops' not in metrics:
            logger.warning("Insufficient metrics data for analysis")
            return None
        
        # Get historical data if available
        historical = metrics.get('historical', {})
        has_history = historical.get('samples', 0) >= config.get('min_samples', 10)
        
        # Extract current metrics
        read_ops = metrics['read_ops']
        write_ops = metrics['write_ops']
        read_bytes = metrics.get('read_bytes', 0)
        write_bytes = metrics.get('write_bytes', 0)
        io_pattern = metrics.get('io_pattern', 'unknown')
        
        # Access patterns
        access_patterns = metrics.get('access_patterns', {})
        open_files = access_patterns.get('open_files', 0)
        processes = access_patterns.get('processes', [])
        
        # Log detection data for debugging
        logger.debug(f"AI Workload Detection: io_pattern={io_pattern}, read_ops={read_ops}, write_ops={write_ops}")
        
        # AI process detection
        ai_processes = []
        for proc in processes:
            cmd = proc.get('command', '').lower()
            if any(ai_proc in cmd for ai_proc in ['python', 'pytorch', 'tensorflow', 'nvidia-smi', 'ml', 'cuda']):
                ai_processes.append(proc)
        
        has_ai_processes = len(ai_processes) > 0
        
        # Detect patterns
        
        # 1. Inference pattern: high read ops, low write ops, often random access
        if (read_ops > 1000 and write_ops < 100) or (
                io_pattern in ['random_read', 'sequential_read'] and read_ops > write_ops * 10):
            if has_ai_processes or has_history:
                return 'inference'
        
        # 2. Training pattern: balanced read/write, often sequential read with periodic writes
        if (read_ops > 500 and write_ops > 500) or (
                io_pattern in ['sequential_read', 'mixed'] and read_ops > 300 and write_ops > 300):
            if has_ai_processes or has_history:
                return 'training'
        
        # 3. Checkpointing pattern: burst writes, often larger than reads
        if write_ops > 1000 and write_bytes > read_bytes * 2:
            if has_history:
                # Look for spikes in write operations compared to historical average
                avg_write_ops = historical.get('avg_write_ops', 0)
                if avg_write_ops > 0 and write_ops > avg_write_ops * 2:
                    return 'checkpointing'
            elif has_ai_processes:
                return 'checkpointing'
        
        # No clear pattern detected
        return None


def setup(middleware):
    middleware.logger.info("Loading AI Workload Detector plugin") 