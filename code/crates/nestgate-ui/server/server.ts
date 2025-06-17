/**
 * NestGate Server
 * 
 * Main server application for NestGate ZFS management
 */

import express, { Request, Response, NextFunction } from 'express';
import http from 'http';
import path from 'path';
import { exec } from 'child_process';
import os from 'os';
import fs from 'fs/promises';
import WebSocketManager from './websocket';
import { isStrictLiveMode, getConfig } from '../src/config';
import { validateStrictModeOnStartup } from './strictModeValidator';
import logger from './logger';

// Initialize Express application
const app = express();

// Require PORT from environment - no hardcoded fallbacks
const PORT = process.env.PORT || process.env.SERVER_PORT || process.env.WEBSOCKET_PORT;
if (!PORT) {
  console.error('ERROR: PORT, SERVER_PORT, or WEBSOCKET_PORT environment variable is required');
  console.error('This should be set by the NestGate Port Manager');
  process.exit(1);
}

const port = parseInt(PORT, 10);
if (isNaN(port) || port < 1 || port > 65535) {
  console.error(`ERROR: Invalid port value: ${PORT}`);
  process.exit(1);
}

// Create HTTP server
const server = http.createServer(app);

// Setup WebSocket manager
const webSocketManager = new WebSocketManager(server);

// Middleware
app.use(express.json());
app.use(express.static('public'));
app.use(express.static('crates/nestgate-ui/public'));

// Strict mode middleware to block mock data endpoints
app.use((req: Request, res: Response, next: NextFunction) => {
  // Check if we're in strict mode
  if (isStrictLiveMode()) {
    // List of patterns that indicate mock data endpoints
    const mockEndpointPatterns = [
      /\/api\/mock\//i,
      /\/api\/.*\/mock/i,
      /\/mock-api\//i,
      /simulate=true/i,
      /mock=true/i,
      /test=true/i
    ];
    
    // Check URL against mock patterns
    const isMockEndpoint = mockEndpointPatterns.some(pattern => 
      pattern.test(req.path) || 
      (req.query && pattern.test(JSON.stringify(req.query)))
    );
    
    // Also check for mock-related headers or body parameters
    const hasMockHeader = req.headers['x-mock-data'] === 'true' || 
                          req.headers['x-test-mode'] === 'true';
    
    const hasMockBody = req.body && 
                        (req.body.mock === true || 
                         req.body.mockData === true || 
                         req.body.testMode === true);
    
    if (isMockEndpoint || hasMockHeader || hasMockBody) {
      // Log the attempt
      logger.logMockDataAccess(req, 'Blocked by strict mode middleware');
      
      // Return a forbidden response
      return res.status(403).json(createResponse(false, null, 'Mock data endpoints are disabled in strict mode'));
    }
  }
  
  next();
});

// Special middleware to expose environment variables to the browser
app.use((req: Request, res: Response, next: NextFunction) => {
  // Only add this script to HTML responses
  res.setHeader('Cache-Control', 'no-cache');
  
  // Store the original send function
  const originalSend = res.send;
  
  // Replace the send function with our custom implementation
  res.send = function(body) {
    // Only modify HTML responses
    if (typeof body === 'string' && body.includes('<!DOCTYPE html>')) {
      // Get environment variables to expose
      const config = getConfig();
      
      // Force strict mode to always be true for production
      const env = {
        // Force these values to true for consistency
        STRICT_DATA_MODE: true,
        REACT_APP_STRICT_DATA_MODE: true,
        USE_REAL_DISKS: true,
        REACT_APP_USE_REAL_DISKS: true,
        USE_MOCK_ALL: false,
        REACT_APP_USE_MOCK_ALL: false,
        REACT_APP_USE_MOCK_WEBSOCKET: false
      };
      
      // Inject the environment variables script
      const envScript = `<script>window.ENV = ${JSON.stringify(env)};</script>`;
      
      // Add the script as the first element in the head tag
      if (body.includes('<head>')) {
        body = body.replace('<head>', `<head>\n  ${envScript}`);
      } else if (body.includes('<HEAD>')) {
        body = body.replace('<HEAD>', `<HEAD>\n  ${envScript}`);
      } else {
        // Fallback: insert after doctype if head tag not found
        body = body.replace('<!DOCTYPE html>', `<!DOCTYPE html>\n${envScript}`);
      }
    }
    
    // Call the original send function
    return originalSend.call(this, body);
  };
  
  next();
});

// Standardized API response
interface ApiResponse<T> {
  status: 'success' | 'error';
  data: T | null;
  message: string;
  timestamp: number;
}

function createResponse<T>(success: boolean, data: T | null = null, message = ''): ApiResponse<T> {
  return {
    status: success ? "success" : "error",
    data,
    message,
    timestamp: Date.now()
  };
}

// Routes
app.get('/', (req: Request, res: Response) => {
  res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

app.get('/dashboard', (req: Request, res: Response) => {
  res.sendFile(path.join(__dirname, 'public', 'dashboard.html'));
});

app.get('/ws-test', (req: Request, res: Response) => {
  res.sendFile(path.join(__dirname, 'public', 'ws-test.html'));
});

// API Routes

// Get system status
app.get('/api/status', async (req: Request, res: Response) => {
  try {
    const status = await webSocketManager.getSystemStatus();
    res.json(createResponse(true, status));
  } catch (error) {
    console.error('Error getting system status:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get system status'));
  }
});

// Health check
app.get('/api/health', (req: Request, res: Response) => {
  res.json(createResponse(true, { status: 'healthy' }));
});

// Get ZFS pools
app.get('/api/pools', async (req: Request, res: Response) => {
  try {
    const pools = await webSocketManager.getZfsPools();
    res.json(createResponse(true, pools));
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    console.error('Error getting ZFS pools:', error);
    res.status(500).json(createResponse(false, null, errorMessage));
  }
});

// Get pool datasets
app.get('/api/pools/:poolName/datasets', (req: Request, res: Response) => {
  const { poolName } = req.params;
  
  exec(`zfs list -H -o name,used,available,mountpoint -r ${poolName}`, (error, stdout) => {
    if (error) {
      console.error(`Error getting datasets for pool ${poolName}:`, error);
      return res.status(500).json(createResponse(false, null, `Failed to get datasets for pool ${poolName}`));
    }
    
    const lines = stdout.trim().split('\n');
    const datasets: Array<{name: string; used: number; available: number; mountpoint: string}> = [];
    
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
app.get('/api/pools/:poolName/snapshots', (req: Request, res: Response) => {
  const { poolName } = req.params;
  
  exec(`zfs list -H -o name,used,creation -t snapshot -r ${poolName}`, (error, stdout) => {
    if (error) {
      console.error(`Error getting snapshots for pool ${poolName}:`, error);
      return res.status(500).json(createResponse(false, null, `Failed to get snapshots for pool ${poolName}`));
    }
    
    const lines = stdout.trim().split('\n');
    const snapshots: Array<{name: string; dataset: string; used: number; creation: number}> = [];
    
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
app.get('/api/performance', async (req: Request, res: Response) => {
  try {
    const metrics = await webSocketManager.getPerformanceMetrics();
    res.json(createResponse(true, metrics));
  } catch (error) {
    console.error('Error getting performance metrics:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get performance metrics'));
  }
});

// Get disk health
app.get('/api/disks/health', async (req: Request, res: Response) => {
  try {
    const disks = await webSocketManager.getDiskHealth();
    res.json(createResponse(true, disks));
  } catch (error) {
    console.error('Error getting disk health:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get disk health'));
  }
});

// Get services status
app.get('/api/services', async (req: Request, res: Response) => {
  try {
    const services = await webSocketManager.getServicesStatus();
    res.json(createResponse(true, services));
  } catch (error) {
    console.error('Error getting services status:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get services status'));
  }
});

// Add ZFS command execution endpoint
app.post('/api/exec', async (req: Request, res: Response) => {
  const { command } = req.body;
  
  // Security check: Only allow zfs and zpool commands
  if (!command.startsWith('zfs ') && !command.startsWith('zpool ') && !command.includes('echo')) {
    return res.status(403).send('Only ZFS commands are allowed');
  }
  
  try {
    exec(command, (error, stdout, stderr) => {
      if (error) {
        console.error(`Error executing command: ${command}`, error);
        return res.status(500).send(stderr || error.message);
      }
      
      res.send(stdout);
    });
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    console.error(`Error in /api/exec for command: ${command}`, error);
    res.status(500).send(errorMessage);
  }
});

// Start the server
server.listen(port, () => {
  const strictMode = isStrictLiveMode() ? ' in strict mode' : '';
  logger.info(`NestGate server is running on port ${port}${strictMode}`);
  
  // Validate strict mode configuration
  validateStrictModeOnStartup();
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