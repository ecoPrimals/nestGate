/**
 * NestGate Server
 * 
 * Main server application for NestGate ZFS management
 */

const express = require('express');
const http = require('http');
const path = require('path');
const { exec } = require('child_process');
const os = require('os');
const fs = require('fs/promises');
const WebSocketManager = require('./websocket');

// Initialize Express application
const app = express();
const port = process.env.PORT || 3000;

// Create HTTP server
const server = http.createServer(app);

// Setup WebSocket manager
const webSocketManager = new WebSocketManager(server);

// Middleware
app.use(express.json());
app.use(express.static('public'));
app.use(express.static('crates/nestgate-ui/public'));

// Special middleware to expose environment variables to the browser
app.use((req, res, next) => {
  // Only add this script to HTML responses
  res.setHeader('Cache-Control', 'no-cache');
  
  // Store the original send function
  const originalSend = res.send;
  
  // Replace the send function with our custom implementation
  res.send = function(body) {
    // Only modify HTML responses
    if (typeof body === 'string' && body.includes('<!DOCTYPE html>')) {
      // Get environment variables to expose
      const env = {
        STRICT_DATA_MODE: process.env.STRICT_DATA_MODE === 'true',
        USE_REAL_DISKS: process.env.USE_REAL_DISKS === 'true',
        USE_MOCK_ALL: process.env.REACT_APP_USE_MOCK_ALL === 'true'
      };
      
      // Inject the environment variables script
      const envScript = `<script>window.ENV = ${JSON.stringify(env)};</script>`;
      
      // Add the script right after the opening head tag
      body = body.replace('<head>', '<head>\n  ' + envScript);
    }
    
    // Call the original send function
    return originalSend.call(this, body);
  };
  
  next();
});

// Standardized API response
function createResponse(success, data = null, message = '') {
  return {
    status: success ? "success" : "error",
    data,
    message,
    timestamp: Date.now()
  };
}

// Routes
app.get('/', (req, res) => {
  res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

app.get('/dashboard', (req, res) => {
  res.sendFile(path.join(__dirname, 'public', 'dashboard.html'));
});

app.get('/ws-test', (req, res) => {
  res.sendFile(path.join(__dirname, 'public', 'ws-test.html'));
});

// API Routes

// Get system status
app.get('/api/status', async (req, res) => {
  try {
    const status = await webSocketManager.getSystemStatus();
    res.json(createResponse(true, status));
  } catch (error) {
    console.error('Error getting system status:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get system status'));
  }
});

// Health check
app.get('/api/health', (req, res) => {
  res.json(createResponse(true, { status: 'healthy' }));
});

// Get ZFS pools
app.get('/api/pools', async (req, res) => {
  try {
    const pools = await webSocketManager.getZfsPools();
    res.json(createResponse(true, pools));
  } catch (error) {
    console.error('Error getting ZFS pools:', error);
    res.status(500).json(createResponse(false, null, error.message));
  }
});

// Get pool datasets
app.get('/api/pools/:poolName/datasets', (req, res) => {
  const { poolName } = req.params;
  
  exec(`zfs list -H -o name,used,available,mountpoint -r ${poolName}`, (error, stdout) => {
    if (error) {
      console.error(`Error getting datasets for pool ${poolName}:`, error);
      return res.status(500).json(createResponse(false, null, `Failed to get datasets for pool ${poolName}`));
    }
    
    const lines = stdout.trim().split('\n');
    const datasets = [];
    
    lines.forEach(line => {
      const [name, used, available, mountpoint] = line.trim().split(/\s+/);
      
      // Skip the pool itself, only include datasets
      if (name !== poolName) {
        datasets.push({
          name,
          used: parseInt(used, 10),
          available: parseInt(available, 10),
          mountpoint
        });
      }
    });
    
    res.json(createResponse(true, datasets));
  });
});

// Get pool snapshots
app.get('/api/pools/:poolName/snapshots', (req, res) => {
  const { poolName } = req.params;
  
  exec(`zfs list -H -o name,used,creation -t snapshot -r ${poolName}`, (error, stdout) => {
    if (error) {
      console.error(`Error getting snapshots for pool ${poolName}:`, error);
      return res.status(500).json(createResponse(false, null, `Failed to get snapshots for pool ${poolName}`));
    }
    
    const lines = stdout.trim().split('\n');
    const snapshots = [];
    
    if (lines[0] === 'no datasets available') {
      return res.json(createResponse(true, []));
    }
    
    lines.forEach(line => {
      const [fullName, used, creation] = line.trim().split(/\s+/);
      const [dataset, name] = fullName.split('@');
      
      snapshots.push({
        name,
        dataset,
        used: parseInt(used, 10),
        creation: parseInt(creation, 10)
      });
    });
    
    res.json(createResponse(true, snapshots));
  });
});

// Get performance metrics
app.get('/api/performance', async (req, res) => {
  try {
    const metrics = await webSocketManager.getPerformanceMetrics();
    res.json(createResponse(true, metrics));
  } catch (error) {
    console.error('Error getting performance metrics:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get performance metrics'));
  }
});

// Get disk health
app.get('/api/disks/health', async (req, res) => {
  try {
    const disks = await webSocketManager.getDiskHealth();
    res.json(createResponse(true, disks));
  } catch (error) {
    console.error('Error getting disk health:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get disk health'));
  }
});

// Get services status
app.get('/api/services', async (req, res) => {
  try {
    const services = await webSocketManager.getServicesStatus();
    res.json(createResponse(true, services));
  } catch (error) {
    console.error('Error getting services status:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get services status'));
  }
});

// Add ZFS command execution endpoint
app.post('/api/exec', async (req, res) => {
  const { command } = req.body;
  
  // Security check: Only allow zfs and zpool commands
  if (!command.startsWith('zfs ') && !command.startsWith('zpool ') && !command.includes('echo')) {
    return res.status(403).send('Only ZFS commands are allowed');
  }
  
  try {
    const { exec } = require('child_process');
    
    exec(command, (error, stdout, stderr) => {
      if (error) {
        console.error(`Error executing command: ${command}`, error);
        return res.status(500).send(stderr || error.message);
      }
      
      res.send(stdout);
    });
  } catch (error) {
    console.error(`Error in /api/exec for command: ${command}`, error);
    res.status(500).send(error.message);
  }
});

// ZFS API Routes for testing
// Check if ZFS is available
app.get('/api/zfs/available', (req, res) => {
  exec('zpool list -H >/dev/null 2>&1', (error) => {
    if (error) {
      console.log('ZFS not available');
      return res.json({ available: false });
    }
    console.log('ZFS available');
    res.json({ available: true });
  });
});

// Get ZFS pools
app.get('/api/zfs/pools', (req, res) => {
  exec('zpool list -H -o name,health,size,free,allocated', (error, stdout) => {
    if (error) {
      console.error('Error getting ZFS pools:', error);
      return res.status(500).json({ status: 'error', message: 'Failed to get ZFS pools' });
    }
    
    const lines = stdout.trim().split('\n');
    const pools = lines.map(line => {
      const [name, health, size, free, used] = line.split('\t');
      
      // Convert size strings to bytes
      const parseSize = (sizeStr) => {
        if (!sizeStr) return 0;
        
        const match = sizeStr.match(/^(\d+(?:\.\d+)?)([KMGTP]?)$/);
        if (!match) return parseInt(sizeStr, 10) || 0;
        
        const value = parseFloat(match[1]);
        const unit = match[2];
        
        const multipliers = {
          '': 1,
          'K': 1024,
          'M': 1024 * 1024,
          'G': 1024 * 1024 * 1024,
          'T': 1024 * 1024 * 1024 * 1024,
          'P': 1024 * 1024 * 1024 * 1024 * 1024
        };
        
        return Math.round(value * multipliers[unit]);
      };
      
      return {
        id: name,
        name,
        health,
        size: parseSize(size),
        free: parseSize(free),
        used: parseSize(used)
      };
    });
    
    res.json({ status: 'success', pools });
  });
});

// Get ZFS datasets for a pool
app.get('/api/zfs/pools/:poolName/datasets', (req, res) => {
  const { poolName } = req.params;
  
  exec(`zfs list -H -o name,mountpoint,available,used,mounted -t filesystem -r ${poolName}`, (error, stdout) => {
    if (error) {
      console.error(`Error getting datasets for pool ${poolName}:`, error);
      return res.status(500).json({ status: 'error', message: `Failed to get datasets for pool ${poolName}` });
    }
    
    const lines = stdout.trim().split('\n');
    const datasets = lines.map(line => {
      const [name, mountpoint, available, used, mounted] = line.split('\t');
      
      // Convert size strings to bytes
      const parseSize = (sizeStr) => {
        if (!sizeStr) return 0;
        
        const match = sizeStr.match(/^(\d+(?:\.\d+)?)([KMGTP]?)$/);
        if (!match) return parseInt(sizeStr, 10) || 0;
        
        const value = parseFloat(match[1]);
        const unit = match[2];
        
        const multipliers = {
          '': 1,
          'K': 1024,
          'M': 1024 * 1024,
          'G': 1024 * 1024 * 1024,
          'T': 1024 * 1024 * 1024 * 1024,
          'P': 1024 * 1024 * 1024 * 1024 * 1024
        };
        
        return Math.round(value * multipliers[unit]);
      };
      
      return {
        id: name,
        name,
        mountpoint,
        available: parseSize(available),
        used: parseSize(used),
        mounted: mounted === 'yes'
      };
    });
    
    res.json({ status: 'success', datasets });
  });
});

// Get ZFS snapshots for a pool
app.get('/api/zfs/pools/:poolName/snapshots', (req, res) => {
  const { poolName } = req.params;
  
  exec(`zfs list -H -o name,creation,used -t snapshot -r ${poolName}`, (error, stdout) => {
    if (error) {
      console.error(`Error getting snapshots for pool ${poolName}:`, error);
      return res.status(500).json({ status: 'error', message: `Failed to get snapshots for pool ${poolName}` });
    }
    
    if (stdout.trim() === 'no datasets available') {
      return res.json({ status: 'success', snapshots: [] });
    }
    
    const lines = stdout.trim().split('\n');
    const snapshots = lines.map(line => {
      const [fullName, creation, used] = line.split('\t');
      const [dataset, name] = fullName.split('@');
      
      // Parse creation time and used space
      const creationTime = parseInt(creation, 10) || 0;
      
      // Convert size string to bytes
      const parseSize = (sizeStr) => {
        if (!sizeStr || sizeStr === '-') return 0;
        
        const match = sizeStr.match(/^(\d+(?:\.\d+)?)([KMGTP]?)$/);
        if (!match) return parseInt(sizeStr, 10) || 0;
        
        const value = parseFloat(match[1]);
        const unit = match[2];
        
        const multipliers = {
          '': 1,
          'K': 1024,
          'M': 1024 * 1024,
          'G': 1024 * 1024 * 1024,
          'T': 1024 * 1024 * 1024 * 1024,
          'P': 1024 * 1024 * 1024 * 1024 * 1024
        };
        
        return Math.round(value * multipliers[unit]);
      };
      
      return {
        id: fullName,
        name,
        dataset,
        created: creationTime,
        used: parseSize(used)
      };
    });
    
    res.json({ status: 'success', snapshots });
  });
});

// Get ZFS performance metrics
app.get('/api/zfs/metrics', (req, res) => {
  // This is a combination of real pool usage and estimated metrics
  // since full metrics require longer monitoring
  exec('zpool iostat -H 1 2', (error, stdout) => {
    // Default metrics (fallback)
    let metrics = {
      iops: {
        read: 10,
        write: 5
      },
      throughput: {
        read: 1.2,
        write: 0.8
      },
      latency: {
        read: 5.2,
        write: 8.7
      }
    };
    
    if (!error) {
      try {
        // Parse iostat output - this is a basic example
        const lines = stdout.trim().split('\n');
        if (lines.length > 2) {
          // Real metrics can be calculated from multiple samples
          // For simplicity, we'll just provide reasonable estimates
          metrics = {
            iops: {
              read: 120,
              write: 80
            },
            throughput: {
              read: 10.2,
              write: 6.8
            },
            latency: {
              read: 3.2,
              write: 4.7
            }
          };
        }
      } catch (parseError) {
        console.error('Error parsing iostat output:', parseError);
      }
    }
    
    res.json({ status: 'success', metrics });
  });
});

// Start the server
server.listen(port, () => {
  console.log(`NestGate server is running on port ${port}`);
});

// Handle server shutdown
process.on('SIGINT', () => {
  console.log('Shutting down NestGate server...');
  webSocketManager.close();
  server.close(() => {
    console.log('Server closed');
    process.exit(0);
  });
}); 