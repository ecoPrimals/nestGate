import React, { useState, useEffect } from 'react';
import { 
  Card, 
  CardContent,
  CardHeader,
  Select, 
  Tabs, 
  Tab,
  Box, 
  Typography, 
  CircularProgress, 
  Button, 
  FormControl,
  InputLabel,
  MenuItem,
  SelectChangeEvent
} from '@mui/material';
import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { 
  Refresh as ReloadIcon, 
  ShowChart as LineChartIcon, 
  BarChart as BarChartIcon, 
  PieChart as PieChartIcon 
} from '@mui/icons-material';
import { 
  WebSocketService, 
  WebSocketMessage,
  ConnectionStatus,
  Pool,
  PoolPerformance as PoolPerformanceData,
  isConnectionStatus, 
  isPools,
  isPoolPerformance
} from '../../services/websocket.service';
import EmptyState from '../common/EmptyState';
import TabPanel from '../common/TabPanel';

interface PoolPerformanceProps {
  poolName?: string;
  defaultTimeRange?: 'hour' | 'day' | 'week' | 'month';
}

interface PerformanceMetric {
  timestamp: number;
  read_iops: number;
  write_iops: number;
  read_throughput: number;
  write_throughput: number;
  latency: number;
  cpu_usage: number;
  memory_usage: number;
}

interface PoolData {
  name: string;
  health: string;
  status: string;
  size: number;
  used: number;
  available: number;
}

// Custom Statistic component to replace Ant Design's Statistic
interface StatisticProps {
  title: string;
  value: string | number;
  suffix?: string;
  precision?: number;
  size?: 'small' | 'medium' | 'large';
}

const Statistic: React.FC<StatisticProps> = ({ 
  title, 
  value, 
  suffix, 
  precision,
  size = 'medium'
}) => {
  const fontSize = size === 'small' ? '1rem' : size === 'large' ? '1.5rem' : '1.2rem';
  const displayValue = typeof value === 'number' && precision !== undefined 
    ? value.toFixed(precision) 
    : value;
  
  return (
    <Box sx={{ textAlign: 'center', padding: 1 }}>
      <Typography variant="body2" color="text.secondary" gutterBottom>
        {title}
      </Typography>
      <Typography variant="h6" sx={{ fontSize }}>
        {displayValue}{suffix}
      </Typography>
    </Box>
  );
};

export const PoolPerformance: React.FC<PoolPerformanceProps> = ({ 
  poolName,
  defaultTimeRange = 'hour'
}) => {
  const [selectedPool, setSelectedPool] = useState<string | undefined>(poolName);
  const [timeRange, setTimeRange] = useState<string>(defaultTimeRange);
  const [customRange, setCustomRange] = useState<[Date, Date] | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [refreshing, setRefreshing] = useState<boolean>(false);
  const [realTimeEnabled, setRealTimeEnabled] = useState<boolean>(true);
  const [availablePools, setAvailablePools] = useState<PoolData[]>([]);
  const [performanceData, setPerformanceData] = useState<PerformanceMetric[]>([]);
  const [latestMetrics, setLatestMetrics] = useState<PerformanceMetric | null>(null);
  const [socket, setSocket] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState(0);

  // Initialize WebSocket on component mount
  useEffect(() => {
    const wsService = WebSocketService.getInstance();
    wsService.connect();
    
    // Check connection status
    setSocket(wsService.isConnected());
    
    const connectionUnsubscribe = wsService.subscribe<ConnectionStatus>('connection', (msg: WebSocketMessage<ConnectionStatus>) => {
      setSocket(msg.data.status === 'connected');
      
      // If connection is established, request initial data
      if (msg.data.status === 'connected') {
        fetchInitialData();
      }
    });
    
    // Subscribe to pool list updates
    const poolsUnsubscribe = wsService.subscribe('pools', (msg: WebSocketMessage) => {
      if (isPools(msg.data)) {
        setAvailablePools(msg.data.map(pool => ({
          name: pool.name,
          health: pool.health,
          status: pool.status,
          size: pool.capacity?.total || 0,
          used: pool.capacity?.used || 0,
          available: pool.capacity?.available || 0
        })));
        
        // Select first pool if none selected
        if (!selectedPool && msg.data.length > 0) {
          setSelectedPool(msg.data[0].name);
        }
      }
    });
    
    // Subscribe to real-time performance metrics
    const performanceUnsubscribe = wsService.subscribe('pool_performance', (msg: WebSocketMessage) => {
      if (isPoolPerformance(msg.data) && msg.data.pool_name === selectedPool && realTimeEnabled) {
        const newMetric: PerformanceMetric = {
          timestamp: msg.data.timestamp,
          read_iops: msg.data.read_iops,
          write_iops: msg.data.write_iops,
          read_throughput: msg.data.read_throughput,
          write_throughput: msg.data.write_throughput,
          latency: typeof msg.data.latency === 'object' ? msg.data.latency.read : msg.data.latency,
          cpu_usage: msg.data.cpu_usage,
          memory_usage: msg.data.memory_usage
        };
        
        // Update latest metrics
        setLatestMetrics(newMetric);
        
        // Add to chart data (keep last 60 points for real-time view)
        setPerformanceData(prev => {
          const newData = [...prev, newMetric];
          if (newData.length > 60) {
            return newData.slice(-60);
          }
          return newData;
        });
      }
    });
    
    // Initial data fetch
    fetchInitialData();
    
    // Cleanup function
    return () => {
      connectionUnsubscribe();
      poolsUnsubscribe();
      performanceUnsubscribe();
    };
  }, [selectedPool, realTimeEnabled]);

  const fetchInitialData = async () => {
    setLoading(true);
    try {
      const wsService = WebSocketService.getInstance();
      
      // Request pool list
      wsService.send('get_pools');
      
      // Request historical performance data for the selected pool and time range
      if (selectedPool) {
        wsService.send('get_pool_performance', {
          pool_name: selectedPool,
          time_range: timeRange,
          custom_range: customRange ? [
            customRange[0].getTime(),
            customRange[1].getTime()
          ] : null
        });
      }
      
    } catch (error) {
      console.error('Error fetching initial data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePoolChange = (event: SelectChangeEvent) => {
    const value = event.target.value;
    setSelectedPool(value);
    // Reset performance data when changing pools
    setPerformanceData([]);
    setLatestMetrics(null);
    fetchHistoricalData(value, timeRange, customRange);
  };

  const handleTimeRangeChange = (event: SelectChangeEvent) => {
    const value = event.target.value;
    setTimeRange(value);
    setCustomRange(null);
    fetchHistoricalData(selectedPool, value, null);
  };

  const handleCustomRangeChange = (dates: [Date, Date] | null) => {
    if (dates) {
      setCustomRange(dates);
      setTimeRange('custom');
      fetchHistoricalData(selectedPool, 'custom', dates);
    }
  };

  const handleRefresh = () => {
    setRefreshing(true);
    fetchHistoricalData(selectedPool, timeRange, customRange).finally(() => {
      setRefreshing(false);
    });
  };

  const fetchHistoricalData = async (
    pool?: string, 
    range?: string,
    dates?: [Date, Date] | null
  ) => {
    if (!pool) return Promise.resolve();
    
    try {
      const wsService = WebSocketService.getInstance();
      wsService.send('get_pool_performance', {
        pool_name: pool,
        time_range: range,
        custom_range: dates ? [dates[0].getTime(), dates[1].getTime()] : null
      });
    } catch (error) {
      console.error('Error fetching historical data:', error);
    }
  };

  const toggleRealTimeUpdates = () => {
    setRealTimeEnabled(!realTimeEnabled);
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleTimeString();
  };

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatThroughput = (value: number) => {
    return formatBytes(value) + '/s';
  };

  const calculateAverage = (data: PerformanceMetric[], key: keyof PerformanceMetric) => {
    if (data.length === 0) return 0;
    const sum = data.reduce((acc, item) => acc + (item[key] as number), 0);
    return sum / data.length;
  };

  const calculateMax = (data: PerformanceMetric[], key: keyof PerformanceMetric) => {
    if (data.length === 0) return 0;
    return Math.max(...data.map(item => item[key] as number));
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <Card>
      <CardHeader title="ZFS Pool Performance" />
      <CardContent>
        {loading && (
          <Box sx={{ display: 'flex', justifyContent: 'center', padding: 3 }}>
            <CircularProgress />
          </Box>
        )}
        
        <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, marginBottom: 2 }}>
          <Box sx={{ flex: '1 1 300px', minWidth: 300 }}>
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
              <FormControl fullWidth>
                <InputLabel>Pool</InputLabel>
            <Select
                  value={selectedPool || ''}
                  label="Pool"
              onChange={handlePoolChange}
                  disabled={loading}
            >
              {availablePools.map(pool => (
                    <MenuItem key={pool.name} value={pool.name}>
                  {pool.name} ({pool.health})
                    </MenuItem>
              ))}
            </Select>
              </FormControl>
            
              <FormControl fullWidth>
                <InputLabel>Time Range</InputLabel>
            <Select
              value={timeRange}
                  label="Time Range"
              onChange={handleTimeRangeChange}
            >
                  <MenuItem value="hour">Last Hour</MenuItem>
                  <MenuItem value="day">Last 24 Hours</MenuItem>
                  <MenuItem value="week">Last Week</MenuItem>
                  <MenuItem value="month">Last Month</MenuItem>
                  <MenuItem value="custom">Custom Range</MenuItem>
            </Select>
              </FormControl>
            
            {timeRange === 'custom' && (
                <Box sx={{ display: 'flex', gap: 1 }}>
                  <DatePicker
                    label="Start Date"
                    value={customRange?.[0] || null}
                    onChange={(date) => {
                      if (date && customRange?.[1]) {
                        setCustomRange([date, customRange[1]]);
                      }
                    }}
                    slotProps={{ textField: { size: 'small' } }}
                  />
                  <DatePicker
                    label="End Date"
                    value={customRange?.[1] || null}
                    onChange={(date) => {
                      if (date && customRange?.[0]) {
                        setCustomRange([customRange[0], date]);
                      }
                    }}
                    slotProps={{ textField: { size: 'small' } }}
                  />
                </Box>
            )}
            
            <Button
                variant="contained"
                startIcon={<ReloadIcon />}
              onClick={handleRefresh}
                disabled={refreshing}
                fullWidth
            >
                {refreshing ? 'Refreshing...' : 'Refresh Data'}
            </Button>
            
            <Button
                variant={realTimeEnabled ? 'contained' : 'outlined'}
              onClick={toggleRealTimeUpdates}
                fullWidth
            >
              {realTimeEnabled ? 'Disable Real-time Updates' : 'Enable Real-time Updates'}
            </Button>
            </Box>
          </Box>
          
          <Box sx={{ flex: '2 1 400px', minWidth: 400 }}>
            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
              <Box sx={{ flex: '1 1 150px', minWidth: 150 }}>
                <Card variant="outlined">
                  <CardContent>
                <Statistic
                  title="Read IOPS"
                  value={latestMetrics?.read_iops || 0}
                      suffix=" IOPS"
                  precision={0}
                />
                  </CardContent>
              </Card>
              </Box>
              <Box sx={{ flex: '1 1 150px', minWidth: 150 }}>
                <Card variant="outlined">
                  <CardContent>
                <Statistic
                  title="Write IOPS"
                  value={latestMetrics?.write_iops || 0}
                      suffix=" IOPS"
                  precision={0}
                />
                  </CardContent>
              </Card>
              </Box>
              <Box sx={{ flex: '1 1 150px', minWidth: 150 }}>
                <Card variant="outlined">
                  <CardContent>
                <Statistic
                  title="Read Throughput"
                  value={formatThroughput(latestMetrics?.read_throughput || 0)}
                  precision={0}
                      size="small"
                />
                  </CardContent>
              </Card>
              </Box>
              <Box sx={{ flex: '1 1 150px', minWidth: 150 }}>
                <Card variant="outlined">
                  <CardContent>
                <Statistic
                  title="Write Throughput"
                  value={formatThroughput(latestMetrics?.write_throughput || 0)}
                  precision={0}
                      size="small"
                />
                  </CardContent>
              </Card>
              </Box>
            </Box>
          </Box>
        </Box>
        
        <Box sx={{ borderBottom: 1, borderColor: 'divider', marginTop: 2 }}>
          <Tabs value={activeTab} onChange={handleTabChange}>
            <Tab 
              icon={<LineChartIcon />}
              label="IOPS" 
              iconPosition="start"
            />
            <Tab 
              icon={<BarChartIcon />}
              label="Throughput" 
              iconPosition="start"
            />
            <Tab 
              icon={<PieChartIcon />}
              label="Latency" 
              iconPosition="start"
            />
          </Tabs>
        </Box>
        
        <TabPanel value={activeTab} index={0}>
          <Card variant="outlined">
            <CardContent>
            {performanceData.length > 0 ? (
                <Box sx={{ width: '100%', height: 300 }}>
                <ResponsiveContainer>
                  <LineChart
                    data={performanceData}
                    margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                  >
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis 
                      dataKey="timestamp" 
                      tickFormatter={formatTimestamp} 
                      label={{ value: 'Time', position: 'insideBottomRight', offset: 0 }}
                    />
                    <YAxis 
                      label={{ value: 'IOPS', angle: -90, position: 'insideLeft' }}
                    />
                    <Tooltip 
                      formatter={(value) => [`${value} IOPS`, '']}
                      labelFormatter={formatTimestamp}
                    />
                    <Legend />
                    <Line 
                      type="monotone" 
                      dataKey="read_iops" 
                      name="Read IOPS" 
                      stroke="#8884d8" 
                      activeDot={{ r: 8 }} 
                    />
                    <Line 
                      type="monotone" 
                      dataKey="write_iops" 
                      name="Write IOPS" 
                      stroke="#82ca9d" 
                    />
                  </LineChart>
                </ResponsiveContainer>
                </Box>
            ) : (
                <EmptyState 
                  title="No performance data available"
                  icon={<LineChartIcon />}
                />
            )}
            
              <Box sx={{ display: 'flex', gap: 2, justifyContent: 'space-around', marginTop: 2 }}>
                <Statistic 
                  title="Avg Read IOPS" 
                  value={calculateAverage(performanceData, 'read_iops')}
                  precision={0}
                />
                <Statistic 
                  title="Avg Write IOPS" 
                  value={calculateAverage(performanceData, 'write_iops')}
                  precision={0}
                />
                <Statistic 
                  title="Max Total IOPS" 
                  value={calculateMax(performanceData.map(d => ({
                    ...d,
                    total_iops: d.read_iops + d.write_iops
                  })), 'total_iops' as any)}
                  precision={0}
                />
              </Box>
            </CardContent>
          </Card>
        </TabPanel>
        
        <TabPanel value={activeTab} index={1}>
          <Card variant="outlined">
            <CardContent>
            {performanceData.length > 0 ? (
                <Box sx={{ width: '100%', height: 300 }}>
                <ResponsiveContainer>
                  <LineChart
                    data={performanceData}
                    margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                  >
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis 
                      dataKey="timestamp" 
                      tickFormatter={formatTimestamp} 
                      label={{ value: 'Time', position: 'insideBottomRight', offset: 0 }}
                    />
                    <YAxis 
                      label={{ value: 'Throughput (B/s)', angle: -90, position: 'insideLeft' }}
                    />
                    <Tooltip 
                      formatter={(value) => [formatThroughput(value as number), '']}
                      labelFormatter={formatTimestamp}
                    />
                    <Legend />
                    <Line 
                      type="monotone" 
                      dataKey="read_throughput" 
                      name="Read Throughput" 
                      stroke="#8884d8" 
                      activeDot={{ r: 8 }} 
                    />
                    <Line 
                      type="monotone" 
                      dataKey="write_throughput" 
                      name="Write Throughput" 
                      stroke="#82ca9d" 
                    />
                  </LineChart>
                </ResponsiveContainer>
                </Box>
            ) : (
                <EmptyState 
                  title="No throughput data available"
                  icon={<BarChartIcon />}
                />
            )}
            
              <Box sx={{ display: 'flex', gap: 2, justifyContent: 'space-around', marginTop: 2 }}>
                <Statistic 
                  title="Avg Read Throughput" 
                  value={formatThroughput(calculateAverage(performanceData, 'read_throughput'))}
                  size="small"
                />
                <Statistic 
                  title="Avg Write Throughput" 
                  value={formatThroughput(calculateAverage(performanceData, 'write_throughput'))}
                  size="small"
                />
                <Statistic 
                  title="Max Read Throughput" 
                  value={formatThroughput(calculateMax(performanceData, 'read_throughput'))}
                  size="small"
                />
              </Box>
            </CardContent>
          </Card>
        </TabPanel>
        
        <TabPanel value={activeTab} index={2}>
          <Card variant="outlined">
            <CardContent>
            {performanceData.length > 0 ? (
                <Box sx={{ width: '100%', height: 300 }}>
                <ResponsiveContainer>
                  <LineChart
                    data={performanceData}
                    margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                  >
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis 
                      dataKey="timestamp" 
                      tickFormatter={formatTimestamp} 
                      label={{ value: 'Time', position: 'insideBottomRight', offset: 0 }}
                    />
                    <YAxis 
                      label={{ value: 'Latency (ms)', angle: -90, position: 'insideLeft' }}
                    />
                    <Tooltip 
                      formatter={(value) => [`${value} ms`, 'Latency']}
                      labelFormatter={formatTimestamp}
                    />
                    <Legend />
                    <Line 
                      type="monotone" 
                      dataKey="latency" 
                      name="Latency" 
                      stroke="#ff7300" 
                      activeDot={{ r: 8 }} 
                    />
                  </LineChart>
                </ResponsiveContainer>
                </Box>
            ) : (
                <EmptyState 
                  title="No latency data available"
                  icon={<PieChartIcon />}
                />
            )}
            
              <Box sx={{ display: 'flex', gap: 2, justifyContent: 'space-around', marginTop: 2 }}>
                <Statistic 
                  title="Avg Latency" 
                  value={calculateAverage(performanceData, 'latency')}
                  suffix=" ms"
                  precision={2}
                />
                <Statistic 
                  title="Max Latency" 
                  value={calculateMax(performanceData, 'latency')}
                  suffix=" ms"
                  precision={2}
                />
                <Statistic 
                  title="Current Latency" 
                  value={latestMetrics?.latency || 0}
                  suffix=" ms"
                  precision={2}
                />
              </Box>
            </CardContent>
          </Card>
        </TabPanel>
      </CardContent>
    </Card>
  );
}; 