/**
 * NestGate Dashboard JavaScript
 * Handles both real ZFS data and mock data with clear indicators
 */

// Track what mode we're in
let dataSourceMode = 'loading';

// WebSocket connection
let socket;
let reconnectAttempts = 0;
const maxReconnectAttempts = 5;

// Initialize dashboard on page load
document.addEventListener('DOMContentLoaded', function() {
  initDashboard();
});

/**
 * Initialize the dashboard with appropriate data source
 */
async function initDashboard() {
  try {
    // Try to load the Tauri ZFS plugin
    if (window.__TAURI__) {
      // We're in a Tauri app - use the real plugin
      dataSourceMode = 'production';
      document.getElementById('data-source-indicator').textContent = 'Real Data';
      document.getElementById('data-source-indicator').className = 'badge badge-success';
      
      // Load data using the Tauri plugin
      await loadRealZfsData();
    } else {
      // Try server-side ZFS commands
      try {
        const isAvailable = await checkZfsAvailability();
        if (isAvailable) {
          // We have ZFS available through server API
          dataSourceMode = 'test';
          document.getElementById('data-source-indicator').textContent = 'Test Data';
          document.getElementById('data-source-indicator').className = 'badge badge-warning';
          
          // Load data using the server API
          await loadTestZfsData();
          
          // Connect to WebSocket for real-time updates
          connectWebSocket();
        } else {
          // Fall back to mock data
          dataSourceMode = 'mock';
          document.getElementById('data-source-indicator').textContent = 'Mock Data';
          document.getElementById('data-source-indicator').className = 'badge badge-danger';
          
          // Load mock data
          loadMockData();
        }
      } catch (error) {
        console.error('Error checking ZFS availability:', error);
        // Fall back to mock data
        dataSourceMode = 'mock';
        document.getElementById('data-source-indicator').textContent = 'Mock Data';
        document.getElementById('data-source-indicator').className = 'badge badge-danger';
        
        // Load mock data
        loadMockData();
      }
    }
  } catch (error) {
    console.error('Error initializing dashboard:', error);
    // Fall back to mock data in case of any errors
    dataSourceMode = 'mock';
    document.getElementById('data-source-indicator').textContent = 'Mock Data';
    document.getElementById('data-source-indicator').className = 'badge badge-danger';
    
    // Load mock data
    loadMockData();
  }
}

/**
 * Connect to WebSocket for real-time updates
 */
function connectWebSocket() {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const wsUrl = `${protocol}//${window.location.host}/ws`;
  
  console.log(`Connecting to WebSocket at ${wsUrl}`);
  
  socket = new WebSocket(wsUrl);
  
  socket.onopen = function() {
    console.log('WebSocket connection established');
    showNotification('Connected to real-time updates', 'success');
    reconnectAttempts = 0;
    
    // Set a periodic ping to keep connection alive
    setInterval(() => {
      if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify({ type: 'ping', timestamp: Date.now() }));
      }
    }, 30000);
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
          
        case 'pong':
          console.log('Pong received');
          break;
          
        case 'systemUpdate':
          updateSystemUI(data.data);
          break;
          
        case 'poolsUpdate':
          updatePoolsUI(data.data);
          break;
          
        case 'datasetsUpdate':
          updateDatasetsUI(data.poolName, data.data);
          break;
          
        case 'performanceUpdate':
          updateMetricsUI(data.data);
          break;
          
        case 'diskHealthUpdate':
          updateDiskHealthUI(data.data);
          break;
          
        case 'servicesUpdate':
          updateServicesUI(data.data);
          break;
          
        default:
          console.log('Unknown message type:', data.type);
      }
    } catch (error) {
      console.error('Error handling WebSocket message:', error);
    }
  };
  
  socket.onclose = function(event) {
    console.log('WebSocket connection closed:', event.code, event.reason);
    
    if (reconnectAttempts < maxReconnectAttempts) {
      reconnectAttempts++;
      const timeout = Math.min(1000 * reconnectAttempts, 10000);
      
      showNotification(`Connection lost. Reconnecting (${reconnectAttempts}/${maxReconnectAttempts})...`, 'warning');
      
      setTimeout(connectWebSocket, timeout);
    } else {
      showNotification('Could not reconnect to server. Please refresh the page.', 'error');
    }
  };
  
  socket.onerror = function(error) {
    console.error('WebSocket error:', error);
  };
}

/**
 * Check if ZFS is available through the server API
 */
async function checkZfsAvailability() {
  try {
    const response = await fetch('/api/zfs/available');
    if (!response.ok) {
      throw new Error(`Failed to check ZFS availability: ${response.status}`);
    }
    const data = await response.json();
    return data.available === true;
  } catch (error) {
    console.error('Error checking ZFS availability:', error);
    return false;
  }
}

/**
 * Load real ZFS data using the Tauri plugin
 */
async function loadRealZfsData() {
  try {
    // Try to get ZFS pools
    const isAvailable = await window.__TAURI__.invoke('plugin:zfs|is_available');
    if (!isAvailable) {
      throw new Error('ZFS is not available');
    }
    
    // Load pools
    const pools = await window.__TAURI__.invoke('plugin:zfs|list_pools');
    updatePoolsUI(pools);
    
    // Load datasets for each pool
    for (const pool of pools) {
      const datasets = await window.__TAURI__.invoke('plugin:zfs|list_datasets', { poolName: pool.name });
      updateDatasetsUI(pool.name, datasets);
    }
    
    // Load performance metrics
    const metrics = await window.__TAURI__.invoke('plugin:zfs|get_pool_metrics');
    updateMetricsUI(metrics);
    
    console.log('Loaded real ZFS data via Tauri plugin');
    
    // Set up periodic refresh for real data
    setInterval(async () => {
      try {
        const pools = await window.__TAURI__.invoke('plugin:zfs|list_pools');
        updatePoolsUI(pools);
        
        const metrics = await window.__TAURI__.invoke('plugin:zfs|get_pool_metrics');
        updateMetricsUI(metrics);
      } catch (error) {
        console.error('Error refreshing Tauri data:', error);
      }
    }, 10000);
  } catch (error) {
    console.error('Error loading real ZFS data:', error);
    showError('Failed to load real ZFS data. Falling back to mock data.');
    
    // Fall back to mock data
    dataSourceMode = 'mock';
    document.getElementById('data-source-indicator').textContent = 'Mock Data';
    document.getElementById('data-source-indicator').className = 'badge badge-danger';
    loadMockData();
  }
}

/**
 * Load test ZFS data using the server API
 */
async function loadTestZfsData() {
  try {
    // Load pools
    const poolsResponse = await fetch('/api/zfs/pools');
    if (!poolsResponse.ok) {
      throw new Error(`Failed to load pools: ${poolsResponse.status}`);
    }
    const poolsData = await poolsResponse.json();
    updatePoolsUI(poolsData.pools);
    
    // Load datasets
    for (const pool of poolsData.pools) {
      const datasetsResponse = await fetch(`/api/zfs/pools/${pool.name}/datasets`);
      if (!datasetsResponse.ok) {
        console.error(`Failed to load datasets for pool ${pool.name}`);
        continue;
      }
      const datasetsData = await datasetsResponse.json();
      updateDatasetsUI(pool.name, datasetsData.datasets);
    }
    
    // Load metrics
    const metricsResponse = await fetch('/api/zfs/metrics');
    if (!metricsResponse.ok) {
      console.error('Failed to load metrics');
    } else {
      const metricsData = await metricsResponse.json();
      updateMetricsUI(metricsData.metrics);
    }
    
    console.log('Loaded test ZFS data via server API');
  } catch (error) {
    console.error('Error loading test ZFS data:', error);
    showError('Failed to load test ZFS data. Falling back to mock data.');
    
    // Fall back to mock data
    dataSourceMode = 'mock';
    document.getElementById('data-source-indicator').textContent = 'Mock Data';
    document.getElementById('data-source-indicator').className = 'badge badge-danger';
    loadMockData();
  }
}

/**
 * Load mock ZFS data
 */
function loadMockData() {
  console.log('Loading mock ZFS data');
  
  // Mock pools
  const mockPools = [
    {
      id: 'nestpool',
      name: 'nestpool',
      health: 'ONLINE',
      size: 7810371559424, // 7.28 TB
      free: 5857778669568, // 5.46 TB
      used: 1952592889856, // 1.82 TB
    },
    {
      id: 'backup',
      name: 'backup',
      health: 'ONLINE',
      size: 3905185779712, // 3.64 TB
      free: 3398711132364, // 3.17 TB
      used: 506474647348, // 471.7 GB
    }
  ];
  updatePoolsUI(mockPools);
  
  // Mock datasets
  const mockDatasets = {
    'nestpool': [
      {
        id: 'nestpool/hot',
        name: 'nestpool/hot',
        mountpoint: '/nestpool/hot',
        available: 5857778669568, // 5.46 TB
        used: 1952592889856, // 1.82 TB
        mounted: true
      },
      {
        id: 'nestpool/warm',
        name: 'nestpool/warm',
        mountpoint: '/nestpool/warm',
        available: 5857778669568, // 5.46 TB
        used: 0,
        mounted: true
      },
      {
        id: 'nestpool/cold',
        name: 'nestpool/cold',
        mountpoint: '/nestpool/cold',
        available: 5857778669568, // 5.46 TB
        used: 0,
        mounted: true
      }
    ],
    'backup': [
      {
        id: 'backup/data',
        name: 'backup/data',
        mountpoint: '/backup/data',
        available: 3398711132364, // 3.17 TB
        used: 506474647348, // 471.7 GB
        mounted: true
      }
    ]
  };
  
  for (const [poolName, datasets] of Object.entries(mockDatasets)) {
    updateDatasetsUI(poolName, datasets);
  }
  
  // Mock metrics
  const mockMetrics = {
    iops: {
      read: 1200,
      write: 800
    },
    throughput: {
      read: 120, // MB/s
      write: 85 // MB/s
    },
    latency: {
      read: 5.2, // ms
      write: 8.7 // ms
    }
  };
  updateMetricsUI(mockMetrics);
  
  // Set up mock data refresh for demo purposes
  setInterval(() => {
    // Create some variation in the mock data
    const newMockMetrics = {
      iops: {
        read: 1000 + Math.round(Math.random() * 400),
        write: 700 + Math.round(Math.random() * 200)
      },
      throughput: {
        read: 100 + Math.round(Math.random() * 40),
        write: 75 + Math.round(Math.random() * 20)
      },
      latency: {
        read: 4 + Math.random() * 2,
        write: 7 + Math.random() * 3
      }
    };
    updateMetricsUI(newMockMetrics);
  }, 5000);
}

/**
 * Update the system UI with new data
 */
function updateSystemUI(data) {
  if (!data) return;
  
  const systemHealthSection = document.getElementById('system-health');
  if (!systemHealthSection) return;
  
  let html = '';
  
  if (dataSourceMode === 'mock') {
    html += '<div class="mock-data-warning">MOCK DATA</div>';
  }
  
  html += `
    <h2>System Health</h2>
    <p>${data.status}</p>
    <div class="system-metrics">
      <div class="metric">
        <h3>CPU Usage</h3>
        <p>${data.cpu_usage}%</p>
      </div>
      <div class="metric">
        <h3>Memory Usage</h3>
        <p>${data.memory_usage}%</p>
      </div>
      <div class="metric">
        <h3>Disk Health</h3>
        <p>optimal</p>
      </div>
    </div>
  `;
  
  systemHealthSection.innerHTML = html;
}

/**
 * Update the pools UI with the provided data
 */
function updatePoolsUI(pools) {
  const poolsContent = document.getElementById('pools-content');
  if (!poolsContent) return;
  
  let totalCapacity = 0;
  let totalUsed = 0;
  
  let poolsHtml = '';
  
  // If we have mock data, show a warning
  if (dataSourceMode === 'mock') {
    poolsHtml += '<div class="mock-data-warning">MOCK DATA</div>';
  }
  
  if (pools.length === 0) {
    poolsHtml = '<p>No ZFS pools found.</p>';
  } else {
    // Calculate total capacity and used space
    for (const pool of pools) {
      totalCapacity += pool.size;
      totalUsed += pool.used;
    }
    
    // Update storage usage section
    const usedPercentage = Math.round((totalUsed / totalCapacity) * 100);
    document.getElementById('storage-usage-percentage').textContent = `${usedPercentage}%`;
    document.getElementById('storage-usage-bar').style.width = `${usedPercentage}%`;
    document.getElementById('total-capacity').textContent = formatBytes(totalCapacity);
    document.getElementById('used-space').textContent = formatBytes(totalUsed);
    document.getElementById('free-space').textContent = formatBytes(totalCapacity - totalUsed);
    
    // Create the pools list
    poolsHtml = '<div class="pools-grid">';
    
    pools.forEach(pool => {
      const usedPercentage = Math.round((pool.used / pool.size) * 100);
      
      poolsHtml += `
        <div class="pool-card">
          <div class="pool-header">
            <h3>${pool.name}</h3>
            <span class="pool-health">${pool.health}</span>
            ${dataSourceMode === 'mock' ? '<span class="mock-badge">MOCK</span>' : ''}
          </div>
          <div class="pool-details">
            <p>${formatBytes(pool.size)}</p>
            <div class="progress-bar">
              <div class="progress-value" style="width: ${usedPercentage}%"></div>
            </div>
            <p>${usedPercentage}%</p>
          </div>
          <div class="pool-actions">
            <button class="button view-datasets" data-pool="${pool.name}">Datasets</button>
            <button class="button view-snapshots" data-pool="${pool.name}">Snapshots</button>
          </div>
        </div>
      `;
    });
    
    poolsHtml += '</div>';
  }
  
  poolsContent.innerHTML = poolsHtml;
  
  // Add event listeners
  document.querySelectorAll('.view-datasets').forEach(btn => {
    btn.addEventListener('click', () => {
      const pool = btn.dataset.pool;
      showDatasets(pool);
    });
  });
  
  document.querySelectorAll('.view-snapshots').forEach(btn => {
    btn.addEventListener('click', () => {
      const pool = btn.dataset.pool;
      showSnapshots(pool);
    });
  });
}

/**
 * Update the datasets UI for a specific pool
 */
function updateDatasetsUI(poolName, datasets) {
  // This function would update a datasets section if visible
  // For now, we'll just store the datasets for later use
  window.datasetsCache = window.datasetsCache || {};
  window.datasetsCache[poolName] = datasets;
}

/**
 * Update the disk health UI
 */
function updateDiskHealthUI(disks) {
  // Not implemented yet
  console.log('Disk health update received:', disks);
}

/**
 * Update the services UI
 */
function updateServicesUI(services) {
  // Not implemented yet
  console.log('Services update received:', services);
}

/**
 * Update the metrics UI with the provided data
 */
function updateMetricsUI(metrics) {
  // If we have mock data, show a warning
  if (dataSourceMode === 'mock') {
    document.getElementById('metrics-mock-indicator').style.display = 'block';
  } else {
    document.getElementById('metrics-mock-indicator').style.display = 'none';
  }
  
  // Add a timestamp indicator to show real-time updates
  const now = new Date();
  const timeString = now.toLocaleTimeString();
  document.getElementById('metrics-timestamp').textContent = `Last update: ${timeString}`;
  
  // Update the metrics display
  if (metrics.throughput) {
    document.getElementById('read-throughput').textContent = metrics.throughput.read;
    document.getElementById('write-throughput').textContent = metrics.throughput.write;
  }
  
  if (metrics.latency) {
    document.getElementById('read-latency').textContent = metrics.latency.read;
    document.getElementById('write-latency').textContent = metrics.latency.write;
  }
  
  if (metrics.iops) {
    document.getElementById('iops').textContent = metrics.iops.read + metrics.iops.write;
  }
}

/**
 * Show datasets for a specific pool
 */
function showDatasets(poolName) {
  const datasets = window.datasetsCache && window.datasetsCache[poolName];
  
  if (!datasets) {
    showError(`No datasets found for pool: ${poolName}`);
    return;
  }
  
  let modalContent = `
    <div class="modal-header">
      <h2>Datasets for ${poolName}</h2>
      <button class="close-modal">&times;</button>
    </div>
    <div class="modal-body">
  `;
  
  if (dataSourceMode === 'mock') {
    modalContent += '<div class="mock-data-warning">MOCK DATA</div>';
  }
  
  if (datasets.length === 0) {
    modalContent += '<p>No datasets found for this pool.</p>';
  } else {
    modalContent += `
      <div class="datasets-table">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Used</th>
              <th>Available</th>
              <th>Mount Point</th>
            </tr>
          </thead>
          <tbody>
    `;
    
    datasets.forEach(dataset => {
      modalContent += `
        <tr>
          <td>${dataset.name}</td>
          <td>${formatBytes(dataset.used)}</td>
          <td>${formatBytes(dataset.available)}</td>
          <td>${dataset.mountpoint}</td>
        </tr>
      `;
    });
    
    modalContent += `
          </tbody>
        </table>
      </div>
    `;
  }
  
  modalContent += `
    </div>
    <div class="modal-footer">
      <button class="close-modal">Close</button>
    </div>
  `;
  
  showModal(modalContent);
}

/**
 * Show snapshots for a specific pool
 */
function showSnapshots(poolName) {
  // This function would load and display snapshots
  // For now, we'll just show a dummy message
  let modalContent = `
    <div class="modal-header">
      <h2>Snapshots for ${poolName}</h2>
      <button class="close-modal">&times;</button>
    </div>
    <div class="modal-body">
  `;
  
  if (dataSourceMode === 'mock') {
    modalContent += '<div class="mock-data-warning">MOCK DATA</div>';
  }
  
  modalContent += `
      <p>Snapshot functionality will be implemented in a future update.</p>
    </div>
    <div class="modal-footer">
      <button class="close-modal">Close</button>
    </div>
  `;
  
  showModal(modalContent);
}

/**
 * Show a modal with the provided content
 */
function showModal(content) {
  // Create modal elements if they don't exist
  let modalContainer = document.getElementById('modal-container');
  
  if (!modalContainer) {
    modalContainer = document.createElement('div');
    modalContainer.id = 'modal-container';
    modalContainer.className = 'modal-container';
    document.body.appendChild(modalContainer);
  }
  
  // Create modal content
  modalContainer.innerHTML = `
    <div class="modal-backdrop"></div>
    <div class="modal-content">
      ${content}
    </div>
  `;
  
  // Show the modal
  modalContainer.style.display = 'flex';
  
  // Add event listeners to close buttons
  const closeButtons = modalContainer.querySelectorAll('.close-modal');
  closeButtons.forEach(button => {
    button.addEventListener('click', closeModal);
  });
  
  // Close on backdrop click
  const backdrop = modalContainer.querySelector('.modal-backdrop');
  backdrop.addEventListener('click', closeModal);
}

/**
 * Close the currently open modal
 */
function closeModal() {
  const modalContainer = document.getElementById('modal-container');
  if (modalContainer) {
    modalContainer.style.display = 'none';
  }
}

/**
 * Show an error message
 */
function showError(message) {
  showNotification(message, 'error');
}

/**
 * Show a notification message
 */
function showNotification(message, type = 'info') {
  const notification = document.getElementById('notification');
  if (!notification) return;
  
  notification.textContent = message;
  notification.className = `notification ${type}`;
  notification.style.display = 'block';
  
  setTimeout(() => {
    notification.style.display = 'none';
  }, 5000);
}

/**
 * Format bytes to a human-readable format
 */
function formatBytes(bytes, decimals = 2) {
  if (bytes === 0) return '0 Bytes';
  
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
} 