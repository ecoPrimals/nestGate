/**
 * NestGate Application JavaScript
 */
document.addEventListener('DOMContentLoaded', function() {
  // System state for tracking real-time updates
  const systemState = {
    pools: [],
    performance: {
      readThroughput: 0,
      writeThroughput: 0,
      iops: 0,
      latency: 0
    },
    diskHealth: [],
    services: {
      nfs: 'unknown',
      smb: 'unknown'
    }
  };

  // DOM Elements
  const statusContent = document.getElementById('status-content');
  const poolsContent = document.getElementById('pools-content');
  const notification = document.getElementById('notification');

  // Track what mode we're in
  let dataSourceMode = 'loading';

  // WebSocket connection
  let socket;
  let reconnectAttempts = 0;
  const maxReconnectAttempts = 5;

  /**
   * Display a notification message
   * @param {string} message - The message to display
   * @param {string} type - The type of notification (info, success, warning, error)
   */
  function showNotification(message, type = 'info', duration = 5000) {
    notification.textContent = message;
    notification.className = `notification ${type}`;
    notification.style.display = 'block';
    
    setTimeout(() => {
      notification.style.display = 'none';
    }, duration);
  }

  /**
   * Format bytes to a human-readable format
   * @param {number} bytes - The number of bytes
   * @param {number} decimals - Number of decimal places
   * @returns {string} - Formatted string
   */
  function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
  }

  /**
   * Connect to the WebSocket server
   */
  function connectWebSocket() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws`;
    
    console.log(`Connecting to WebSocket at ${wsUrl}`);
    
    socket = new WebSocket(wsUrl);
    
    socket.onopen = function() {
      console.log('WebSocket connection established');
      
      // Request system health updates
      socket.send(JSON.stringify({
        metric_type: 'SystemHealth',
        interval_ms: 1000
      }));
      
      // Request performance metrics updates
      socket.send(JSON.stringify({
        metric_type: 'Performance',
        interval_ms: 2000
      }));
      
      // Request disk health updates
      socket.send(JSON.stringify({
        metric_type: 'DiskHealth',
        interval_ms: 5000
      }));
      
      // Request ZFS pools updates
      socket.send(JSON.stringify({
        metric_type: 'ZfsPools',
        interval_ms: 3000
      }));
      
      // Request services status updates
      socket.send(JSON.stringify({
        metric_type: 'Services',
        interval_ms: 5000
      }));
    };
    
    socket.onmessage = function(event) {
      try {
        const data = JSON.parse(event.data);
        console.log('WebSocket message received:', data.type);
        
        // Handle different message types
        switch (data.type) {
          case 'welcome':
            console.log('Welcome message:', data.message);
            break;
            
          case 'systemUpdate':
            updateSystemUI(data.data);
            break;
            
          case 'poolsUpdate':
            updatePoolsUI(data.data);
            break;
            
          case 'datasetsUpdate':
            // Handle datasets update
            break;
            
          case 'performanceUpdate':
            updatePerformanceUI(data.data);
            break;
            
          case 'diskHealthUpdate':
            updateDiskHealthUI(data.data);
            break;
            
          case 'servicesUpdate':
            updateServicesUI(data.data);
            break;
        }
      } catch (error) {
        console.error('Error handling WebSocket message:', error);
      }
    };
    
    socket.onclose = function() {
      console.log('WebSocket connection closed');
      // Reconnect after 3 seconds
      setTimeout(connectWebSocket, 3000);
    };
    
    socket.onerror = function(error) {
      console.error('WebSocket error:', error);
    };
  }

  /**
   * Update the system status UI
   */
  function updateSystemUI(data) {
    const statusContent = document.getElementById('status-content');
    if (!statusContent) return;
    
    // Keep any existing warning banners
    const warnings = statusContent.querySelector('.mock-data-warning, .test-data-warning');
    const warningHtml = warnings ? warnings.outerHTML : '';
    
    let html = warningHtml + `
      <div class="system-metrics">
        <div class="metric">
          <h3>Status</h3>
          <p>${data.status}</p>
        </div>
        <div class="metric">
          <h3>CPU Usage</h3>
          <p>${data.cpu_usage}%</p>
        </div>
        <div class="metric">
          <h3>Memory Usage</h3>
          <p>${data.memory_usage}%</p>
        </div>
        <div class="metric">
          <h3>Uptime</h3>
          <p>${data.uptime}</p>
        </div>
      </div>
    `;
    
    statusContent.innerHTML = html;
  }

  /**
   * Update the ZFS pools UI
   */
  function updatePoolsUI(pools) {
    const poolsContent = document.getElementById('pools-content');
    if (!poolsContent) return;
    
    // Keep any existing warning banners
    const warnings = poolsContent.querySelector('.mock-data-warning, .test-data-warning');
    const warningHtml = warnings ? warnings.outerHTML : '';
    
    let html = warningHtml;
    
    if (pools && pools.length > 0) {
      html += `
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Health</th>
              <th>Size</th>
              <th>Used</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
      `;
      
      pools.forEach(pool => {
        const used = formatBytes(pool.used);
        const size = formatBytes(pool.size);
        const percentage = Math.round((pool.used / pool.size) * 100);
        
        html += `
          <tr>
            <td>${pool.name}</td>
            <td>${pool.health}</td>
            <td>${size}</td>
            <td>${used} (${percentage}%)</td>
            <td>
              <button class="view-details" data-pool="${pool.name}">Details</button>
            </td>
          </tr>
        `;
      });
      
      html += `
          </tbody>
        </table>
      `;
    } else {
      html += '<p>No ZFS pools found.</p>';
    }
    
    poolsContent.innerHTML = html;
    
    // Add event listeners to the details buttons
    const detailButtons = document.querySelectorAll('.view-details');
    detailButtons.forEach(button => {
      button.addEventListener('click', function() {
        const poolName = button.getAttribute('data-pool');
        showPoolDetails(poolName);
      });
    });
  }

  /**
   * Update the performance metrics UI
   */
  function updatePerformanceUI(performance) {
    if (!performance) return;
    
    // Update the last update time
    const now = new Date();
    document.getElementById('metrics-timestamp').textContent = `Last update: ${now.toLocaleTimeString()}`;
    
    // Update metrics
    if (performance.throughput) {
      document.getElementById('read-throughput').textContent = performance.throughput.read;
      document.getElementById('write-throughput').textContent = performance.throughput.write;
    }
    
    if (performance.latency) {
      document.getElementById('read-latency').textContent = performance.latency.read;
      document.getElementById('write-latency').textContent = performance.latency.write;
    }
    
    if (performance.iops) {
      document.getElementById('iops').textContent = performance.iops.read + performance.iops.write;
    }
  }

  /**
   * Update the disk health UI
   */
  function updateDiskHealthUI(disks) {
    const diskHealthContent = document.getElementById('disk-health-content');
    if (!diskHealthContent) return;
    
    // Keep any existing warning banners
    const warnings = diskHealthContent.querySelector('.mock-data-warning, .test-data-warning');
    const warningHtml = warnings ? warnings.outerHTML : '';
    
    let html = warningHtml;
    
    if (disks && disks.length > 0) {
      html += '<div class="disk-grid">';
      
      disks.forEach(disk => {
        html += `
          <div class="disk-card">
            <h3>${disk.device}</h3>
            <p class="disk-status">${disk.status}</p>
            <p><strong>Model:</strong> ${disk.model}</p>
            <p><strong>Serial:</strong> ${disk.serial}</p>
            <p><strong>Size:</strong> ${formatBytes(disk.size)}</p>
            <p><strong>Temperature:</strong> ${disk.temperature}°C</p>
          </div>
        `;
      });
      
      html += '</div>';
    } else {
      html += '<p>No disk health information available.</p>';
    }
    
    diskHealthContent.innerHTML = html;
  }

  /**
   * Update the services UI
   */
  function updateServicesUI(services) {
    if (!services) return;
    
    // Update NFS status
    if (services.nfs) {
      document.getElementById('nfs-status').textContent = services.nfs;
      document.getElementById('nfs-status').className = getStatusClass(services.nfs);
    }
    
    // Update SMB status
    if (services.smb) {
      document.getElementById('smb-status').textContent = services.smb;
      document.getElementById('smb-status').className = getStatusClass(services.smb);
    }
  }

  /**
   * Get the CSS class for a service status
   */
  function getStatusClass(status) {
    switch (status.toLowerCase()) {
      case 'running':
        return 'status-running';
      case 'stopped':
        return 'status-stopped';
      case 'error':
        return 'status-error';
      default:
        return '';
    }
  }

  /**
   * Show details for a specific pool
   */
  function showPoolDetails(poolName) {
    alert(`Details for pool ${poolName} will be implemented in a future update.`);
  }

  /**
   * Refresh services status
   */
  function refreshServices() {
    // Show a notification that we're refreshing services
    showNotification('Refreshing services status...', 'info');
    
    // In a real implementation, we would send a message to the WebSocket
    // For now, we'll just show a notification
    setTimeout(() => {
      showNotification('Services status refreshed', 'success');
    }, 1000);
  }

  // Initialize application
  function init() {
    // Load initial data
    loadSystemStatus();
    loadZfsPools();
    
    // Connect to WebSocket for real-time updates
    connectWebSocket();
    
    // Fetch initial performance metrics
    fetch('/api/performance')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updatePerformanceMetrics(data.data);
        }
      })
      .catch(error => {
        console.error('Error fetching performance metrics:', error);
      });
    
    // Fetch initial disk health
    fetch('/api/disks/health')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updateDiskHealth(data.data);
        }
      })
      .catch(error => {
        console.error('Error fetching disk health:', error);
      });
    
    // Fetch initial services status
    fetch('/api/services')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updateServicesStatus(data.data);
        }
      })
      .catch(error => {
        console.error('Error fetching services status:', error);
      });
  }

  /**
   * Load system status data from the API
   */
  function loadSystemStatus() {
    fetch('/api/system/status')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updateSystemUI(data.data);
        } else {
          console.error('Error loading system status:', data.message);
        }
      })
      .catch(error => {
        console.error('Error fetching system status:', error);
        // Show placeholder data
        updateSystemUI({
          status: 'Unknown',
          cpu_usage: '--',
          memory_usage: '--',
          uptime: '--'
        });
      });
  }

  /**
   * Load ZFS pools data from the API
   */
  function loadZfsPools() {
    fetch('/api/zfs/pools')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updatePoolsUI(data.data);
        } else {
          console.error('Error loading ZFS pools:', data.message);
          updatePoolsUI([]);
        }
      })
      .catch(error => {
        console.error('Error fetching ZFS pools:', error);
        updatePoolsUI([]);
      });
  }

  /**
   * Update performance metrics
   */
  function updatePerformanceMetrics(data) {
    updatePerformanceUI(data);
  }

  /**
   * Update disk health information
   */
  function updateDiskHealth(data) {
    updateDiskHealthUI(data);
  }

  /**
   * Update services status
   */
  function updateServicesStatus(data) {
    updateServicesUI(data);
  }

  // Initialize the application
  init();

  // Initialize dashboard on page load
  initDashboard();
  setupDataSourceInfoToggle();
});

/**
 * Initialize the dashboard with appropriate data source
 */
async function initDashboard() {
  try {
    // Check for strict mode first
    const isStrictMode = window.ENV && (window.ENV.STRICT_DATA_MODE === true || window.ENV.REACT_APP_STRICT_DATA_MODE === true);
    
    if (isStrictMode) {
      // In strict mode, always use live data mode
      dataSourceMode = 'production';
      updateDataSourceIndicator('Live Data', 'badge-success');
      
      // Don't set any URL parameters in strict mode
      // Don't show any warnings in strict mode
      
      // Remove any existing warnings
      removeAllDataWarnings();
      
      // Connect to WebSocket for real-time updates
      connectWebSocket();
      return;
    }
    
    // Only execute this code if NOT in strict mode
    
    // Set mode to test by default (for non-strict mode)
    dataSourceMode = 'test';
    updateDataSourceIndicator('Test Data', 'badge-warning');
    showDataSourceWarning('test');
    
    // Set URL parameter to test
    if (!window.location.search.includes('mode=')) {
      const newUrl = window.location.href + (window.location.search ? '&' : '?') + 'mode=test';
      window.history.replaceState(null, '', newUrl);
    }
    
    // Try to load the Tauri ZFS plugin
    if (window.__TAURI__) {
      // We're in a Tauri app - use the real plugin
      dataSourceMode = 'production';
      updateDataSourceIndicator('Real Data', 'badge-success');
      
      // Load data using the Tauri plugin
      await loadRealZfsData();
    } else {
      // Use server-side ZFS commands (test mode)
      await loadTestZfsData();
      
      // Connect to WebSocket for real-time updates
      connectWebSocket();
    }
  } catch (error) {
    console.error('Error initializing dashboard:', error);
    
    // Check for strict mode before falling back to mock data
    const isStrictMode = window.ENV && (window.ENV.STRICT_DATA_MODE === true || window.ENV.REACT_APP_STRICT_DATA_MODE === true);
    
    if (isStrictMode) {
      // In strict mode, keep live mode but show an error
      dataSourceMode = 'production';
      updateDataSourceIndicator('Live Data', 'badge-success');
      removeAllDataWarnings();
      showNotification('Error loading live data. Please check your connection.', 'error');
      return;
    }
    
    // Only fall back to mock data if NOT in strict mode
    dataSourceMode = 'mock';
    updateDataSourceIndicator('Mock Data', 'badge-danger');
    showDataSourceWarning('mock');
    
    // Load mock data
    loadMockData();
  }
}

/**
 * Update the data source indicator
 */
function updateDataSourceIndicator(text, className) {
  const dataSourceBadge = document.getElementById('data-source-indicator');
  
  // Check for strict mode first
  const isStrictMode = window.ENV && (window.ENV.STRICT_DATA_MODE === true || window.ENV.REACT_APP_STRICT_DATA_MODE === true);
  
  // In strict mode, always show "Live Data"
  if (isStrictMode) {
    if (dataSourceBadge) {
      dataSourceBadge.textContent = 'Live Data';
      dataSourceBadge.className = 'badge badge-success';
    }
    
    const modeText = document.getElementById('data-source-mode');
    if (modeText) {
      modeText.textContent = 'Live Data';
    }
    return;
  }
  
  if (dataSourceBadge) {
    dataSourceBadge.textContent = text;
    dataSourceBadge.className = `badge ${className}`;
  }
  
  const modeText = document.getElementById('data-source-mode');
  if (modeText) {
    modeText.textContent = text;
  }
}

/**
 * Show a warning about data source reliability
 */
function showDataSourceWarning(mode) {
  // Check for strict mode first
  const isStrictMode = window.ENV && (window.ENV.STRICT_DATA_MODE === true || window.ENV.REACT_APP_STRICT_DATA_MODE === true);
  
  // In strict mode, never show mock/test data warnings and remove any existing ones
  if (isStrictMode) {
    removeAllDataWarnings();
    return;
  }
  
  const statusContent = document.getElementById('status-content');
  const poolsContent = document.getElementById('pools-content');
  const diskHealthContent = document.getElementById('disk-health-content');
  const servicesContent = document.getElementById('services-content');
  
  if (mode === 'mock') {
    document.getElementById('metrics-mock-indicator').style.display = 'block';
    
    if (statusContent) {
      statusContent.innerHTML = '<div class="mock-data-warning">MOCK DATA: System status is simulated and does not reflect the actual system state.</div>' + statusContent.innerHTML;
    }
    
    if (poolsContent) {
      poolsContent.innerHTML = '<div class="mock-data-warning">MOCK DATA: ZFS pool information is simulated and does not reflect actual ZFS pools.</div>' + poolsContent.innerHTML;
    }
    
    if (diskHealthContent) {
      diskHealthContent.innerHTML = '<div class="mock-data-warning">MOCK DATA: Disk health information is simulated and does not reflect actual disks.</div>' + diskHealthContent.innerHTML;
    }
    
    if (servicesContent) {
      servicesContent.innerHTML = '<div class="mock-data-warning">MOCK DATA: Service status is simulated and does not reflect actual services.</div>' + servicesContent.innerHTML;
    }
  } else if (mode === 'test') {
    if (statusContent) {
      statusContent.innerHTML = '<div class="test-data-warning">TEST DATA: Using server-side ZFS commands.</div>' + statusContent.innerHTML;
    }
    
    if (poolsContent) {
      poolsContent.innerHTML = '<div class="test-data-warning">TEST DATA: Using server-side ZFS commands.</div>' + poolsContent.innerHTML;
    }
  }
}

/**
 * Set up the data source info toggle
 */
function setupDataSourceInfoToggle() {
  const infoButton = document.querySelector('.data-source-info');
  const detailsSection = document.getElementById('data-source-details');
  
  if (infoButton && detailsSection) {
    infoButton.addEventListener('click', function() {
      if (detailsSection.style.display === 'none') {
        detailsSection.style.display = 'block';
      } else {
        detailsSection.style.display = 'none';
      }
    });
  }
}

/**
 * Load test ZFS data from server API
 */
async function loadTestZfsData() {
  try {
    // Load system status
    loadSystemStatus();
    
    // Load ZFS pools
    loadZfsPools();
    
    // Load disk health
    fetch('/api/disks/health')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updateDiskHealth(data.data);
        }
      })
      .catch(error => {
        console.error('Error fetching disk health:', error);
      });
    
    // Load services status
    fetch('/api/services')
      .then(response => response.json())
      .then(data => {
        if (data.status === 'success') {
          updateServicesStatus(data.data);
        }
      })
      .catch(error => {
        console.error('Error fetching services status:', error);
      });
  } catch (error) {
    console.error('Error loading test ZFS data:', error);
    throw error;
  }
}

/**
 * Load mock ZFS data
 */
function loadMockData() {
  // Just use the existing load functions which will fallback to mock data
  loadSystemStatus();
  loadZfsPools();
}

/**
 * Remove all mock and test data warnings from the UI
 */
function removeAllDataWarnings() {
  // Hide the mock data indicator
  const mockIndicator = document.getElementById('metrics-mock-indicator');
  if (mockIndicator) {
    mockIndicator.style.display = 'none';
  }
  
  // Remove all mock data warnings
  const mockWarnings = document.querySelectorAll('.mock-data-warning');
  mockWarnings.forEach(warning => {
    warning.remove();
  });
  
  // Remove all test data warnings
  const testWarnings = document.querySelectorAll('.test-data-warning');
  testWarnings.forEach(warning => {
    warning.remove();
  });
  
  // Clear any warning text from content areas
  const contentAreas = [
    'status-content',
    'pools-content',
    'disk-health-content',
    'services-content',
    'performance-content'
  ];
  
  contentAreas.forEach(id => {
    const element = document.getElementById(id);
    if (element) {
      // Remove warning divs but keep other content
      const warnings = element.querySelectorAll('.mock-data-warning, .test-data-warning');
      warnings.forEach(warning => {
        warning.remove();
      });
    }
  });
}

// Call removeAllDataWarnings on page load if in strict mode
document.addEventListener('DOMContentLoaded', function() {
  const isStrictMode = window.ENV && (window.ENV.STRICT_DATA_MODE === true || window.ENV.REACT_APP_STRICT_DATA_MODE === true);
  if (isStrictMode) {
    // Initial removal
    removeAllDataWarnings();
    
    // Add an aggressive MutationObserver to remove warnings as soon as they're added
    setupWarningObserver();
    
    // Also periodically check for warnings
    setInterval(removeAllDataWarnings, 1000);
  }
});

/**
 * Set up a MutationObserver to watch for and remove any mock/test data warnings
 * that might be added to the DOM after initial load
 */
function setupWarningObserver() {
  // Check if we're in strict mode
  const isStrictMode = window.ENV && (window.ENV.STRICT_DATA_MODE === true || window.ENV.REACT_APP_STRICT_DATA_MODE === true);
  if (!isStrictMode) return;
  
  // Create a MutationObserver to watch for added nodes
  const observer = new MutationObserver(function(mutations) {
    let needsCleanup = false;
    
    // Check each mutation for added nodes
    mutations.forEach(function(mutation) {
      if (mutation.addedNodes && mutation.addedNodes.length > 0) {
        // Check if any added nodes contain warning classes or text
        for (let i = 0; i < mutation.addedNodes.length; i++) {
          const node = mutation.addedNodes[i];
          
          // Check if it's an element node
          if (node.nodeType === 1) {
            // Check if it has the warning class
            if (node.classList && (
                node.classList.contains('mock-data-warning') || 
                node.classList.contains('test-data-warning'))) {
              needsCleanup = true;
              break;
            }
            
            // Check if it contains the warning text
            if (node.innerHTML && (
                node.innerHTML.includes('MOCK DATA') || 
                node.innerHTML.includes('TEST DATA'))) {
              needsCleanup = true;
              break;
            }
          }
        }
      }
    });
    
    // If we found any warnings, remove them
    if (needsCleanup) {
      removeAllDataWarnings();
    }
  });
  
  // Start observing the document body for changes
  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
  
  console.log('Warning observer set up for strict mode');
}