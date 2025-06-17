/**
 * Port Manager Utility
 * 
 * This utility provides functions to interact with the NestGate port manager.
 */

import fs from 'fs';
import path from 'path';
import { ServiceType } from '../types/port-manager';

// Default ports if port manager is not available
const DEFAULT_PORTS = {
  [ServiceType.SERVER]: 3002,
  [ServiceType.API]: 3003,
  [ServiceType.UI]: 3000,
  [ServiceType.FILE_MONITOR]: 9500,
  [ServiceType.OTHER]: 8080,
};

/**
 * Gets the port for a specific service type
 * Will use the port manager if available, otherwise falls back to defaults
 */
export function getServicePort(serviceType: ServiceType): number {
  try {
    // Try to load the port manager
    const baseDir = process.cwd();
    const portManagerPaths = [
      path.join(baseDir, 'dist/crates/nestgate-port-manager/lib/ports.js'),
      path.join(baseDir, 'crates/nestgate-port-manager/lib/ports.js'),
      path.join(baseDir, '../crates/nestgate-port-manager/lib/ports.js'),
    ];
    
    // Find the first available port manager module
    for (const portManagerPath of portManagerPaths) {
      if (fs.existsSync(portManagerPath)) {
        try {
          const portManager = require(portManagerPath);
          if (portManager) {
            switch (serviceType) {
              case ServiceType.SERVER:
                return portManager.getServerPort() || DEFAULT_PORTS[ServiceType.SERVER];
              case ServiceType.API:
                return portManager.getApiPort() || DEFAULT_PORTS[ServiceType.API];
              case ServiceType.UI:
                return portManager.getUiPort() || DEFAULT_PORTS[ServiceType.UI];
              case ServiceType.FILE_MONITOR:
                return portManager.getFileMonitorPort() || DEFAULT_PORTS[ServiceType.FILE_MONITOR];
              default:
                return DEFAULT_PORTS[serviceType];
            }
          }
        } catch (innerErr) {
          console.warn(`Failed to use port manager at ${portManagerPath}:`, innerErr);
        }
      }
    }

    // If we reach here, port manager not found or failed
    console.warn(`Port manager not found or failed, using default port for ${serviceType}`);
    return DEFAULT_PORTS[serviceType];
  } catch (err) {
    console.warn('Error getting service port:', err);
    return DEFAULT_PORTS[serviceType];
  }
}

/**
 * Kill processes using a specific port
 */
export function killProcessOnPort(port: number): boolean {
  try {
    if (process.platform === 'win32') {
      const { execSync } = require('child_process');
      execSync(`for /f "tokens=5" %a in ('netstat -aon ^| find ":${port}" ^| find "LISTENING"') do taskkill /F /PID %a`, { stdio: 'ignore' });
      return true;
    } else {
      const { execSync } = require('child_process');
      execSync(`lsof -i:${port} -t | xargs kill -9 2>/dev/null || true`, { stdio: 'ignore' });
      return true;
    }
  } catch (err) {
    console.warn(`Failed to kill process on port ${port}:`, err);
    return false;
  }
}

/**
 * Check if a port is in use
 */
export async function isPortInUse(port: number): Promise<boolean> {
  try {
    const net = require('net');
    const server = net.createServer();
    
    return new Promise<boolean>((resolve) => {
      server.once('error', (err: Error & { code?: string }) => {
        if (err.code === 'EADDRINUSE') {
          resolve(true);
        } else {
          resolve(false);
        }
      });
      
      server.once('listening', () => {
        server.close();
        resolve(false);
      });
      
      server.listen(port);
    });
  } catch (err) {
    console.warn(`Failed to check if port ${port} is in use:`, err);
    return false;
  }
} 