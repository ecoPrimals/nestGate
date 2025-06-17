/**
 * NestGate API Router
 * Handles all API routes for the NestGate storage management system
 */

const express = require('express');
const { exec } = require('child_process');
const util = require('util');
const execPromise = util.promisify(exec);

const router = express.Router();
let wsManager = null;

/**
 * Set WebSocket manager for the API router
 */
const setWebSocketManager = (manager) => {
  wsManager = manager;
};

/**
 * Check if ZFS is available
 */
router.get('/zfs/available', async (req, res) => {
  try {
    const { stdout, stderr } = await execPromise('which zpool');
    const available = stdout.trim().length > 0;
    
    res.json({
      available,
      message: available ? 'ZFS is available' : 'ZFS is not available'
    });
  } catch (error) {
    res.json({
      available: false,
      message: 'ZFS is not available',
      error: error.message
    });
  }
});

/**
 * Get ZFS pools
 */
router.get('/zfs/pools', async (req, res) => {
  try {
    if (!wsManager) {
      return res.status(500).json({
        error: 'WebSocket manager not initialized'
      });
    }
    
    const pools = await wsManager.getZfsPools();
    
    res.json({
      pools,
      timestamp: Date.now()
    });
  } catch (error) {
    console.error('Error getting ZFS pools:', error);
    res.status(500).json({
      error: 'Failed to retrieve ZFS pools',
      message: error.message
    });
  }
});

/**
 * Get ZFS datasets
 */
router.get('/zfs/pools/:poolName/datasets', async (req, res) => {
  try {
    const { poolName } = req.params;
    
    if (!wsManager) {
      return res.status(500).json({
        error: 'WebSocket manager not initialized'
      });
    }
    
    // In a full implementation, this would get datasets from the ZFS monitor
    // For this demo, we'll return mock datasets
    const pools = await wsManager.getZfsPools();
    const pool = pools.find(p => p.name === poolName);
    
    if (!pool) {
      return res.status(404).json({
        error: `Pool ${poolName} not found`
      });
    }
    
    // Mock datasets based on the pool
    // In a real implementation, this would come from zfs list command
    const available = pool.free;
    
    let datasets = [];
    
    if (poolName === 'nestpool') {
      datasets = [
        {
          id: `${poolName}/testdata`,
          name: `${poolName}/testdata`,
          mountpoint: `/${poolName}/testdata`,
          available: available * 0.5,
          used: pool.used * 0.7,
          mounted: true
        },
        {
          id: `${poolName}/hot`,
          name: `${poolName}/hot`,
          mountpoint: `/${poolName}/hot`,
          available: available * 0.2,
          used: pool.used * 0.2,
          mounted: true
        },
        {
          id: `${poolName}/warm`,
          name: `${poolName}/warm`,
          mountpoint: `/${poolName}/warm`,
          available: available * 0.2,
          used: pool.used * 0.1,
          mounted: true
        },
        {
          id: `${poolName}/cold`,
          name: `${poolName}/cold`,
          mountpoint: `/${poolName}/cold`,
          available: available * 0.1,
          used: 0,
          mounted: true
        }
      ];
    } else {
      datasets = [
        {
          id: `${poolName}/data`,
          name: `${poolName}/data`,
          mountpoint: `/${poolName}/data`,
          available: available,
          used: pool.used,
          mounted: true
        }
      ];
    }
    
    res.json({
      datasets,
      timestamp: Date.now()
    });
  } catch (error) {
    console.error(`Error getting datasets for pool ${req.params.poolName}:`, error);
    res.status(500).json({
      error: `Failed to retrieve datasets for pool ${req.params.poolName}`,
      message: error.message
    });
  }
});

/**
 * Get ZFS metrics
 */
router.get('/zfs/metrics', async (req, res) => {
  try {
    if (!wsManager) {
      return res.status(500).json({
        error: 'WebSocket manager not initialized'
      });
    }
    
    const metrics = await wsManager.getPerformanceMetrics();
    
    res.json({
      metrics,
      timestamp: Date.now()
    });
  } catch (error) {
    console.error('Error getting ZFS metrics:', error);
    res.status(500).json({
      error: 'Failed to retrieve ZFS metrics',
      message: error.message
    });
  }
});

/**
 * Get system status
 */
router.get('/system/status', async (req, res) => {
  try {
    if (!wsManager) {
      return res.status(500).json({
        error: 'WebSocket manager not initialized'
      });
    }
    
    const status = await wsManager.getSystemStatus();
    
    res.json({
      status,
      timestamp: Date.now()
    });
  } catch (error) {
    console.error('Error getting system status:', error);
    res.status(500).json({
      error: 'Failed to retrieve system status',
      message: error.message
    });
  }
});

/**
 * Get disk health
 */
router.get('/system/disks', async (req, res) => {
  try {
    if (!wsManager) {
      return res.status(500).json({
        error: 'WebSocket manager not initialized'
      });
    }
    
    const disks = await wsManager.getDiskHealth();
    
    res.json({
      disks,
      timestamp: Date.now()
    });
  } catch (error) {
    console.error('Error getting disk health:', error);
    res.status(500).json({
      error: 'Failed to retrieve disk health',
      message: error.message
    });
  }
});

/**
 * Get services status
 */
router.get('/system/services', async (req, res) => {
  try {
    if (!wsManager) {
      return res.status(500).json({
        error: 'WebSocket manager not initialized'
      });
    }
    
    const services = await wsManager.getServicesStatus();
    
    res.json({
      services,
      timestamp: Date.now()
    });
  } catch (error) {
    console.error('Error getting services status:', error);
    res.status(500).json({
      error: 'Failed to retrieve services status',
      message: error.message
    });
  }
});

module.exports = router;
module.exports.setWebSocketManager = setWebSocketManager; 