/**
 * WebSocket Manager for NestGate
 * 
 * Handles real-time updates and client connections
 */

const WebSocket = require('ws');
const { exec } = require('child_process');
const os = require('os');
const fs = require('fs/promises');
const path = require('path');
const ZfsMonitor = require('./monitor');

class WebSocketManager {
  constructor(server) {
    this.wss = new WebSocket.Server({ server });
    this.clients = new Set();
    this.updateIntervals = {};
    this.zfsMonitor = new ZfsMonitor(this);
    
    // Initialize WebSocket server
    this.init();
  }
  
  /**
   * Initialize WebSocket server
   */
  init() {
    console.log('Initializing WebSocket server');
    
    this.wss.on('connection', (ws, req) => {
      this.handleConnection(ws, req);
    });
    
    // Start the ZFS monitor with 3 second updates
    this.zfsMonitor.start(3000);
  }
  
  /**
   * Handle a new WebSocket connection
   */
  handleConnection(ws, req) {
    const ip = req.socket.remoteAddress;
    console.log(`New WebSocket connection from ${ip}`);
    
    // Add the client to our set
    this.clients.add(ws);
    
    // Send welcome message
    this.sendToClient(ws, {
      type: 'welcome',
      message: 'Connected to NestGate WebSocket server',
      timestamp: Date.now()
    });
    
    // Send initial data
    this.sendInitialData(ws);
    
    // Set up event handlers
    ws.on('message', (message) => {
      this.handleMessage(ws, message);
    });
    
    ws.on('close', () => {
      console.log(`WebSocket connection closed from ${ip}`);
      this.clients.delete(ws);
      
      // Clean up any intervals for this client
      this.cleanupClientIntervals(ws);
    });
    
    ws.on('error', (error) => {
      console.error(`WebSocket error from ${ip}:`, error);
      this.clients.delete(ws);
      
      // Clean up any intervals for this client
      this.cleanupClientIntervals(ws);
    });
  }
  
  /**
   * Handle a WebSocket message
   */
  handleMessage(ws, message) {
    try {
      const parsedMessage = JSON.parse(message);
      
      console.log('Received message:', parsedMessage);
      
      // Handle different message types
      if (parsedMessage.type) {
        // Handle standard type-based messages
        switch (parsedMessage.type) {
          case 'ping':
            this.sendToClient(ws, {
              type: 'pong',
              timestamp: Date.now()
            });
            break;
            
          case 'getSystemStatus':
            this.sendSystemStatus(ws);
            break;
            
          case 'getZfsPools':
            this.sendZfsPools(ws);
            break;
            
          case 'getPerformanceMetrics':
            this.sendPerformanceMetrics(ws);
            break;
            
          default:
            console.log(`Unknown message type: ${parsedMessage.type}`);
        }
      } 
      // Handle legacy or dashboard-specific messages with metric_type
      else if (parsedMessage.metric_type) {
        switch (parsedMessage.metric_type) {
          case 'SystemHealth':
            // Set up periodic system status updates
            this.setupMetricInterval(ws, 'SystemHealth', parsedMessage.interval_ms || 5000, () => {
              this.sendSystemStatus(ws);
            });
            break;
            
          case 'Performance':
            // Set up periodic performance metrics updates
            this.setupMetricInterval(ws, 'Performance', parsedMessage.interval_ms || 5000, () => {
              this.sendPerformanceMetrics(ws);
            });
            break;
            
          case 'DiskHealth':
            // Set up periodic disk health updates
            this.setupMetricInterval(ws, 'DiskHealth', parsedMessage.interval_ms || 5000, () => {
              this.sendDiskHealth(ws);
            });
            break;
            
          case 'ZfsPools':
            // Set up periodic ZFS pools updates
            this.setupMetricInterval(ws, 'ZfsPools', parsedMessage.interval_ms || 5000, () => {
              this.sendZfsPools(ws);
            });
            break;
            
          case 'Services':
            // Set up periodic services status updates
            this.setupMetricInterval(ws, 'Services', parsedMessage.interval_ms || 5000, () => {
              this.sendServicesStatus(ws);
            });
            break;
            
          default:
            console.log(`Unknown metric_type: ${parsedMessage.metric_type}`);
        }
      } else {
        console.log('Message without type or metric_type:', parsedMessage);
      }
    } catch (error) {
      console.error('Error handling WebSocket message:', error);
    }
  }
  
  /**
   * Set up a periodic interval for sending metric updates
   */
  setupMetricInterval(ws, metricType, intervalMs, callback) {
    // Clear any existing interval for this metric type
    this.clearMetricInterval(ws, metricType);
    
    // Store client intervals by WebSocket object and metric type
    if (!this.updateIntervals[ws]) {
      this.updateIntervals[ws] = {};
    }
    
    // Create a new interval
    this.updateIntervals[ws][metricType] = setInterval(() => {
      if (ws.readyState === WebSocket.OPEN) {
        callback();
      } else {
        // Clean up if the connection is closed
        this.clearMetricInterval(ws, metricType);
      }
    }, intervalMs);
    
    // Send initial data immediately
    callback();
    
    console.log(`Set up ${metricType} updates every ${intervalMs}ms`);
  }
  
  /**
   * Clear a metric interval for a client
   */
  clearMetricInterval(ws, metricType) {
    if (this.updateIntervals[ws] && this.updateIntervals[ws][metricType]) {
      clearInterval(this.updateIntervals[ws][metricType]);
      delete this.updateIntervals[ws][metricType];
    }
  }
  
  /**
   * Clean up all intervals for a client
   */
  cleanupClientIntervals(ws) {
    if (this.updateIntervals[ws]) {
      // Clear all intervals for this client
      for (const metricType in this.updateIntervals[ws]) {
        clearInterval(this.updateIntervals[ws][metricType]);
      }
      
      // Remove the client from the intervals map
      delete this.updateIntervals[ws];
    }
  }
  
  /**
   * Send a message to a specific client
   */
  sendToClient(ws, data) {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(data));
    }
  }
  
  /**
   * Broadcast a message to all connected clients
   */
  broadcast(data) {
    this.clients.forEach(client => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(JSON.stringify(data));
      }
    });
  }
  
  /**
   * Send initial data to a client
   */
  sendInitialData(ws) {
    // Send system status
    this.sendSystemStatus(ws);
    
    // Send ZFS pools
    this.sendZfsPools(ws);
    
    // Send performance metrics
    this.sendPerformanceMetrics(ws);
    
    // Send disk health
    this.sendDiskHealth(ws);
    
    // Send services status
    this.sendServicesStatus(ws);
  }
  
  /**
   * Get and send system status
   */
  async sendSystemStatus(ws) {
    const status = {
      status: 'healthy',
      cpu_usage: Math.floor(30 + Math.random() * 15), // 30-45%
      memory_usage: Math.floor(30 + Math.random() * 20), // 30-50%
      uptime: this.formatUptime(process.uptime()),
      diagnostics: {
        api_server: 'operational',
        web_interface: 'operational',
        zfs_subsystem: 'operational'
      }
    };
    
    this.sendToClient(ws, {
      type: 'systemUpdate',
      data: status,
      timestamp: Date.now()
    });
  }
  
  /**
   * Get and send ZFS pools
   */
  async sendZfsPools(ws) {
    this.sendToClient(ws, {
      type: 'poolsUpdate',
      data: this.zfsMonitor.stats.pools,
      timestamp: Date.now()
    });
  }
  
  /**
   * Get and send performance metrics
   */
  async sendPerformanceMetrics(ws) {
    this.sendToClient(ws, {
      type: 'performanceUpdate',
      data: this.zfsMonitor.stats.performance,
      timestamp: Date.now()
    });
  }
  
  /**
   * Get and send disk health info
   */
  async sendDiskHealth(ws) {
    // This would normally come from a real disk health check
    // For now, we'll use mock data
    const disks = [
      {
        device: 'sda',
        model: 'WDC WD6001FZWX',
        serial: 'WD-WX31DC5PXLPY',
        size: 6001175126016, // 6 TB
        status: 'PASSED',
        temperature: 42,
        errors: 0
      },
      {
        device: 'sdb',
        model: 'WDC WD6001FZWX',
        serial: 'WD-WX31DC5PXLPZ',
        size: 6001175126016, // 6 TB
        status: 'PASSED',
        temperature: 41,
        errors: 0
      },
      {
        device: 'sdc',
        model: 'WDC WD6001FZWX',
        serial: 'WD-WX31DC5PXLPA',
        size: 6001175126016, // 6 TB
        status: 'PASSED',
        temperature: 43,
        errors: 0
      }
    ];
    
    this.sendToClient(ws, {
      type: 'diskHealthUpdate',
      data: disks,
      timestamp: Date.now()
    });
  }
  
  /**
   * Get and send services status
   */
  async sendServicesStatus(ws) {
    // This would normally come from a real service check
    // For now, we'll use mock data
    const services = {
      nfs: 'running',
      smb: 'running'
    };
    
    this.sendToClient(ws, {
      type: 'servicesUpdate',
      data: services,
      timestamp: Date.now()
    });
  }
  
  /**
   * Format uptime in a human-readable format
   */
  formatUptime(uptime) {
    const days = Math.floor(uptime / 86400);
    const hours = Math.floor((uptime % 86400) / 3600);
    const minutes = Math.floor((uptime % 3600) / 60);
    
    return `${days}d ${hours}h ${minutes}m`;
  }
  
  /**
   * Get system status for API
   */
  async getSystemStatus() {
    return {
      status: 'healthy',
      cpu_usage: Math.floor(30 + Math.random() * 15), // 30-45%
      memory_usage: Math.floor(30 + Math.random() * 20), // 30-50%
      uptime: this.formatUptime(process.uptime()),
      diagnostics: {
        api_server: 'operational',
        web_interface: 'operational',
        zfs_subsystem: 'operational'
      }
    };
  }
  
  /**
   * Get ZFS pools for API
   */
  async getZfsPools() {
    return this.zfsMonitor.stats.pools;
  }
  
  /**
   * Get performance metrics for API
   */
  async getPerformanceMetrics() {
    return this.zfsMonitor.stats.performance;
  }
  
  /**
   * Get disk health for API
   */
  async getDiskHealth() {
    // Check if we should use real disk detection
    const useRealDisks = process.env.REACT_APP_USE_REAL_DISKS === 'true' || 
                         process.env.USE_REAL_DISKS === 'true';
    
    if (useRealDisks) {
      try {
        console.log('Attempting to get real disk information...');
        // Try to get real disk information using lsblk
        const { exec } = require('child_process');
        const util = require('util');
        const execPromise = util.promisify(exec);
        
        // Get disk list with basic info
        const { stdout: lsblkOutput } = await execPromise(
          'lsblk -d -o NAME,SIZE,MODEL,SERIAL -J'
        );
        
        const lsblkData = JSON.parse(lsblkOutput);
        
        if (lsblkData && lsblkData.blockdevices && lsblkData.blockdevices.length > 0) {
          // Get temperature information using smartctl if available
          const disks = [];
          
          for (const device of lsblkData.blockdevices) {
            // Skip loop devices and other non-physical disks
            if (device.name.startsWith('loop') || device.name.startsWith('sr')) {
              continue;
            }
            
            let temperature = 35 + Math.floor(Math.random() * 10); // Default fallback
            let status = 'PASSED';
            
            try {
              // Try to get SMART data if smartctl is available
              const { stdout: smartOutput } = await execPromise(
                `smartctl -A /dev/${device.name} | grep -i temperature`
              ).catch(() => ({ stdout: '' }));
              
              if (smartOutput) {
                const match = smartOutput.match(/\d+$/);
                if (match) {
                  temperature = parseInt(match[0], 10);
                }
              }
              
              // Try to get overall health status
              const { stdout: healthOutput } = await execPromise(
                `smartctl -H /dev/${device.name} | grep -i health`
              ).catch(() => ({ stdout: '' }));
              
              if (healthOutput && healthOutput.toLowerCase().includes('fail')) {
                status = 'FAILED';
              }
            } catch (smartError) {
              console.log(`Could not get SMART data for ${device.name}:`, smartError.message);
            }
            
            disks.push({
              id: device.name,
              device: device.name,
              model: device.model || 'Unknown Model',
              serial: device.serial || 'Unknown Serial',
              size: parseInt(device.size.replace(/[^0-9]/g, ''), 10) * 1024 * 1024 * 1024, // Convert GB to bytes
              status: status,
              temperature: temperature,
              errors: 0
            });
          }
          
          if (disks.length > 0) {
            console.log(`Found ${disks.length} real disks`);
            return disks;
          }
        }
        
        console.log('No real disks found or error parsing data, falling back to mock data');
      } catch (error) {
        console.error('Error getting real disk information:', error);
        console.log('Falling back to mock data');
      }
    }
    
    // Fall back to mock data if real detection fails or is disabled
    console.log('Using mock disk data');
    return [
      {
        device: 'sda',
        model: 'WDC WD6001FZWX',
        serial: 'WD-WX31DC5PXLPY',
        size: 6001175126016, // 6 TB
        status: 'PASSED',
        temperature: 42,
        errors: 0
      },
      {
        device: 'sdb',
        model: 'WDC WD6001FZWX',
        serial: 'WD-WX31DC5PXLPZ',
        size: 6001175126016, // 6 TB
        status: 'PASSED',
        temperature: 41,
        errors: 0
      },
      {
        device: 'sdc',
        model: 'WDC WD6001FZWX',
        serial: 'WD-WX31DC5PXLPA',
        size: 6001175126016, // 6 TB
        status: 'PASSED',
        temperature: 43,
        errors: 0
      }
    ];
  }
  
  /**
   * Get services status for API
   */
  async getServicesStatus() {
    // This would normally come from a real service check
    // For now, we'll use mock data
    return {
      nfs: 'running',
      smb: 'running'
    };
  }
  
  /**
   * Close all connections
   */
  close() {
    console.log('Closing WebSocket server');
    
    // Stop the ZFS monitor
    this.zfsMonitor.stop();
    
    // Clean up all update intervals
    for (const ws in this.updateIntervals) {
      for (const metricType in this.updateIntervals[ws]) {
        clearInterval(this.updateIntervals[ws][metricType]);
      }
    }
    this.updateIntervals = {};
    
    // Close all client connections
    this.clients.forEach(client => {
      client.terminate();
    });
    
    this.clients.clear();
    
    // Close the server
    this.wss.close();
  }
}

module.exports = WebSocketManager; 