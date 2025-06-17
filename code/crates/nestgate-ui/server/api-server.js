/**
 * NestGate API Server
 * 
 * Dedicated API server with authentication for NestGate ZFS management
 */

const express = require('express');
const cors = require('cors');
const http = require('http');
const { exec } = require('child_process');
const WebSocketManager = require('./server/websocket');

// Initialize Express application
const app = express();
const port = process.env.API_PORT || 3030;

// Create HTTP server
const server = http.createServer(app);

// Setup WebSocket manager for data access
const webSocketManager = new WebSocketManager(server);

// Middleware
app.use(express.json());
app.use(cors({
  origin: ['http://localhost:3000', 'http://localhost:5173'],
  methods: ['GET', 'POST', 'PUT', 'DELETE'],
  allowedHeaders: ['Content-Type', 'Authorization']
}));

// Authentication tokens (in a real app, this would be in a database)
const validTokens = {
  'admin-api-token': { id: '1', username: 'admin', role: 'admin' },
  'readonly-api-token': { id: '2', username: 'readonly', role: 'readonly' }
};

// Authentication middleware
const authenticate = (req, res, next) => {
  const authHeader = req.headers.authorization;
  
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return res.status(401).json({
      status: 'error',
      message: 'Authentication required',
      timestamp: Date.now()
    });
  }
  
  const token = authHeader.split(' ')[1];
  const user = validTokens[token];
  
  if (!user) {
    return res.status(401).json({
      status: 'error',
      message: 'Invalid or expired token',
      timestamp: Date.now()
    });
  }
  
  // Add user to request object
  req.user = user;
  next();
};

// Role-based authorization middleware
const authorize = (roles = []) => {
  return (req, res, next) => {
    if (!req.user) {
      return res.status(401).json({
        status: 'error',
        message: 'Authentication required',
        timestamp: Date.now()
      });
    }

    const userRole = req.user.role;
    
    if (roles.length && !roles.includes(userRole)) {
      return res.status(403).json({
        status: 'error',
        message: 'Insufficient permissions',
        timestamp: Date.now()
      });
    }
    
    next();
  };
};

// Standardized API response
function createResponse(success, data = null, message = '') {
  return {
    status: success ? "success" : "error",
    data,
    message,
    timestamp: Date.now()
  };
}

// Authentication endpoints
app.post('/api/auth/login', (req, res) => {
  const { username, password } = req.body;
  
  // In a real application, this would validate against a database
  if (username === 'admin' && password === 'admin') {
    return res.json({
      token: 'admin-api-token',
      user: {
        id: '1',
        username: 'admin',
        role: 'admin'
      }
    });
  }
  
  if (username === 'readonly' && password === 'readonly') {
    return res.json({
      token: 'readonly-api-token',
      user: {
        id: '2',
        username: 'readonly',
        role: 'readonly'
      }
    });
  }
  
  res.status(401).json({
    status: 'error',
    message: 'Invalid credentials',
    timestamp: Date.now()
  });
});

// Get authenticated user
app.get('/api/auth/me', authenticate, (req, res) => {
  res.json(createResponse(true, req.user));
});

// API Routes - All protected by authentication

// Get system status
app.get('/api/status', authenticate, async (req, res) => {
  try {
    const status = await webSocketManager.getSystemStatus();
    res.json(createResponse(true, status));
  } catch (error) {
    console.error('Error getting system status:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get system status'));
  }
});

// Health check (public)
app.get('/api/health', (req, res) => {
  res.json(createResponse(true, { status: 'healthy' }));
});

// Get ZFS pools
app.get('/api/pools', authenticate, async (req, res) => {
  try {
    const pools = await webSocketManager.getZfsPools();
    res.json(createResponse(true, pools));
  } catch (error) {
    console.error('Error getting ZFS pools:', error);
    res.status(500).json(createResponse(false, null, error.message));
  }
});

// Get HDD health information
app.get('/api/hdd-health', authenticate, async (req, res) => {
  try {
    const disks = await webSocketManager.getDiskHealth();
    
    // Transform the data to include health percentage and other fields
    const enhancedDisks = disks.map(disk => {
      // Generate random health percentage between 70 and 100
      const health = Math.floor(Math.random() * 30) + 70;
      
      // Random errors count (0-3)
      const errors = Math.floor(Math.random() * 4);
      
      return {
        ...disk,
        health,
        errors,
        status: health > 90 ? 'ONLINE' : health > 70 ? 'DEGRADED' : 'FAILING'
      };
    });
    
    res.json(enhancedDisks);
  } catch (error) {
    console.error('Error getting disk health:', error);
    res.status(500).json(createResponse(false, null, 'Failed to get disk health'));
  }
});

// Write operations - restricted to admin role
app.post('/api/pools/:poolName/snapshots', authenticate, authorize(['admin']), (req, res) => {
  const { poolName } = req.params;
  const { datasetName, snapshotName } = req.body;
  
  if (!datasetName || !snapshotName) {
    return res.status(400).json(createResponse(false, null, 'Missing required parameters'));
  }
  
  // In a real implementation, this would execute ZFS commands
  // For demo, just simulate success
  res.json(createResponse(true, {
    poolName,
    datasetName,
    snapshotName,
    created: Date.now()
  }));
});

// Start the server
server.listen(port, () => {
  console.log(`NestGate API server is running on port ${port}`);
});

// Handle server shutdown
process.on('SIGINT', () => {
  console.log('Shutting down NestGate API server...');
  webSocketManager.close();
  server.close(() => {
    console.log('API server closed');
    process.exit(0);
  });
}); 