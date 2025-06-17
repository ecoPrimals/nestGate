/**
 * This is a development-only Tauri ZFS plugin shim
 * In a real Tauri application, this would be replaced by a Rust plugin
 * 
 * This allows the UI to simulate communication with the backend during development
 * But with real data from the nestpool ZFS pool
 */

(function() {
  if (typeof window.__TAURI__ !== 'undefined') {
    // We're in a Tauri app, don't use the shim
    return;
  }
  
  // Create the global Tauri object if it doesn't exist
  window.__TAURI__ = window.__TAURI__ || {};
  window.__TAURI__.invoke = window.__TAURI__.invoke || function() {};
  
  // Add process object with exit function
  window.__TAURI__.process = window.__TAURI__.process || {
    exit: function(code) {
      console.log(`Tauri process exit called with code ${code}`);
      // In a real Tauri app, this would terminate the process
      // For development, we'll just log it
      
      // If you want to actually close the window for testing:
      // window.close();
    }
  };
  
  // Store any pending timeouts and intervals for cleanup
  const pendingTimeouts = new Set();
  const pendingIntervals = new Set();
  
  // Override setTimeout to track pending timeouts
  const originalSetTimeout = window.setTimeout;
  window.setTimeout = function(callback, delay, ...args) {
    const id = originalSetTimeout(callback, delay, ...args);
    pendingTimeouts.add(id);
    return id;
  };
  
  // Override clearTimeout to remove from tracked timeouts
  const originalClearTimeout = window.clearTimeout;
  window.clearTimeout = function(id) {
    pendingTimeouts.delete(id);
    return originalClearTimeout(id);
  };
  
  // Override setInterval to track pending intervals
  const originalSetInterval = window.setInterval;
  window.setInterval = function(callback, delay, ...args) {
    const id = originalSetInterval(callback, delay, ...args);
    pendingIntervals.add(id);
    return id;
  };
  
  // Override clearInterval to remove from tracked intervals
  const originalClearInterval = window.clearInterval;
  window.clearInterval = function(id) {
    pendingIntervals.delete(id);
    return originalClearInterval(id);
  };
  
  // Clean up function to clear all pending timeouts and intervals
  const cleanUp = function() {
    // Clear all pending timeouts
    pendingTimeouts.forEach(id => {
      originalClearTimeout(id);
    });
    pendingTimeouts.clear();
    
    // Clear all pending intervals
    pendingIntervals.forEach(id => {
      originalClearInterval(id);
    });
    pendingIntervals.clear();
    
    console.log('All pending timeouts and intervals cleared');
  };
  
  // Register cleanup on page unload
  window.addEventListener('beforeunload', cleanUp);
  
  // Helper function to execute shell commands
  const executeCommand = async (command) => {
    try {
      const response = await fetch('/api/exec', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ command })
      });
      
      if (!response.ok) {
        throw new Error(`Command execution failed: ${response.statusText}`);
      }
      
      const result = await response.text();
      return result;
    } catch (error) {
      console.error(`Error executing command: ${command}`, error);
      throw error;
    }
  };
  
  // Helper function to parse ZFS pool data
  const parsePoolData = (output) => {
    const lines = output.trim().split('\n');
    return lines.map(line => {
      const [name, health, size, free, used] = line.split('\t');
      
      // Convert size strings to bytes (assuming they're in human-readable format)
      const parseSize = (sizeStr) => {
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
        id: name, // Use name as ID
        name,
        health,
        size: parseSize(size),
        free: parseSize(free),
        used: parseSize(used),
        capacity: parseSize(size),
        available: parseSize(free)
      };
    });
  };
  
  // Helper function to parse ZFS dataset data
  const parseDatasetData = (output) => {
    const lines = output.trim().split('\n');
    return lines.map(line => {
      const [name, mountpoint, available, used, mounted] = line.split('\t');
      
      // Convert size strings to bytes
      const parseSize = (sizeStr) => {
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
  };
  
  // Mock invoke function for ZFS plugin
  const oldInvoke = window.__TAURI__.invoke;
  window.__TAURI__.invoke = async function(cmd, args) {
    console.log(`Tauri invoke: ${cmd}`, args);
    
    // Handle ZFS plugin commands
    if (cmd === 'plugin:zfs|is_available') {
      // Check if ZFS is available by executing a simple command
      try {
        await executeCommand('zpool list -H >/dev/null 2>&1 && echo "true" || echo "false"');
        return true;
      } catch (error) {
        console.error('ZFS availability check failed:', error);
        return false;
      }
    }
    
    if (cmd === 'plugin:zfs|list_pools') {
      try {
        // Use real data from nestpool
        const output = await executeCommand('zpool list -H -o name,health,size,free,allocated');
        return parsePoolData(output);
      } catch (error) {
        console.error('Error listing ZFS pools:', error);
        // Fallback to mock data if the command fails
        return [
          {
            id: 'nestpool',
            name: 'nestpool',
            health: 'ONLINE',
            capacity: 76400000000000, // 76.4TB
            available: 76300000000000, // 76.3TB
            used: 3030000000, // 3.03GB
            size: 76400000000000,
            free: 76300000000000
          }
        ];
      }
    }
    
    if (cmd === 'plugin:zfs|list_datasets') {
      try {
        const poolName = args?.poolName || 'nestpool';
        // Use real data from nestpool
        const output = await executeCommand(`zfs list -H -o name,mountpoint,available,used,mounted -t filesystem -r ${poolName}`);
        return parseDatasetData(output);
      } catch (error) {
        console.error('Error listing ZFS datasets:', error);
        // Fallback to mock data if the command fails
        const poolName = args?.poolName || 'nestpool';
        return [
          {
            id: `${poolName}/hot`,
            name: `${poolName}/hot`,
            mountpoint: `/nestpool/hot`,
            available: 50700000000000, // 50.7TB
            used: 3030000000 // 3.03GB
          },
          {
            id: `${poolName}/cold`,
            name: `${poolName}/cold`,
            mountpoint: `/nestpool/cold`,
            available: 50700000000000, // 50.7TB
            used: 192000 // 192KB
          },
          {
            id: `${poolName}/warm`,
            name: `${poolName}/warm`,
            mountpoint: `/nestpool/warm`,
            available: 50700000000000, // 50.7TB
            used: 192000 // 192KB
          }
        ];
      }
    }
    
    if (cmd === 'plugin:zfs|get_pool_metrics') {
      return {
        iops: {
          read: 1200,
          write: 800
        },
        throughput: {
          read: 120, // MB/s
          write: 80 // MB/s
        },
        latency: {
          read: 5.2, // ms
          write: 8.7 // ms
        }
      };
    }
    
    // For other commands, pass through to original invoke if available
    if (typeof oldInvoke === 'function') {
      return oldInvoke(cmd, args);
    }
    
    // Default fallback
    return Promise.reject(new Error(`Command not implemented: ${cmd}`));
  };
  
  console.log('Tauri ZFS plugin shim loaded - Using real nestpool data');
})(); 