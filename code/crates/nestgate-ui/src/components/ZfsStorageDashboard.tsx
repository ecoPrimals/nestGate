import React, { useState, useEffect } from 'react';
import { getApiBaseUrl } from '../config';

// Types matching the NAS API
interface TierUsage {
  available: number;
  used: number;
  total: number;
  compression_ratio: number;
}

interface TierInfo {
  id: string;
  name: string;
  path: string;
  usage: TierUsage;
  properties: Record<string, string>;
  monitoring: {
    enabled: boolean;
    active_events: number;
    recent_events: number;
  };
}

interface DatasetInfo {
  id: string;
  name: string;
  tier: string;
  mountpoint: string;
  available: number;
  used: number;
  compression: string;
  recordsize: string;
  readonly: boolean;
}

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

const ZfsStorageDashboard: React.FC = () => {
  const [tiers, setTiers] = useState<TierInfo[]>([]);
  const [datasets, setDatasets] = useState<DatasetInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdate, setLastUpdate] = useState<Date>(new Date());
  const [apiBase, setApiBase] = useState<string>('');

  // Initialize API base URL from Port Manager
  useEffect(() => {
    const initializeApi = async () => {
      try {
        const baseUrl = await getApiBaseUrl();
        setApiBase(baseUrl);
      } catch (err) {
        setError('Failed to get API URL from Port Manager. Please ensure Port Manager is running.');
        console.error('Failed to initialize API base URL:', err);
      }
    };
    
    initializeApi();
  }, []);

  // Format bytes to human readable
  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  // Calculate usage percentage
  const getUsagePercentage = (used: number, total: number): number => {
    return total > 0 ? Math.round((used / total) * 100) : 0;
  };

  // Get tier style
  const getTierStyle = (tierId: string) => {
    switch (tierId) {
      case 'hot':
        return { icon: '🔥', color: '#d32f2f', bgColor: '#ffebee' };
      case 'warm':
        return { icon: '🌡️', color: '#ed6c02', bgColor: '#fff3e0' };
      case 'cold':
        return { icon: '❄️', color: '#0288d1', bgColor: '#e3f2fd' };
      default:
        return { icon: '💾', color: '#666', bgColor: '#f5f5f5' };
    }
  };

  // Fetch real ZFS data from API
  const fetchZfsData = async () => {
    if (!apiBase) {
      setError('API URL not available from Port Manager');
      return;
    }

    try {
      setLoading(true);
      setError(null);

      // Fetch tiers
      const tiersResponse = await fetch(`${apiBase}/storage/tiers`);
      const tiersData: ApiResponse<TierInfo[]> = await tiersResponse.json();
      
      if (!tiersData.success) {
        throw new Error(tiersData.error || 'Failed to fetch tier data');
      }

      // Fetch datasets
      const datasetsResponse = await fetch(`${apiBase}/storage/datasets`);
      const datasetsData: ApiResponse<DatasetInfo[]> = await datasetsResponse.json();
      
      if (!datasetsData.success) {
        throw new Error(datasetsData.error || 'Failed to fetch dataset data');
      }

      setTiers(tiersData.data || []);
      setDatasets(datasetsData.data || []);
      setLastUpdate(new Date());
      
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error occurred';
      setError(errorMessage);
      console.error('Failed to fetch ZFS data:', err);
    } finally {
      setLoading(false);
    }
  };

  // Auto-refresh data every 30 seconds, but only if apiBase is available
  useEffect(() => {
    if (!apiBase) return;
    
    fetchZfsData();
    
    const interval = setInterval(fetchZfsData, 30000);
    return () => clearInterval(interval);
  }, [apiBase]);

  if (loading && tiers.length === 0) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', padding: '64px' }}>
        <div style={{ 
          width: '32px', 
          height: '32px', 
          border: '3px solid #f0f0f0', 
          borderTop: '3px solid #1976d2',
          borderRadius: '50%',
          animation: 'spin 1s linear infinite',
          marginRight: '16px'
        }} />
        <span>Loading real ZFS data...</span>
        <style>
          {`
            @keyframes spin {
              0% { transform: rotate(0deg); }
              100% { transform: rotate(360deg); }
            }
          `}
        </style>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ 
        margin: '32px', 
        padding: '16px', 
        backgroundColor: '#ffebee', 
        border: '1px solid #f44336',
        borderRadius: '4px',
        color: '#d32f2f'
      }}>
        <h3>⚠️ ZFS Integration Error</h3>
        <p>{error}</p>
        <small style={{ color: '#666' }}>
          Ensure ZFS is installed and nestpool is configured, or check that the NAS API server is running.
        </small>
      </div>
    );
  }

  return (
    <div style={{ padding: '24px', fontFamily: 'Arial, sans-serif' }}>
      {/* Header */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '24px' }}>
        <div>
          <h1 style={{ fontSize: '2.5rem', margin: '0 0 8px 0' }}>ZFS Storage Dashboard</h1>
          <p style={{ color: '#666', margin: 0 }}>Real-time access to live ZFS pool data</p>
        </div>
        <div style={{ textAlign: 'right' }}>
          <div style={{ marginBottom: '8px' }}>
            <span style={{ 
              display: 'inline-flex', 
              alignItems: 'center',
              padding: '4px 8px', 
              border: '1px solid #4caf50',
              borderRadius: '16px',
              fontSize: '0.75rem',
              color: '#4caf50',
              marginRight: '8px'
            }}>
              ⚡ Live Data
            </span>
            <button 
              onClick={fetchZfsData}
              disabled={loading}
              style={{
                padding: '4px 8px',
                border: '1px solid #1976d2',
                borderRadius: '4px',
                backgroundColor: loading ? '#f5f5f5' : '#1976d2',
                color: loading ? '#999' : 'white',
                cursor: loading ? 'not-allowed' : 'pointer'
              }}
            >
              {loading ? '⟳' : '🔄'} Refresh
            </button>
          </div>
          <p style={{ fontSize: '0.875rem', color: '#666', margin: 0 }}>
            Last updated: {lastUpdate.toLocaleTimeString()}
          </p>
        </div>
      </div>

      {/* Tier Overview */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '24px', marginBottom: '24px' }}>
        {tiers.map((tier) => {
          const style = getTierStyle(tier.id);
          const usagePercent = getUsagePercentage(tier.usage.used, tier.usage.total);
          
          return (
            <div key={tier.id} style={{ border: `2px solid ${style.color}`, borderRadius: '8px' }}>
              <div style={{ backgroundColor: style.bgColor, padding: '16px' }}>
                <h3 style={{ margin: 0, color: style.color, display: 'flex', alignItems: 'center' }}>
                  <span style={{ fontSize: '1.5em', marginRight: '8px' }}>{style.icon}</span>
                  {tier.name.toUpperCase()} Tier
                </h3>
              </div>
              <div style={{ padding: '16px' }}>
                {/* Usage Progress */}
                <div style={{ marginBottom: '16px' }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                    <span>Usage</span>
                    <span>{usagePercent}%</span>
                  </div>
                  <div style={{ 
                    width: '100%', 
                    backgroundColor: '#f0f0f0', 
                    borderRadius: '4px', 
                    height: '8px' 
                  }}>
                    <div style={{ 
                      width: `${usagePercent}%`, 
                      backgroundColor: style.color, 
                      height: '100%', 
                      borderRadius: '4px' 
                    }} />
                  </div>
                  <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '4px', fontSize: '0.75rem', color: '#666' }}>
                    <span>{formatBytes(tier.usage.used)} used</span>
                    <span>{formatBytes(tier.usage.available)} free</span>
                  </div>
                </div>

                {/* Properties */}
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px', marginBottom: '16px' }}>
                  <div>
                    <div style={{ fontWeight: 'bold', fontSize: '0.75rem' }}>Compression:</div>
                    <div>{tier.properties.compression || 'N/A'}</div>
                  </div>
                  <div>
                    <div style={{ fontWeight: 'bold', fontSize: '0.75rem' }}>Ratio:</div>
                    <div>{tier.usage.compression_ratio.toFixed(1)}x</div>
                  </div>
                  <div>
                    <div style={{ fontWeight: 'bold', fontSize: '0.75rem' }}>Record Size:</div>
                    <div>{tier.properties.recordsize || 'N/A'}</div>
                  </div>
                  <div>
                    <div style={{ fontWeight: 'bold', fontSize: '0.75rem' }}>Events:</div>
                    <div>{tier.monitoring.active_events} active</div>
                  </div>
                </div>

                {/* Path */}
                <div style={{ paddingTop: '16px', borderTop: '1px solid #ddd' }}>
                  <div style={{ fontWeight: 'bold', fontSize: '0.75rem', color: '#666' }}>Mount Path:</div>
                  <code style={{ 
                    display: 'block', 
                    backgroundColor: '#f5f5f5', 
                    padding: '4px', 
                    borderRadius: '4px', 
                    marginTop: '4px',
                    fontSize: '0.75rem'
                  }}>
                    {tier.path}
                  </code>
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Datasets Table */}
      <div style={{ border: '1px solid #ddd', borderRadius: '8px', marginBottom: '24px' }}>
        <div style={{ padding: '16px', borderBottom: '1px solid #ddd' }}>
          <h3 style={{ margin: 0, display: 'flex', alignItems: 'center' }}>
            🗄️ ZFS Datasets
          </h3>
        </div>
        <div style={{ padding: '16px' }}>
          {datasets.length === 0 ? (
            <p style={{ textAlign: 'center', color: '#666', padding: '32px' }}>No datasets found</p>
          ) : (
            <div style={{ overflowX: 'auto' }}>
              <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                <thead>
                  <tr style={{ borderBottom: '1px solid #ddd' }}>
                    <th style={{ textAlign: 'left', padding: '8px' }}>Dataset</th>
                    <th style={{ textAlign: 'left', padding: '8px' }}>Tier</th>
                    <th style={{ textAlign: 'left', padding: '8px' }}>Usage</th>
                    <th style={{ textAlign: 'left', padding: '8px' }}>Compression</th>
                    <th style={{ textAlign: 'left', padding: '8px' }}>Mount Point</th>
                    <th style={{ textAlign: 'left', padding: '8px' }}>Status</th>
                  </tr>
                </thead>
                <tbody>
                  {datasets.map((dataset) => {
                    const tierStyle = getTierStyle(dataset.tier);
                    const usagePercent = getUsagePercentage(
                      dataset.used, 
                      dataset.used + dataset.available
                    );
                    
                    return (
                      <tr key={dataset.id} style={{ borderBottom: '1px solid #f0f0f0' }}>
                        <td style={{ padding: '8px', fontWeight: 'bold' }}>{dataset.name}</td>
                        <td style={{ padding: '8px' }}>
                          <span style={{ 
                            padding: '2px 8px', 
                            borderRadius: '12px', 
                            backgroundColor: tierStyle.bgColor,
                            color: tierStyle.color,
                            fontSize: '0.75rem'
                          }}>
                            {tierStyle.icon} {dataset.tier}
                          </span>
                        </td>
                        <td style={{ padding: '8px' }}>
                          <div style={{ width: '80px' }}>
                            <div style={{ 
                              width: '100%', 
                              backgroundColor: '#f0f0f0', 
                              borderRadius: '2px', 
                              height: '4px',
                              marginBottom: '4px'
                            }}>
                              <div style={{ 
                                width: `${usagePercent}%`, 
                                backgroundColor: tierStyle.color, 
                                height: '100%', 
                                borderRadius: '2px' 
                              }} />
                            </div>
                            <span style={{ fontSize: '0.75rem', color: '#666' }}>
                              {formatBytes(dataset.used)}
                            </span>
                          </div>
                        </td>
                        <td style={{ padding: '8px', fontFamily: 'monospace', fontSize: '0.875rem' }}>
                          {dataset.compression}
                        </td>
                        <td style={{ padding: '8px', fontFamily: 'monospace', fontSize: '0.875rem' }}>
                          {dataset.mountpoint}
                        </td>
                        <td style={{ padding: '8px' }}>
                          <span style={{ 
                            padding: '2px 8px', 
                            borderRadius: '12px', 
                            backgroundColor: dataset.readonly ? '#f5f5f5' : '#e3f2fd',
                            color: dataset.readonly ? '#666' : '#1976d2',
                            fontSize: '0.75rem'
                          }}>
                            {dataset.readonly ? "Read-Only" : "Read-Write"}
                          </span>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>

      {/* Real-time Verification */}
      <div style={{ border: '1px solid #ddd', borderRadius: '8px' }}>
        <div style={{ padding: '16px', borderBottom: '1px solid #ddd' }}>
          <h3 style={{ margin: 0, display: 'flex', alignItems: 'center' }}>
            💻 ZFS Integration Status
          </h3>
        </div>
        <div style={{ padding: '16px' }}>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '32px' }}>
            <div>
              <h4 style={{ marginTop: 0, marginBottom: '8px' }}>API Endpoints</h4>
              <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                <li style={{ marginBottom: '4px' }}>
                  <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '2px' }}>
                    GET /api/v1/storage/tiers
                  </code>
                </li>
                <li style={{ marginBottom: '4px' }}>
                  <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '2px' }}>
                    GET /api/v1/storage/datasets
                  </code>
                </li>
                <li>
                  <code style={{ backgroundColor: '#f5f5f5', padding: '2px 4px', borderRadius: '2px' }}>
                    WS /api/v1/storage/events
                  </code>
                </li>
              </ul>
            </div>
            <div>
              <h4 style={{ marginTop: 0, marginBottom: '8px' }}>Data Source</h4>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                <span style={{ 
                  padding: '2px 8px', 
                  borderRadius: '12px', 
                  border: '1px solid #4caf50',
                  color: '#4caf50',
                  fontSize: '0.75rem'
                }}>
                  ✅ Real ZFS Commands
                </span>
                <span style={{ 
                  padding: '2px 8px', 
                  borderRadius: '12px', 
                  border: '1px solid #2196f3',
                  color: '#2196f3',
                  fontSize: '0.75rem'
                }}>
                  ✅ Live Pool Data
                </span>
                <span style={{ 
                  padding: '2px 8px', 
                  borderRadius: '12px', 
                  border: '1px solid #f44336',
                  color: '#f44336',
                  fontSize: '0.75rem'
                }}>
                  ❌ No Mock Fallbacks
                </span>
              </div>
            </div>
          </div>
          
          <div style={{ 
            marginTop: '16px', 
            padding: '12px', 
            backgroundColor: '#e3f2fd', 
            border: '1px solid #2196f3',
            borderRadius: '4px'
          }}>
            <strong style={{ color: '#1976d2' }}>Browser Console Test:</strong>
            <p style={{ margin: '8px 0 0 0', fontSize: '0.875rem', color: '#1976d2' }}>
              Open Developer Tools and run (replace API_URL with actual service URL):
            </p>
            <code style={{ 
              display: 'block', 
              backgroundColor: 'rgba(0,0,0,0.1)', 
              padding: '8px', 
              borderRadius: '4px', 
              marginTop: '8px',
              fontFamily: 'monospace',
              fontSize: '0.75rem'
            }}>
              fetch('API_URL/storage/tiers').then(r =&gt; r.json()).then(console.log)
            </code>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ZfsStorageDashboard; 