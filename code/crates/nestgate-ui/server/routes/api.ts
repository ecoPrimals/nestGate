/**
 * NestGate API Routes
 * 
 * Defines the API endpoints for NestGate services
 */

import express from 'express';
import { ApiResponse } from '../types/index';
import WebSocketManager from '../websocket';
import { Router } from 'express';

const router: Router = express.Router();
let wsManager: WebSocketManager | null = null;

/**
 * Set the WebSocket manager instance for API access
 */
export function setWebSocketManager(manager: WebSocketManager): void {
  wsManager = manager;
}

// Helper function to create standardized API responses
function createResponse<T>(success: boolean, data: T | null = null, message = ''): ApiResponse<T> {
  return {
    status: success ? 'success' : 'error',
    data,
    message,
    timestamp: Date.now()
  };
}

// API routes
router.get('/health', (req, res) => {
  res.json(createResponse(true, { status: 'healthy' }));
});

// Get ZFS pools
router.get('/pools', async (req, res) => {
  try {
    if (!wsManager) {
      throw new Error('WebSocket manager not initialized');
    }
    
    const pools = await wsManager.getZfsPools();
    res.json(createResponse(true, pools));
  } catch (error) {
    console.error('Error fetching ZFS pools:', error);
    res.status(500).json(createResponse(false, null, `Failed to fetch ZFS pools: ${error instanceof Error ? error.message : String(error)}`));
  }
});

// Get system status
router.get('/system', async (req, res) => {
  try {
    if (!wsManager) {
      throw new Error('WebSocket manager not initialized');
    }
    
    const status = await wsManager.getSystemStatus();
    res.json(createResponse(true, status));
  } catch (error) {
    console.error('Error fetching system status:', error);
    res.status(500).json(createResponse(false, null, `Failed to fetch system status: ${error instanceof Error ? error.message : String(error)}`));
  }
});

// Get performance metrics
router.get('/performance', async (req, res) => {
  try {
    if (!wsManager) {
      throw new Error('WebSocket manager not initialized');
    }
    
    const metrics = await wsManager.getPerformanceMetrics();
    res.json(createResponse(true, metrics));
  } catch (error) {
    console.error('Error fetching performance metrics:', error);
    res.status(500).json(createResponse(false, null, `Failed to fetch performance metrics: ${error instanceof Error ? error.message : String(error)}`));
  }
});

export default router; 